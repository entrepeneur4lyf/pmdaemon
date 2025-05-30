# Library Usage

PMDaemon can be used as a Rust library, allowing you to embed process management capabilities directly into your applications. This guide shows how to integrate PMDaemon's powerful process management features into your Rust projects.

## Installation

Add PMDaemon to your `Cargo.toml`:

```toml
[dependencies]
pmdaemon = "0.1.1"
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## Basic Usage

### Process Manager Initialization

```rust
use pmdaemon::{ProcessManager, ProcessConfig, HealthCheckConfig};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the process manager
    let mut manager = ProcessManager::new().await?;
    
    // Create a simple process configuration
    let config = ProcessConfig {
        name: "web-server".to_string(),
        script: "node".to_string(),
        args: vec!["server.js".to_string()],
        instances: 2,
        port: Some("3000-3001".to_string()),
        env: {
            let mut env = HashMap::new();
            env.insert("NODE_ENV".to_string(), "production".to_string());
            env
        },
        ..Default::default()
    };
    
    // Start the process
    let process_id = manager.start(config).await?;
    println!("Started process with ID: {}", process_id);
    
    Ok(())
}
```

### Process Configuration

```rust
use pmdaemon::{ProcessConfig, HealthCheckConfig, HealthCheckType};
use std::time::Duration;

fn create_web_service_config() -> ProcessConfig {
    ProcessConfig {
        name: "api-service".to_string(),
        script: "node".to_string(),
        args: vec!["dist/api.js".to_string()],
        instances: 4,
        port: Some("8000-8003".to_string()),
        cwd: Some("/var/www/api".to_string()),
        env: {
            let mut env = std::collections::HashMap::new();
            env.insert("NODE_ENV".to_string(), "production".to_string());
            env.insert("DATABASE_URL".to_string(), "postgres://localhost/mydb".to_string());
            env
        },
        max_memory_restart: Some("512M".to_string()),
        autorestart: true,
        max_restarts: 5,
        min_uptime: Duration::from_secs(10),
        restart_delay: Duration::from_secs(2),
        kill_timeout: Duration::from_secs(30),
        health_check: Some(HealthCheckConfig {
            check_type: HealthCheckType::Http,
            url: Some("http://localhost:8000/health".to_string()),
            script: None,
            timeout: Duration::from_secs(10),
            interval: Duration::from_secs(30),
            retries: 3,
            enabled: true,
        }),
        ..Default::default()
    }
}

fn create_worker_config() -> ProcessConfig {
    ProcessConfig {
        name: "background-worker".to_string(),
        script: "python".to_string(),
        args: vec!["-m".to_string(), "celery".to_string(), "worker".to_string(), "-A".to_string(), "tasks".to_string()],
        instances: 2,
        cwd: Some("/var/www/workers".to_string()),
        env: {
            let mut env = std::collections::HashMap::new();
            env.insert("CELERY_BROKER_URL".to_string(), "redis://localhost:6379/0".to_string());
            env.insert("CELERY_RESULT_BACKEND".to_string(), "redis://localhost:6379/0".to_string());
            env
        },
        max_memory_restart: Some("256M".to_string()),
        health_check: Some(HealthCheckConfig {
            check_type: HealthCheckType::Script,
            url: None,
            script: Some("./scripts/worker-health.sh".to_string()),
            timeout: Duration::from_secs(15),
            interval: Duration::from_secs(60),
            retries: 2,
            enabled: true,
        }),
        ..Default::default()
    }
}
```

## Advanced Usage

### Custom Process Manager

```rust
use pmdaemon::{ProcessManager, ProcessConfig, ProcessStatus, ProcessInfo};
use tokio::time::{sleep, Duration};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct CustomProcessManager {
    manager: ProcessManager,
    processes: Arc<RwLock<std::collections::HashMap<String, ProcessInfo>>>,
}

