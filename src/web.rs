//! Web server for monitoring API and WebSocket support

use crate::config::PortConfig;
use crate::error::{Error, Result};
use crate::manager::ProcessManager;
use crate::monitoring::{Monitor, SystemMetrics};
use crate::process::{ProcessState, ProcessStatus};
use axum::response::IntoResponse;
use axum::{
    extract::{
        ws::{Message, WebSocket},
        Path, Query, State, WebSocketUpgrade,
    },
    http::{header, HeaderValue, StatusCode},
    middleware::{self, Next},
    response::{Json, Response},
    routing::{get, post},
    Router,
};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::sync::{broadcast, RwLock};
use tokio::time::interval;
use tower_http::cors::CorsLayer;
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::trace::TraceLayer;
use tracing::{debug, error, info, warn};

/// Query parameters for process listing
#[derive(Debug, Deserialize)]
pub struct ListQuery {
    /// Filter by namespace
    pub namespace: Option<String>,
    /// Include monitoring data
    pub monitoring: Option<bool>,
}

/// Query parameters for logs
#[derive(Debug, Deserialize)]
pub struct LogsQuery {
    /// Number of lines to return
    pub lines: Option<usize>,
    /// Follow logs (streaming)
    pub follow: Option<bool>,
}

/// Request body for process actions
#[derive(Debug, Deserialize)]
pub struct ProcessActionRequest {
    /// Optional port override for restart/reload
    pub port: Option<u16>,
    /// Optional port range for restart/reload
    pub port_range: Option<(u16, u16)>,
}

/// WebSocket message types
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum WebSocketMessage {
    /// Process status update
    ProcessUpdate {
        /// Updated process status information
        process: ProcessStatus,
    },
    /// System metrics update
    SystemUpdate {
        /// Current system metrics
        metrics: SystemMetrics,
    },
    /// Process list update
    ProcessList {
        /// List of all current processes
        processes: Vec<ProcessStatus>,
    },
    /// Error message
    Error {
        /// Error message to send to client
        message: String,
    },
}

/// Shared application state
#[derive(Clone)]
pub struct AppState {
    /// Process manager
    pub manager: Arc<RwLock<ProcessManager>>,
    /// System monitor
    pub monitor: Arc<RwLock<Monitor>>,
    /// WebSocket broadcast channel
    pub broadcast_tx: broadcast::Sender<WebSocketMessage>,
    /// API key for authentication (optional)
    pub api_key: Option<String>,
}

/// Web server for monitoring API
pub struct WebServer {
    /// Shared application state
    state: AppState,
}

impl WebServer {
    /// Create a new web server with process manager
    pub async fn new(manager: Arc<RwLock<ProcessManager>>) -> Result<Self> {
        Self::new_with_api_key(manager, None).await
    }

    /// Create a new web server with process manager and API key
    pub async fn new_with_api_key(
        manager: Arc<RwLock<ProcessManager>>,
        api_key: Option<String>,
    ) -> Result<Self> {
        let monitor = Arc::new(RwLock::new(Monitor::new()));
        let (broadcast_tx, _) = broadcast::channel(1000);

        let state = AppState {
            manager,
            monitor,
            broadcast_tx,
            api_key,
        };

        Ok(Self { state })
    }

    /// API key authentication middleware
    async fn auth_middleware(
        State(state): State<AppState>,
        req: axum::http::Request<axum::body::Body>,
        next: Next,
    ) -> std::result::Result<Response, StatusCode> {
        // Skip auth for root endpoint and WebSocket upgrade
        let path = req.uri().path();
        if path == "/" || path.starts_with("/ws") {
            return Ok(next.run(req).await);
        }

        // Check if API key is required
        if let Some(required_key) = &state.api_key {
            // Get Authorization header
            let auth_header = req
                .headers()
                .get(header::AUTHORIZATION)
                .and_then(|h| h.to_str().ok());

            // Check for Bearer token or API key header
            let provided_key = if let Some(auth) = auth_header {
                auth.strip_prefix("Bearer ")
                    .or_else(|| auth.strip_prefix("ApiKey "))
            } else {
                // Also check X-API-Key header
                req.headers().get("X-API-Key").and_then(|h| h.to_str().ok())
            };

            // Verify API key
            if provided_key != Some(required_key) {
                return Err(StatusCode::UNAUTHORIZED);
            }
        }

        Ok(next.run(req).await)
    }

    /// Start the web server
    pub async fn start(&self, host: &str, port: u16) -> Result<()> {
        let app = self.create_router().await;

        let addr = format!("{}:{}", host, port);
        let listener = TcpListener::bind(&addr)
            .await
            .map_err(|e| Error::web_server(format!("Failed to bind to {}: {}", addr, e)))?;

        info!("Web server listening on http://{}", addr);

        // Start background monitoring task
        let state_clone = self.state.clone();
        tokio::spawn(async move {
            if let Err(e) = Self::monitoring_task(state_clone).await {
                error!("Monitoring task error: {}", e);
            }
        });

        axum::serve(listener, app)
            .await
            .map_err(|e| Error::web_server(format!("Server error: {}", e)))?;

        Ok(())
    }

    /// Background task for monitoring and broadcasting updates
    async fn monitoring_task(state: AppState) -> Result<()> {
        let mut interval = interval(Duration::from_secs(2));

        loop {
            interval.tick().await;

            // Get system metrics
            let system_metrics = {
                let mut monitor = state.monitor.write().await;
                monitor.get_system_metrics().await
            };

            // Broadcast system update
            let _ = state.broadcast_tx.send(WebSocketMessage::SystemUpdate {
                metrics: system_metrics,
            });

            // Get process list and broadcast
            let processes = {
                let manager = state.manager.read().await;
                manager.list().await.unwrap_or_default()
            };

            let _ = state
                .broadcast_tx
                .send(WebSocketMessage::ProcessList { processes });
        }
    }

    /// Create the router with all routes
    async fn create_router(&self) -> Router {
        Router::new()
            // Root endpoint
            .route("/", get(root_handler))
            // Process management endpoints
            .route("/api/processes", get(list_processes))
            .route(
                "/api/processes/:id",
                get(get_process).delete(delete_process),
            )
            .route("/api/processes/:id/start", post(start_process))
            .route("/api/processes/:id/stop", post(stop_process))
            .route("/api/processes/:id/restart", post(restart_process))
            .route("/api/processes/:id/reload", post(reload_process))
            .route("/api/processes/:id/logs", get(get_process_logs))
            // System information
            .route("/api/system", get(system_info))
            .route("/api/status", get(status_info))
            // WebSocket endpoint
            .route("/ws", get(websocket_handler))
            // Add middleware layers
            .layer(middleware::from_fn_with_state(
                self.state.clone(),
                Self::auth_middleware,
            ))
            .layer(
                CorsLayer::new()
                    .allow_origin("*".parse::<HeaderValue>().unwrap())
                    .allow_methods([
                        axum::http::Method::GET,
                        axum::http::Method::POST,
                        axum::http::Method::DELETE,
                        axum::http::Method::OPTIONS,
                    ])
                    .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION, header::ACCEPT]),
            )
            .layer(SetResponseHeaderLayer::if_not_present(
                header::X_CONTENT_TYPE_OPTIONS,
                HeaderValue::from_static("nosniff"),
            ))
            .layer(SetResponseHeaderLayer::if_not_present(
                header::X_FRAME_OPTIONS,
                HeaderValue::from_static("DENY"),
            ))
            .layer(SetResponseHeaderLayer::if_not_present(
                header::X_XSS_PROTECTION,
                HeaderValue::from_static("1; mode=block"),
            ))
            .layer(TraceLayer::new_for_http())
            .with_state(self.state.clone())
    }
}

impl Clone for WebServer {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
        }
    }
}

// Handler functions

/// Root handler - API information
async fn root_handler() -> Json<Value> {
    Json(json!({
        "name": "PMDaemon",
        "version": env!("CARGO_PKG_VERSION"),
        "description": "PMDaemon - A high-performance, cross-platform process manager built in Rust",
        "status": "running",
        "endpoints": {
            "processes": "/api/processes",
            "system": "/api/system",
            "status": "/api/status",
            "websocket": "/ws"
        }
    }))
}