impl CustomProcessManager {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let manager = ProcessManager::new().await?;
        let processes = Arc::new(RwLock::new(std::collections::HashMap::new()));
        
        Ok(Self {
            manager,
            processes,
        })
    }
    
    pub async fn deploy_service(&mut self, name: &str, config: ProcessConfig) -> Result<(), Box<dyn std::error::Error>> {
        println!("🚀 Deploying service: {}", name);
        
        // Stop existing service if it exists
        if let Ok(_) = self.manager.get_process_info(name).await {
            println!("🛑 Stopping existing service: {}", name);
            self.manager.stop(name).await?;
            self.wait_for_stop(name).await?;
        }
        
        // Start new service
        let process_id = self.manager.start(config.clone()).await?;
        println!("✅ Started service {} with ID: {}", name, process_id);
        
        // Health checks are configured in the ProcessConfig
        // The process manager handles health monitoring internally
        
        // Update our process tracking
        let info = self.manager.get_process_info(name).await?;
        self.processes.write().await.insert(name.to_string(), info);
        
        Ok(())
    }
    
    pub async fn scale_service(&mut self, name: &str, instances: u32) -> Result<(), Box<dyn std::error::Error>> {
        println!("📈 Scaling service {} to {} instances", name, instances);
        
        let mut config = self.manager.get_process_config(name).await?;
        config.instances = instances;
        
        // Restart with new instance count
        self.manager.restart_process_with_config(name, config).await?;
        
        // Wait for all instances to be healthy
        self.manager.wait_for_healthy(name, Duration::from_secs(120)).await?;
        
        println!("✅ Service {} scaled to {} instances", name, instances);
        Ok(())
    }
    
    pub async fn rolling_update(&mut self, name: &str, new_config: ProcessConfig) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔄 Performing rolling update for service: {}", name);
        
        let current_info = self.manager.get_process_info(name).await?;
        let instances = current_info.instances;
        
        // Update one instance at a time
        for i in 0..instances {
            println!("🔄 Updating instance {} of {}", i + 1, instances);
            
            // Stop one instance
            self.manager.stop_process_instance(name, i).await?;
            
            // Start new instance with new config
            self.manager.start_process_instance(name, i, new_config.clone()).await?;
            
            // Wait for it to be healthy
            self.manager.wait_for_instance_healthy(name, i, Duration::from_secs(60)).await?;
            
            // Small delay between updates
            sleep(Duration::from_secs(5)).await;
        }
        
        println!("✅ Rolling update completed for service: {}", name);
        Ok(())
    }
    
    pub async fn monitor_processes(&self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            let processes = self.manager.list_processes().await?;
            
            for process in processes {
                if process.status == ProcessStatus::Errored {
                    println!("⚠️  Process {} is in error state, attempting restart", process.name);
                    
                    if let Err(e) = self.manager.restart_process(&process.name).await {
                        println!("❌ Failed to restart {}: {}", process.name, e);
                    } else {
                        println!("✅ Restarted process: {}", process.name);
                    }
                }
                
                // Check memory usage
                if let Some(memory_limit) = &process.max_memory_restart {
                    let limit_bytes = parse_memory_limit(memory_limit)?;
                    if process.memory_usage > (limit_bytes as f64 * 0.9) {
                        println!("⚠️  Process {} is approaching memory limit", process.name);
                    }
                }
            }
            
            sleep(Duration::from_secs(30)).await;
        }
    }
    
    async fn wait_for_stop(&self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let timeout = Duration::from_secs(30);
        let start = std::time::Instant::now();
        
        while start.elapsed() < timeout {
            match self.manager.get_process_info(name).await {
                Err(_) => return Ok(()), // Process not found, it's stopped
                Ok(info) if info.status == ProcessStatus::Stopped => return Ok(()),
                _ => sleep(Duration::from_secs(1)).await,
            }
        }
        
        Err("Process did not stop within timeout".into())
    }
}