/// List all processes
async fn list_processes(
    State(state): State<AppState>,
    Query(query): Query<ListQuery>,
) -> impl IntoResponse {
    match state.manager.read().await.list().await {
        Ok(mut processes) => {
            // Filter by namespace if specified
            if let Some(namespace) = &query.namespace {
                processes.retain(|p| &p.namespace == namespace);
            }

            // Convert to PM2-compatible format
            let pm2_processes: Vec<Value> = processes
                .into_iter()
                .map(|p| process_status_to_pm2_format(&p))
                .collect();

            Json(json!({
                "success": true,
                "data": pm2_processes,
                "meta": {
                    "total": pm2_processes.len(),
                    "namespace": query.namespace
                }
            }))
            .into_response()
        }
        Err(e) => {
            error!("Failed to list processes: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": "Failed to list processes",
                    "message": e.to_string()
                })),
            )
                .into_response()
        }
    }
}

/// Get specific process
async fn get_process(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    match state.manager.read().await.get_process_info(&id).await {
        Ok(status) => Json(json!({
            "success": true,
            "data": process_status_to_pm2_format(&status)
        }))
        .into_response(),
        Err(e) => {
            warn!("Process not found: {}", id);
            (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "success": false,
                    "error": "Process not found",
                    "message": e.to_string()
                })),
            )
                .into_response()
        }
    }
}

/// Delete a process
async fn delete_process(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match state.manager.write().await.delete(&id).await {
        Ok(()) => Json(json!({
            "success": true,
            "message": format!("Process '{}' deleted successfully", id)
        }))
        .into_response(),
        Err(e) => {
            error!("Failed to delete process {}: {}", id, e);
            (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "success": false,
                    "error": "Failed to delete process",
                    "message": e.to_string()
                })),
            )
                .into_response()
        }
    }
}

/// Start a process
async fn start_process(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    // For starting, we need to get the process config and start it
    // This is different from create_process which creates a new process
    match state.manager.read().await.get_process_info(&id).await {
        Ok(status) => {
            if status.state == ProcessState::Online {
                return Json(json!({
                    "success": false,
                    "error": "Process already running",
                    "message": format!("Process '{}' is already online", id)
                }))
                .into_response();
            }

            // Process exists but is not running, try to start it
            // Note: This is a simplified implementation
            // In a real scenario, we'd need to restart the stopped process
            Json(json!({
                "success": true,
                "message": format!("Process '{}' start requested", id),
                "note": "Process restart functionality would be implemented here"
            }))
            .into_response()
        }
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(json!({
                "success": false,
                "error": "Process not found",
                "message": e.to_string()
            })),
        )
            .into_response(),
    }
}

/// Stop a process
async fn stop_process(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    match state.manager.write().await.stop(&id).await {
        Ok(()) => Json(json!({
            "success": true,
            "message": format!("Process '{}' stopped successfully", id)
        }))
        .into_response(),
        Err(e) => {
            error!("Failed to stop process {}: {}", id, e);
            (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "success": false,
                    "error": "Failed to stop process",
                    "message": e.to_string()
                })),
            )
                .into_response()
        }
    }
}

/// Restart a process
async fn restart_process(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(request): Json<Option<ProcessActionRequest>>,
) -> impl IntoResponse {
    let port_config = request.and_then(|r| {
        if let Some(port) = r.port {
            Some(PortConfig::Single(port))
        } else if let Some((start, end)) = r.port_range {
            Some(PortConfig::Range(start, end))
        } else {
            None
        }
    });

    let result = if let Some(port_config) = port_config {
        state
            .manager
            .write()
            .await
            .restart_with_port(&id, Some(port_config))
            .await
    } else {
        state.manager.write().await.restart(&id).await
    };

    match result {
        Ok(()) => Json(json!({
            "success": true,
            "message": format!("Process '{}' restarted successfully", id)
        }))
        .into_response(),
        Err(e) => {
            error!("Failed to restart process {}: {}", id, e);
            (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "success": false,
                    "error": "Failed to restart process",
                    "message": e.to_string()
                })),
            )
                .into_response()
        }
    }
}

/// Reload a process (graceful restart)
async fn reload_process(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(request): Json<Option<ProcessActionRequest>>,
) -> impl IntoResponse {
    let port_config = request.and_then(|r| {
        if let Some(port) = r.port {
            Some(PortConfig::Single(port))
        } else if let Some((start, end)) = r.port_range {
            Some(PortConfig::Range(start, end))
        } else {
            None
        }
    });

    let result = if let Some(port_config) = port_config {
        state
            .manager
            .write()
            .await
            .reload_with_port(&id, Some(port_config))
            .await
    } else {
        state.manager.write().await.reload(&id).await
    };

    match result {
        Ok(()) => Json(json!({
            "success": true,
            "message": format!("Process '{}' reloaded successfully", id)
        }))
        .into_response(),
        Err(e) => {
            error!("Failed to reload process {}: {}", id, e);
            (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "success": false,
                    "error": "Failed to reload process",
                    "message": e.to_string()
                })),
            )
                .into_response()
        }
    }
}

/// Get process logs
async fn get_process_logs(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(query): Query<LogsQuery>,
) -> impl IntoResponse {
    match state
        .manager
        .read()
        .await
        .get_logs(&id, query.lines.unwrap_or(50))
        .await
    {
        Ok(logs) => Json(json!({
            "success": true,
            "data": {
                "logs": logs,
                "lines": query.lines.unwrap_or(50),
                "process_id": id
            }
        }))
        .into_response(),
        Err(e) => {
            error!("Failed to get logs for process {}: {}", id, e);
            (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "success": false,
                    "error": "Failed to get process logs",
                    "message": e.to_string()
                })),
            )
                .into_response()
        }
    }
}

/// Get system information
async fn system_info(State(state): State<AppState>) -> impl IntoResponse {
    let system_metrics = {
        let mut monitor = state.monitor.write().await;
        monitor.get_system_metrics().await
    };

    Json(json!({
        "success": true,
        "data": {
            "system": {
                "cpu": system_metrics.cpu_usage,
                "memory": {
                    "used": system_metrics.memory_usage,
                    "total": system_metrics.memory_total,
                    "percentage": (system_metrics.memory_usage as f64 / system_metrics.memory_total as f64 * 100.0)
                },
                "load_average": system_metrics.load_average,
                "uptime": system_metrics.uptime,
                "timestamp": system_metrics.timestamp
            }
        }
    }))
}

/// Get status information (PM2-compatible)
async fn status_info(State(state): State<AppState>) -> impl IntoResponse {
    let processes = (state.manager.read().await.list().await).unwrap_or_default();

    let system_metrics = {
        let mut monitor = state.monitor.write().await;
        monitor.get_system_metrics().await
    };

    let allocated_ports = state.manager.read().await.get_allocated_ports().await;

    Json(json!({
        "success": true,
        "data": {
            "processes": processes.into_iter().map(|p| process_status_to_pm2_format(&p)).collect::<Vec<_>>(),
            "system": {
                "cpu": system_metrics.cpu_usage,
                "memory": {
                    "used": system_metrics.memory_usage,
                    "total": system_metrics.memory_total
                },
                "load_average": system_metrics.load_average,
                "uptime": system_metrics.uptime
            },
            "ports": {
                "allocated": allocated_ports,
                "total": allocated_ports.len()
            },
            "meta": {
                "pm2_version": env!("CARGO_PKG_VERSION"),
                "node_version": "N/A (Rust)",
                "timestamp": chrono::Utc::now()
            }
        }
    }))
}

/// WebSocket handler for real-time updates
async fn websocket_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> Response {
    ws.on_upgrade(move |socket| handle_websocket(socket, state))
}