fn parse_memory_limit(limit: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let limit = limit.to_uppercase();
    let (number, unit) = if limit.ends_with("GB") || limit.ends_with("G") {
        (limit.trim_end_matches("GB").trim_end_matches("G").parse::<f64>()?, 1024 * 1024 * 1024)
    } else if limit.ends_with("MB") || limit.ends_with("M") {
        (limit.trim_end_matches("MB").trim_end_matches("M").parse::<f64>()?, 1024 * 1024)
    } else if limit.ends_with("KB") || limit.ends_with("K") {
        (limit.trim_end_matches("KB").trim_end_matches("K").parse::<f64>()?, 1024)
    } else {
        (limit.parse::<f64>()?, 1)
    };
    
    Ok((number * unit as f64) as u64)
}
```

### Event Handling

```rust
use pmdaemon::{ProcessManager, ProcessEvent, ProcessStatus};
use tokio::sync::mpsc;

pub struct ProcessEventHandler {
    manager: ProcessManager,
    event_receiver: mpsc::Receiver<ProcessEvent>,
}

impl ProcessEventHandler {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let manager = ProcessManager::new().await?;
        let event_receiver = manager.subscribe_to_events().await?;
        
        Ok(Self {
            manager,
            event_receiver,
        })
    }
    
    pub async fn handle_events(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        while let Some(event) = self.event_receiver.recv().await {
            match event {
                ProcessEvent::StatusChanged { name, old_status, new_status } => {
                    self.handle_status_change(&name, old_status, new_status).await?;
                }
                ProcessEvent::HealthCheckFailed { name, error } => {
                    self.handle_health_check_failure(&name, &error).await?;
                }
                ProcessEvent::MemoryLimitExceeded { name, current_memory, limit } => {
                    self.handle_memory_limit_exceeded(&name, current_memory, limit).await?;
                }
                ProcessEvent::ProcessCrashed { name, exit_code, signal } => {
                    self.handle_process_crash(&name, exit_code, signal).await?;
                }
            }
        }
        
        Ok(())
    }
    
    async fn handle_status_change(&self, name: &str, old_status: ProcessStatus, new_status: ProcessStatus) -> Result<(), Box<dyn std::error::Error>> {
        println!("📊 Process {} status changed: {:?} -> {:?}", name, old_status, new_status);
        
        match new_status {
            ProcessStatus::Online => {
                println!("✅ Process {} is now online", name);
                self.send_notification(&format!("Process {} started successfully", name)).await?;
            }
            ProcessStatus::Errored => {
                println!("❌ Process {} has errored", name);
                self.send_alert(&format!("Process {} has failed", name)).await?;
            }
            ProcessStatus::Stopped => {
                println!("🛑 Process {} has stopped", name);
            }
            _ => {}
        }
        
        Ok(())
    }
    
    async fn handle_health_check_failure(&self, name: &str, error: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("🏥 Health check failed for {}: {}", name, error);
        
        // Attempt to restart the process
        if let Err(e) = self.manager.restart_process(name).await {
            println!("❌ Failed to restart {} after health check failure: {}", name, e);
            self.send_alert(&format!("Critical: Process {} health check failed and restart failed", name)).await?;
        } else {
            println!("🔄 Restarted {} due to health check failure", name);
            self.send_notification(&format!("Process {} restarted due to health check failure", name)).await?;
        }
        
        Ok(())
    }
    
    async fn handle_memory_limit_exceeded(&self, name: &str, current_memory: u64, limit: u64) -> Result<(), Box<dyn std::error::Error>> {
        println!("💾 Process {} exceeded memory limit: {} > {}", name, current_memory, limit);
        
        // Log memory usage details
        if let Ok(info) = self.manager.get_process_info(name).await {
            println!("Memory details for {}: RSS={}, Heap={}", name, info.memory_usage, info.heap_usage.unwrap_or(0));
        }
        
        self.send_alert(&format!("Process {} exceeded memory limit and was restarted", name)).await?;
        
        Ok(())
    }
    
    async fn handle_process_crash(&self, name: &str, exit_code: Option<i32>, signal: Option<i32>) -> Result<(), Box<dyn std::error::Error>> {
        let crash_info = match (exit_code, signal) {
            (Some(code), None) => format!("exit code {}", code),
            (None, Some(sig)) => format!("signal {}", sig),
            (Some(code), Some(sig)) => format!("exit code {} (signal {})", code, sig),
            (None, None) => "unknown reason".to_string(),
        };
        
        println!("💥 Process {} crashed: {}", name, crash_info);
        
        // Get crash logs
        if let Ok(logs) = self.manager.get_process_logs(name, 50).await {
            println!("Recent logs for {}:", name);
            for log in logs.iter().take(10) {
                println!("  {}", log.message);
            }
        }
        
        self.send_alert(&format!("Process {} crashed: {}", name, crash_info)).await?;
        
        Ok(())
    }
    
    async fn send_notification(&self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Implement your notification logic here
        // This could send to Slack, email, etc.
        println!("📢 Notification: {}", message);
        Ok(())
    }
    
    async fn send_alert(&self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Implement your alerting logic here
        // This could send to PagerDuty, email, etc.
        println!("🚨 Alert: {}", message);
        Ok(())
    }
}
```

### Integration Example

```rust
use pmdaemon::{ProcessManager, ProcessConfig};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct ApplicationManager {
    process_manager: Arc<RwLock<ProcessManager>>,
}