/// Handle WebSocket connection
async fn handle_websocket(socket: WebSocket, state: AppState) {
    let (mut sender, mut receiver) = socket.split();
    let mut broadcast_rx = state.broadcast_tx.subscribe();

    info!("WebSocket client connected");

    // Send initial data
    let processes = (state.manager.read().await.list().await).unwrap_or_default();

    let initial_message = WebSocketMessage::ProcessList { processes };
    if let Ok(msg_json) = serde_json::to_string(&initial_message) {
        let _ = sender.send(Message::Text(msg_json)).await;
    }

    // Handle incoming messages and broadcast updates
    let send_task = tokio::spawn(async move {
        while let Ok(msg) = broadcast_rx.recv().await {
            if let Ok(msg_json) = serde_json::to_string(&msg) {
                if sender.send(Message::Text(msg_json)).await.is_err() {
                    break;
                }
            }
        }
    });

    let recv_task = tokio::spawn(async move {
        while let Some(msg) = receiver.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    debug!("Received WebSocket message: {}", text);
                    // Handle client messages if needed
                }
                Ok(Message::Close(_)) => {
                    info!("WebSocket client disconnected");
                    break;
                }
                Err(e) => {
                    warn!("WebSocket error: {}", e);
                    break;
                }
                _ => {}
            }
        }
    });

    // Wait for either task to complete
    tokio::select! {
        _ = send_task => {},
        _ = recv_task => {},
    }

    info!("WebSocket connection closed");
}

/// Convert ProcessStatus to PM2-compatible format
fn process_status_to_pm2_format(status: &ProcessStatus) -> Value {
    json!({
        "pm_id": status.id.to_string(),
        "name": status.name,
        "namespace": status.namespace,
        "version": env!("CARGO_PKG_VERSION"),
        "exec_mode": "fork",
        "pid": status.pid,
        "uptime": status.uptime.map(|u| u.timestamp()).unwrap_or(0),
        "created_at": status.uptime.map(|u| u.timestamp()).unwrap_or(0),
        "status": status.state.to_string(),
        "restart_time": status.restarts,
        "cpu": status.cpu_usage,
        "memory": status.memory_usage,
        "user": "pmdaemon",
        "watching": false,
        "instance_id": status.instance,
        "exec_interpreter": "none",
        "pm_exec_path": "",
        "pm_cwd": "",
        "exec_mode": "fork_mode",
        "node_args": [],
        "pm_out_log_path": format!("/tmp/pmdaemon-{}-out.log", status.name),
        "pm_err_log_path": format!("/tmp/pmdaemon-{}-error.log", status.name),
        "pm_pid_path": format!("/tmp/pmdaemon-{}.pid", status.name),
        "exit_code": status.exit_code,
        "port": status.assigned_port,
        "monit": {
            "memory": status.memory_usage,
            "cpu": status.cpu_usage
        },
        "pm2_env": {
            "name": status.name,
            "namespace": status.namespace,
            "exec_mode": "fork",
            "env": {},
            "pm_id": status.id.to_string(),
            "restart_time": status.restarts,
            "status": status.state.to_string(),
            "port": status.assigned_port
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::process::ProcessState;
    use chrono::Utc;
    use pretty_assertions::assert_eq;
    use serde_json::Value;

    use tempfile::TempDir;
    use uuid::Uuid;

    async fn create_test_app_state() -> (AppState, TempDir) {
        let temp_dir = TempDir::new().unwrap();

        // Use the public API to create ProcessManager
        let manager = Arc::new(RwLock::new(ProcessManager::new().await.unwrap()));
        let monitor = Arc::new(RwLock::new(Monitor::new()));
        let (broadcast_tx, _) = broadcast::channel(100);

        let state = AppState {
            manager,
            monitor,
            broadcast_tx,
            api_key: None,
        };

        (state, temp_dir)
    }

    fn create_test_process_status() -> ProcessStatus {
        ProcessStatus {
            id: Uuid::new_v4(),
            name: "test-process".to_string(),
            state: ProcessState::Online,
            pid: Some(12345),
            uptime: Some(Utc::now()),
            restarts: 2,
            cpu_usage: 25.5,
            memory_usage: 1024 * 1024, // 1MB
            exit_code: None,
            error: None,
            namespace: "default".to_string(),
            instance: Some(1),
            assigned_port: Some(8080),
        }
    }

    #[test]
    fn test_list_query_deserialize() {
        let json = r#"{"namespace": "production", "monitoring": true}"#;
        let query: ListQuery = serde_json::from_str(json).unwrap();
        assert_eq!(query.namespace, Some("production".to_string()));
        assert_eq!(query.monitoring, Some(true));
    }

    #[test]
    fn test_logs_query_deserialize() {
        let json = r#"{"lines": 100, "follow": false}"#;
        let query: LogsQuery = serde_json::from_str(json).unwrap();
        assert_eq!(query.lines, Some(100));
        assert_eq!(query.follow, Some(false));
    }

    #[test]
    fn test_process_action_request_deserialize() {
        let json = r#"{"port": 9000, "port_range": [3000, 3005]}"#;
        let request: ProcessActionRequest = serde_json::from_str(json).unwrap();
        assert_eq!(request.port, Some(9000));
        assert_eq!(request.port_range, Some((3000, 3005)));
    }

    #[test]
    fn test_websocket_message_serialize() {
        let status = create_test_process_status();
        let msg = WebSocketMessage::ProcessUpdate {
            process: status.clone(),
        };

        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("ProcessUpdate"));
        assert!(json.contains("test-process"));
    }

    #[test]
    fn test_websocket_message_system_update() {
        let memory_usage = 2048;
        let memory_total = 8192;
        let memory_percent = (memory_usage as f64 / memory_total as f64 * 100.0) as f32;

        let metrics = SystemMetrics {
            cpu_usage: 50.0,
            memory_usage,
            memory_total,
            memory_percent,
            memory_used: memory_usage,
            load_average: [1.0, 1.5, 2.0],
            uptime: 3600,
            timestamp: Utc::now(),
        };

        let msg = WebSocketMessage::SystemUpdate { metrics };
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("SystemUpdate"));
        assert!(json.contains("50.0"));
    }

    #[test]
    fn test_websocket_message_error() {
        let msg = WebSocketMessage::Error {
            message: "Test error".to_string(),
        };

        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("Error"));
        assert!(json.contains("Test error"));
    }

    #[tokio::test]
    async fn test_web_server_new() {
        let (state, _temp_dir) = create_test_app_state().await;
        let server = WebServer::new(state.manager).await;
        assert!(server.is_ok());
    }

    #[tokio::test]
    async fn test_web_server_clone() {
        let (state, _temp_dir) = create_test_app_state().await;
        let server = WebServer::new(state.manager).await.unwrap();
        let cloned = server.clone();

        // Both should have the same state (Arc pointers)
        assert!(Arc::ptr_eq(&server.state.monitor, &cloned.state.monitor));
    }

    #[test]
    fn test_process_status_to_pm2_format() {
        let status = create_test_process_status();
        let pm2_format = process_status_to_pm2_format(&status);

        assert_eq!(pm2_format["name"], "test-process");
        assert_eq!(pm2_format["namespace"], "default");
        assert_eq!(pm2_format["pid"], 12345);
        assert_eq!(pm2_format["status"], "online");
        assert_eq!(pm2_format["restart_time"], 2);
        assert_eq!(pm2_format["cpu"], 25.5);
        assert_eq!(pm2_format["memory"], 1024 * 1024);
        assert_eq!(pm2_format["port"], 8080);
        assert_eq!(pm2_format["instance_id"], 1);

        // Check nested objects
        assert_eq!(pm2_format["monit"]["cpu"], 25.5);
        assert_eq!(pm2_format["monit"]["memory"], 1024 * 1024);
        assert_eq!(pm2_format["pm2_env"]["name"], "test-process");
        assert_eq!(pm2_format["pm2_env"]["status"], "online");
    }

    #[tokio::test]
    async fn test_root_handler() {
        let response = root_handler().await;
        let json = response.0;

        assert_eq!(json["name"], "PMDaemon");
        assert_eq!(json["status"], "running");
        assert!(json["endpoints"].is_object());
        assert!(json["endpoints"]["processes"].is_string());
        assert!(json["endpoints"]["websocket"].is_string());
    }

    #[tokio::test]
    async fn test_app_state_clone() {
        let (state, _temp_dir) = create_test_app_state().await;
        let cloned = state.clone();

        // Should be the same Arc pointers
        assert!(Arc::ptr_eq(&state.manager, &cloned.manager));
        assert!(Arc::ptr_eq(&state.monitor, &cloned.monitor));
    }

    #[test]
    fn test_websocket_message_debug() {
        let msg = WebSocketMessage::Error {
            message: "Debug test".to_string(),
        };

        let debug_str = format!("{:?}", msg);
        assert!(debug_str.contains("Error"));
        assert!(debug_str.contains("Debug test"));
    }

    #[test]
    fn test_websocket_message_clone() {
        let original = WebSocketMessage::Error {
            message: "Clone test".to_string(),
        };

        let cloned = original.clone();
        match (&original, &cloned) {
            (
                WebSocketMessage::Error { message: msg1 },
                WebSocketMessage::Error { message: msg2 },
            ) => {
                assert_eq!(msg1, msg2);
            }
            _ => panic!("Clone failed"),
        }
    }

    #[test]
    fn test_list_query_default_values() {
        let json = r#"{}"#;
        let query: ListQuery = serde_json::from_str(json).unwrap();
        assert!(query.namespace.is_none());
        assert!(query.monitoring.is_none());
    }

    #[test]
    fn test_logs_query_default_values() {
        let json = r#"{}"#;
        let query: LogsQuery = serde_json::from_str(json).unwrap();
        assert!(query.lines.is_none());
        assert!(query.follow.is_none());
    }

    #[test]
    fn test_process_action_request_default_values() {
        let json = r#"{}"#;
        let request: ProcessActionRequest = serde_json::from_str(json).unwrap();
        assert!(request.port.is_none());
        assert!(request.port_range.is_none());
    }

    #[test]
    fn test_websocket_message_process_list() {
        let processes = vec![create_test_process_status()];
        let msg = WebSocketMessage::ProcessList {
            processes: processes.clone(),
        };

        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("ProcessList"));
        assert!(json.contains("test-process"));
    }

    #[tokio::test]
    async fn test_api_key_authentication() {
        use axum::body::Body;
        use axum::http::{Method, Request, StatusCode};
        use tower::ServiceExt;

        // Create test state with API key
        let _temp_dir = TempDir::new().unwrap();
        let manager = Arc::new(RwLock::new(ProcessManager::new().await.unwrap()));
        let monitor = Arc::new(RwLock::new(Monitor::new()));
        let (broadcast_tx, _) = broadcast::channel(100);

        let test_api_key = "test-secret-api-key-12345".to_string();
        let state = AppState {
            manager,
            monitor,
            broadcast_tx,
            api_key: Some(test_api_key.clone()),
        };

        // Create web server with API key
        let server = WebServer { state };
        let app = server.create_router().await;

        // Test 1: Root endpoint should be accessible without authentication
        let request = Request::builder()
            .method(Method::GET)
            .uri("/")
            .body(Body::empty())
            .unwrap();

        let response = app.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        // Test 2: WebSocket endpoint should be accessible without authentication
        let request = Request::builder()
            .method(Method::GET)
            .uri("/ws")
            .header("upgrade", "websocket")
            .header("connection", "upgrade")
            .header("sec-websocket-key", "dGhlIHNhbXBsZSBub25jZQ==")
            .header("sec-websocket-version", "13")
            .body(Body::empty())
            .unwrap();

        let response = app.clone().oneshot(request).await.unwrap();
        // Should get 426 Upgrade Required for WebSocket without proper setup, not 401
        assert_ne!(response.status(), StatusCode::UNAUTHORIZED);

        // Test 3: API endpoint without authentication should return 401
        let request = Request::builder()
            .method(Method::GET)
            .uri("/api/processes")
            .body(Body::empty())
            .unwrap();

        let response = app.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

        // Test 4: API endpoint with correct Bearer token should succeed
        let request = Request::builder()
            .method(Method::GET)
            .uri("/api/processes")
            .header("Authorization", format!("Bearer {}", test_api_key))
            .body(Body::empty())
            .unwrap();

        let response = app.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        // Test 5: API endpoint with correct ApiKey header should succeed
        let request = Request::builder()
            .method(Method::GET)
            .uri("/api/processes")
            .header("Authorization", format!("ApiKey {}", test_api_key))
            .body(Body::empty())
            .unwrap();

        let response = app.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        // Test 6: API endpoint with correct X-API-Key header should succeed
        let request = Request::builder()
            .method(Method::GET)
            .uri("/api/processes")
            .header("X-API-Key", &test_api_key)
            .body(Body::empty())
            .unwrap();

        let response = app.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        // Test 7: API endpoint with incorrect API key should return 401
        let request = Request::builder()
            .method(Method::GET)
            .uri("/api/processes")
            .header("Authorization", "Bearer wrong-api-key")
            .body(Body::empty())
            .unwrap();

        let response = app.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

        // Test 8: API endpoint with malformed Authorization header should return 401
        let request = Request::builder()
            .method(Method::GET)
            .uri("/api/processes")
            .header("Authorization", "InvalidFormat")
            .body(Body::empty())
            .unwrap();

        let response = app.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

        // Test 9: Different API endpoints should all require authentication
        let protected_endpoints = vec!["/api/system", "/api/status", "/api/processes/test-id"];

        for endpoint in protected_endpoints {
            // Without auth - should fail
            let request = Request::builder()
                .method(Method::GET)
                .uri(endpoint)
                .body(Body::empty())
                .unwrap();

            let response = app.clone().oneshot(request).await.unwrap();
            assert_eq!(
                response.status(),
                StatusCode::UNAUTHORIZED,
                "Endpoint {} should require auth",
                endpoint
            );

            // With auth - should succeed (or at least not return 401)
            let request = Request::builder()
                .method(Method::GET)
                .uri(endpoint)
                .header("Authorization", format!("Bearer {}", test_api_key))
                .body(Body::empty())
                .unwrap();

            let response = app.clone().oneshot(request).await.unwrap();
            assert_ne!(
                response.status(),
                StatusCode::UNAUTHORIZED,
                "Endpoint {} should accept valid auth",
                endpoint
            );
        }
    }

    #[tokio::test]
    async fn test_no_api_key_authentication() {
        use axum::body::Body;
        use axum::http::{Method, Request, StatusCode};
        use tower::ServiceExt;

        // Create test state without API key
        let _temp_dir = TempDir::new().unwrap();
        let manager = Arc::new(RwLock::new(ProcessManager::new().await.unwrap()));
        let monitor = Arc::new(RwLock::new(Monitor::new()));
        let (broadcast_tx, _) = broadcast::channel(100);

        let state = AppState {
            manager,
            monitor,
            broadcast_tx,
            api_key: None, // No API key required
        };

        // Create web server without API key
        let server = WebServer { state };
        let app = server.create_router().await;

        // Test: API endpoints should be accessible without authentication when no API key is set
        let request = Request::builder()
            .method(Method::GET)
            .uri("/api/processes")
            .body(Body::empty())
            .unwrap();

        let response = app.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        // Test: System endpoint should also be accessible
        let request = Request::builder()
            .method(Method::GET)
            .uri("/api/system")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[test]
    fn test_process_status_to_pm2_format_comprehensive() {
        let mut status = create_test_process_status();
        status.state = ProcessState::Stopped;
        status.pid = None;
        status.exit_code = Some(1);
        status.error = Some("Process crashed".to_string());
        status.assigned_port = None;

        let pm2_format = process_status_to_pm2_format(&status);

        assert_eq!(pm2_format["status"], "stopped");
        assert_eq!(pm2_format["pid"], Value::Null);
        assert_eq!(pm2_format["exit_code"], 1);
        assert_eq!(pm2_format["port"], Value::Null);

        // Check that all required PM2 fields are present
        assert!(pm2_format["pm_id"].is_string());
        assert!(pm2_format["exec_mode"].is_string());
        assert!(pm2_format["user"].is_string());
        assert!(pm2_format["watching"].is_boolean());
        assert!(pm2_format["pm_out_log_path"].is_string());
        assert!(pm2_format["pm_err_log_path"].is_string());
        assert!(pm2_format["pm_pid_path"].is_string());
    }
}