impl ApplicationManager {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let manager = ProcessManager::new().await?;
        
        Ok(Self {
            process_manager: Arc::new(RwLock::new(manager)),
        })
    }
    
    pub async fn deploy_application(&self, app_config: ApplicationConfig) -> Result<(), Box<dyn std::error::Error>> {
        let mut manager = self.process_manager.write().await;
        
        // Deploy web servers
        for (i, web_config) in app_config.web_servers.iter().enumerate() {
            let process_config = ProcessConfig {
                name: format!("{}-web-{}", app_config.name, i),
                script: web_config.script.clone(),
                args: web_config.args.clone(),
                instances: web_config.instances,
                port: web_config.port.clone(),
                env: web_config.env.clone(),
                health_check: web_config.health_check.clone(),
                ..Default::default()
            };
            
            manager.start_process(process_config).await?;
        }
        
        // Deploy workers
        for (i, worker_config) in app_config.workers.iter().enumerate() {
            let process_config = ProcessConfig {
                name: format!("{}-worker-{}", app_config.name, i),
                script: worker_config.script.clone(),
                args: worker_config.args.clone(),
                instances: worker_config.instances,
                env: worker_config.env.clone(),
                health_check: worker_config.health_check.clone(),
                ..Default::default()
            };
            
            manager.start_process(process_config).await?;
        }
        
        println!("✅ Application {} deployed successfully", app_config.name);
        Ok(())
    }
    
    pub async fn get_application_status(&self, app_name: &str) -> Result<ApplicationStatus, Box<dyn std::error::Error>> {
        let manager = self.process_manager.read().await;
        let processes = manager.list_processes().await?;
        
        let app_processes: Vec<_> = processes
            .into_iter()
            .filter(|p| p.name.starts_with(&format!("{}-", app_name)))
            .collect();
        
        let total_processes = app_processes.len();
        let healthy_processes = app_processes.iter().filter(|p| p.health == Some("healthy".to_string())).count();
        let online_processes = app_processes.iter().filter(|p| p.status == ProcessStatus::Online).count();
        
        Ok(ApplicationStatus {
            name: app_name.to_string(),
            total_processes,
            healthy_processes,
            online_processes,
            processes: app_processes,
        })
    }
}

#[derive(Debug)]
pub struct ApplicationConfig {
    pub name: String,
    pub web_servers: Vec<ServiceConfig>,
    pub workers: Vec<ServiceConfig>,
}

#[derive(Debug)]
pub struct ServiceConfig {
    pub script: String,
    pub args: Vec<String>,
    pub instances: u32,
    pub port: Option<String>,
    pub env: std::collections::HashMap<String, String>,
    pub health_check: Option<pmdaemon::HealthCheckConfig>,
}

#[derive(Debug)]
pub struct ApplicationStatus {
    pub name: String,
    pub total_processes: usize,
    pub healthy_processes: usize,
    pub online_processes: usize,
    pub processes: Vec<pmdaemon::ProcessInfo>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_manager = ApplicationManager::new().await?;
    
    // Example application configuration
    let app_config = ApplicationConfig {
        name: "my-web-app".to_string(),
        web_servers: vec![
            ServiceConfig {
                script: "node".to_string(),
                args: vec!["server.js".to_string()],
                instances: 4,
                port: Some("3000-3003".to_string()),
                env: {
                    let mut env = std::collections::HashMap::new();
                    env.insert("NODE_ENV".to_string(), "production".to_string());
                    env
                },
                health_check: Some(pmdaemon::HealthCheckConfig {
                    check_type: pmdaemon::HealthCheckType::Http,
                    url: Some("http://localhost:3000/health".to_string()),
                    script: None,
                    timeout: std::time::Duration::from_secs(10),
                    interval: std::time::Duration::from_secs(30),
                    retries: 3,
                    enabled: true,
                }),
            }
        ],
        workers: vec![
            ServiceConfig {
                script: "python".to_string(),
                args: vec!["-m".to_string(), "celery".to_string(), "worker".to_string()],
                instances: 2,
                port: None,
                env: {
                    let mut env = std::collections::HashMap::new();
                    env.insert("CELERY_BROKER_URL".to_string(), "redis://localhost:6379".to_string());
                    env
                },
                health_check: None,
            }
        ],
    };
    
    // Deploy the application
    app_manager.deploy_application(app_config).await?;
    
    // Check application status
    let status = app_manager.get_application_status("my-web-app").await?;
    println!("Application status: {:?}", status);
    
    Ok(())
}
```

## Best Practices

### 1. Error Handling

```rust
use pmdaemon::{ProcessManager, PMDaemonError};

async fn robust_process_management() -> Result<(), Box<dyn std::error::Error>> {
    let mut manager = ProcessManager::new().await?;
    
    match manager.start_process(config).await {
        Ok(process_id) => {
            println!("Process started with ID: {}", process_id);
        }
        Err(PMDaemonError::ProcessAlreadyExists(name)) => {
            println!("Process {} already exists, restarting...", name);
            manager.restart(&name).await?;
        }
        Err(PMDaemonError::PortConflict(port)) => {
            println!("Port {} is in use, trying auto-assignment...", port);
            // Retry with auto port assignment
        }
        Err(e) => {
            return Err(e.into());
        }
    }
    
    Ok(())
}
```

### 2. Resource Management

```rust
use pmdaemon::ProcessManager;
use std::sync::Arc;

// Use Arc for shared ownership
let manager = Arc::new(ProcessManager::new().await?);

// Clone for use in different tasks
let manager_clone = Arc::clone(&manager);
tokio::spawn(async move {
    // Use manager_clone in this task
});
```

### 3. Configuration Validation

```rust
use pmdaemon::{ProcessConfig, ConfigValidator};

fn validate_config(config: &ProcessConfig) -> Result<(), String> {
    let validator = ConfigValidator::new();
    validator.validate(config)
}
```

## Next Steps

- **[API Examples](./api-examples.md)** - REST and WebSocket API usage
- **[WebSocket API](./websocket-api.md)** - Real-time communication
- **[Integration Examples](../examples/integration-examples.md)** - Framework integration
- **[Advanced Configuration](../configuration/advanced-configuration.md)** - Complex configurations
