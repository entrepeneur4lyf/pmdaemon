//! PMDaemon CLI
//!
//! Command-line interface for the PMDaemon process manager.

use clap::{Parser, Subcommand};
use pmdaemon::{ProcessManager, ProcessConfig, EcosystemConfig, Result};
use pmdaemon::config::format_memory;
use std::path::PathBuf;
use tracing::{error, info};
use comfy_table::{Table, Cell, Attribute, Color};
use chrono::{Local, Utc};

#[derive(Parser)]
#[command(name = "pmdaemon")]
#[command(about = "A process manager built in Rust inspired by PM2")]
#[command(version = pmdaemon::VERSION)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose logging
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Configuration file path
    #[arg(short, long, global = true)]
    config: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start a process
    Start {
        /// Script or command to execute (optional when using config file)
        script: Option<String>,

        /// Process name (optional when using config file)
        #[arg(short, long)]
        name: Option<String>,

        /// Number of instances
        #[arg(short, long, default_value = "1")]
        instances: u32,

        /// Working directory
        #[arg(long)]
        cwd: Option<PathBuf>,

        /// Environment variables (key=value)
        #[arg(short, long)]
        env: Vec<String>,

        /// Maximum memory before restart (e.g., 100M, 1G)
        #[arg(long)]
        max_memory: Option<String>,

        /// Port or port range (e.g., 3000, 3000-3010, auto:3000-3100)
        #[arg(short, long)]
        port: Option<String>,

        /// Command line arguments
        #[arg(last = true)]
        args: Vec<String>,
    },

    /// Stop a process
    Stop {
        /// Process name or ID
        identifier: String,
    },

    /// Restart a process
    Restart {
        /// Process name or ID
        identifier: String,

        /// Port or port range (overrides config default)
        #[arg(short, long)]
        port: Option<String>,
    },

    /// Reload a process (graceful restart)
    Reload {
        /// Process name or ID
        identifier: String,

        /// Port or port range (overrides config default)
        #[arg(short, long)]
        port: Option<String>,
    },

    /// Delete a process
    Delete {
        /// Process name, ID, status, or "all"
        identifier: String,

        /// Delete by status (e.g., "stopped", "errored")
        #[arg(long)]
        status: bool,

        /// Force deletion without confirmation
        #[arg(short, long)]
        force: bool,
    },

    /// List all processes
    List,

    /// Monitor processes in real-time
    Monit {
        /// Update interval in seconds
        #[arg(short, long, default_value = "1")]
        interval: u64,
    },

    /// Show process logs
    Logs {
        /// Process name or ID
        identifier: Option<String>,

        /// Number of lines to show
        #[arg(short, long, default_value = "20")]
        lines: usize,

        /// Follow log output
        #[arg(short, long)]
        follow: bool,
    },

    /// Show process information
    Info {
        /// Process name or ID
        identifier: String,
    },

    /// Start web monitoring server
    Web {
        /// Port to bind to
        #[arg(short, long, default_value_t = pmdaemon::DEFAULT_WEB_PORT)]
        port: u16,

        /// Host to bind to
        #[arg(long, default_value = "127.0.0.1")]
        host: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    let log_level = if cli.verbose { "debug" } else { "info" };
    tracing_subscriber::fmt()
        .with_env_filter(format!(
            "pmdaemon={},pmdaemon_cli={}",
            log_level, log_level
        ))
        .init();

    info!("PMDaemon v{} starting", pmdaemon::VERSION);

    // Initialize process manager
    let mut manager = ProcessManager::new().await?;

    match cli.command {
        Commands::Start {
            script,
            name,
            instances,
            cwd,
            env,
            max_memory,
            port,
            args,
        } => {
            // Handle config file vs CLI arguments
            if let Some(config_path) = &cli.config {
                // Load from config file
                let ecosystem = EcosystemConfig::from_file(config_path).await?;

                if let Some(app_name) = &name {
                    // Start specific app from config
                    if let Some(app_config) = ecosystem.get_app(app_name) {
                        let process_id = manager.start(app_config.clone()).await?;
                        println!("Started process '{}' from config file with ID: {}", app_name, process_id);
                    } else {
                        error!("App '{}' not found in config file. Available apps: {}",
                               app_name, ecosystem.app_names().join(", "));
                        std::process::exit(1);
                    }
                } else if script.is_none() {
                    // Start all apps from config
                    let mut started_count = 0;
                    for app_config in &ecosystem.apps {
                        match manager.start(app_config.clone()).await {
                            Ok(process_id) => {
                                println!("Started process '{}' with ID: {}", app_config.name, process_id);
                                started_count += 1;
                            }
                            Err(e) => {
                                error!("Failed to start process '{}': {}", app_config.name, e);
                            }
                        }
                    }
                    println!("Started {} processes from config file", started_count);
                } else {
                    error!("Cannot specify both config file and script. Use config file OR CLI arguments, not both.");
                    std::process::exit(1);
                }
            } else {
                // Traditional CLI mode
                let script = script.ok_or_else(|| {
                    pmdaemon::Error::config("Script is required when not using config file")
                })?;

                let process_name = name.unwrap_or_else(|| {
                    std::path::Path::new(&script)
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("unnamed")
                        .to_string()
                });

                let mut config_builder = ProcessConfig::builder()
                    .name(process_name)
                    .script(script)
                    .instances(instances)
                    .args(args);

                if let Some(cwd) = cwd {
                    config_builder = config_builder.cwd(cwd);
                }

                // Parse memory limit
                if let Some(memory_str) = max_memory {
                    match pmdaemon::config::parse_memory_string(&memory_str) {
                        Ok(memory_bytes) => {
                            config_builder = config_builder.max_memory_restart(memory_bytes);
                            info!("Set memory limit to {} bytes ({})", memory_bytes, memory_str);
                        }
                        Err(e) => {
                            error!("Invalid memory format '{}': {}", memory_str, e);
                            std::process::exit(1);
                        }
                    }
                }

                // Parse port configuration
                if let Some(port_str) = port {
                    match pmdaemon::config::PortConfig::parse(&port_str) {
                        Ok(port_config) => {
                            config_builder = config_builder.port(port_config);
                            info!("Set port configuration: {}", port_str);
                        }
                        Err(e) => {
                            error!("Invalid port format '{}': {}", port_str, e);
                            std::process::exit(1);
                        }
                    }
                }

                // Parse environment variables
                for env_var in env {
                    if let [key_slice, value_slice] = env_var.splitn(2, '=').collect::<Vec<_>>().as_slice() {
                        config_builder = config_builder.env(*key_slice, *value_slice);
                    } else {
                        error!("Invalid environment variable format: {}", env_var);
                        std::process::exit(1);
                    }
                }

                let config = config_builder.build()?;
                let process_id = manager.start(config).await?;

                println!("Started process with ID: {}", process_id);
            }
        }

        Commands::Stop { identifier } => {
            manager.stop(&identifier).await?;
            println!("Stopped process: {}", identifier);
        }

        Commands::Restart { identifier, port } => {
            let port_override = if let Some(port_str) = port {
                match pmdaemon::config::PortConfig::parse(&port_str) {
                    Ok(port_config) => {
                        info!("Using port override: {}", port_str);
                        Some(port_config)
                    }
                    Err(e) => {
                        error!("Invalid port format '{}': {}", port_str, e);
                        std::process::exit(1);
                    }
                }
            } else {
                None
            };

            manager.restart_with_port(&identifier, port_override).await?;
            println!("Restarted process: {}", identifier);
        }

        Commands::Reload { identifier, port } => {
            let port_override = if let Some(port_str) = port {
                match pmdaemon::config::PortConfig::parse(&port_str) {
                    Ok(port_config) => {
                        info!("Using port override: {}", port_str);
                        Some(port_config)
                    }
                    Err(e) => {
                        error!("Invalid port format '{}': {}", port_str, e);
                        std::process::exit(1);
                    }
                }
            } else {
                None
            };

            manager.reload_with_port(&identifier, port_override).await?;
            println!("Reloaded process: {}", identifier);
        }

        Commands::Delete { identifier, status, force } => {
            if identifier == "all" {
                // Delete all processes
                if !force {
                    print!("Are you sure you want to delete ALL processes? (y/N): ");
                    std::io::Write::flush(&mut std::io::stdout()).unwrap();
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    if !input.trim().to_lowercase().starts_with('y') {
                        println!("Deletion cancelled.");
                        return Ok(());
                    }
                }
                let deleted_count = manager.delete_all().await?;
                println!("Stopped and deleted {} processes", deleted_count);
            } else if status {
                // Delete by status
                if !force {
                    print!("Are you sure you want to delete all processes with status '{}'? (y/N): ", identifier);
                    std::io::Write::flush(&mut std::io::stdout()).unwrap();
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    if !input.trim().to_lowercase().starts_with('y') {
                        println!("Deletion cancelled.");
                        return Ok(());
                    }
                }
                let deleted_count = manager.delete_by_status(&identifier).await?;
                println!("Stopped and deleted {} processes with status '{}'", deleted_count, identifier);
            } else {
                // Delete single process by name/ID
                manager.delete(&identifier).await?;
                println!("Stopped and deleted process: {}", identifier);
            }
        }

        Commands::List => {
            let processes = manager.list().await?;

            if processes.is_empty() {
                println!("No processes running");
                return Ok(());
            }

            // Create table
            let mut table = Table::new();
            table
                .load_preset(comfy_table::presets::UTF8_FULL)
                .set_header(vec![
                    Cell::new("ID").add_attribute(Attribute::Bold),
                    Cell::new("Name").add_attribute(Attribute::Bold),
                    Cell::new("Status").add_attribute(Attribute::Bold),
                    Cell::new("PID").add_attribute(Attribute::Bold),
                    Cell::new("Uptime").add_attribute(Attribute::Bold),
                    Cell::new("↻").add_attribute(Attribute::Bold), // Restart symbol
                    Cell::new("CPU %").add_attribute(Attribute::Bold),
                    Cell::new("Memory").add_attribute(Attribute::Bold),
                    Cell::new("Port").add_attribute(Attribute::Bold),
                ]);

            // Add processes to table
            for process in processes {
                let uptime = process.uptime
                    .map(|t| format_duration(Utc::now() - t))
                    .unwrap_or_else(|| "-".to_string());

                let port_display = process.assigned_port
                    .map(|p| p.to_string())
                    .unwrap_or_else(|| "-".to_string());

                let memory_display = format_memory(process.memory_usage);

                // Color status cell
                let status_str = process.state.to_string();
                let status_cell = match status_str.as_str() {
                    "online" => Cell::new(&status_str).fg(Color::Green),
                    "stopped" => Cell::new(&status_str).fg(Color::Red),
                    "stopping" => Cell::new(&status_str).fg(Color::Yellow),
                    "starting" => Cell::new(&status_str).fg(Color::Blue),
                    _ => Cell::new(&status_str),
                };

                // Color CPU usage if high
                let cpu_cell = if process.cpu_usage > 80.0 {
                    Cell::new(format!("{:.1}", process.cpu_usage)).fg(Color::Red)
                } else if process.cpu_usage > 50.0 {
                    Cell::new(format!("{:.1}", process.cpu_usage)).fg(Color::Yellow)
                } else {
                    Cell::new(format!("{:.1}", process.cpu_usage))
                };

                table.add_row(vec![
                    Cell::new(&process.id.to_string()[..8]),
                    Cell::new(truncate_string(&process.name, 20)),
                    status_cell,
                    Cell::new(process.pid.map(|p| p.to_string()).unwrap_or_else(|| "-".to_string())),
                    Cell::new(uptime),
                    Cell::new(process.restarts),
                    cpu_cell,
                    Cell::new(memory_display),
                    Cell::new(port_display),
                ]);
            }

            println!("{}", table);
        }

        Commands::Monit { interval: interval_secs } => {
            use std::io::{self, Write};
            use tokio::time::{interval, Duration};
            use comfy_table::presets::UTF8_FULL;

            println!("Starting real-time monitoring with {}-second intervals... (Press Ctrl+C to exit)\n", interval_secs);

            let mut ticker = interval(Duration::from_secs(interval_secs));

            loop {
                // Clear screen
                print!("\x1B[2J\x1B[1;1H");
                io::stdout().flush()?;

                let system_info = manager.get_system_info().await?;
                let processes = manager.list().await?;

                // System header
                println!("┌─ PMDaemon Process Monitor ─ {} ─┐", Local::now().format("%Y-%m-%d %H:%M:%S"));
                println!("│ System CPU: {:>5.1}% │ Memory: {:>5.1}% ({:.1}GB/{:.1}GB) │ Load: {:.2} {:.2} {:.2} │",
                    system_info.cpu_usage,
                    system_info.memory_percent,
                    system_info.memory_used as f64 / 1024.0 / 1024.0 / 1024.0,
                    system_info.memory_total as f64 / 1024.0 / 1024.0 / 1024.0,
                    system_info.load_average[0],
                    system_info.load_average[1],
                    system_info.load_average[2]
                );
                println!("└{}┘", "─".repeat(80));

                if processes.is_empty() {
                    println!("\n📭 No processes running");
                } else {
                    // Create table
                    let mut table = Table::new();
                    table
                        .load_preset(UTF8_FULL)
                        .set_header(vec![
                            Cell::new("ID").add_attribute(Attribute::Bold),
                            Cell::new("Name").add_attribute(Attribute::Bold),
                            Cell::new("Status").add_attribute(Attribute::Bold),
                            Cell::new("PID").add_attribute(Attribute::Bold),
                            Cell::new("Uptime").add_attribute(Attribute::Bold),
                            Cell::new("↻").add_attribute(Attribute::Bold), // Restart symbol
                            Cell::new("CPU %").add_attribute(Attribute::Bold),
                            Cell::new("Memory").add_attribute(Attribute::Bold),
                            Cell::new("Port").add_attribute(Attribute::Bold),
                        ]);

                    for process in processes {
                        let uptime = process.uptime
                            .map(|t| format_duration(Utc::now() - t))
                            .unwrap_or_else(|| "-".to_string());

                        let port_display = process.assigned_port
                            .map(|p| p.to_string())
                            .unwrap_or_else(|| "-".to_string());

                        let memory_display = format_memory(process.memory_usage);

                        // Color status cell
                        let status_str = process.state.to_string();
                        let status_cell = match status_str.as_str() {
                            "online" => Cell::new(&status_str).fg(Color::Green),
                            "stopped" => Cell::new(&status_str).fg(Color::Red),
                            "stopping" => Cell::new(&status_str).fg(Color::Yellow),
                            "starting" => Cell::new(&status_str).fg(Color::Blue),
                            _ => Cell::new(&status_str),
                        };

                        // Color CPU usage if high
                        let cpu_cell = if process.cpu_usage > 80.0 {
                            Cell::new(format!("{:.1}", process.cpu_usage)).fg(Color::Red)
                        } else if process.cpu_usage > 50.0 {
                            Cell::new(format!("{:.1}", process.cpu_usage)).fg(Color::Yellow)
                        } else {
                            Cell::new(format!("{:.1}", process.cpu_usage))
                        };

                        table.add_row(vec![
                            Cell::new(&process.id.to_string()[..8]),
                            Cell::new(truncate_string(&process.name, 20)),
                            status_cell,
                            Cell::new(process.pid.map(|p| p.to_string()).unwrap_or_else(|| "-".to_string())),
                            Cell::new(uptime),
                            Cell::new(process.restarts),
                            cpu_cell,
                            Cell::new(memory_display),
                            Cell::new(port_display),
                        ]);
                    }

                    println!("{}", table);
                }

                println!("\n💡 Press Ctrl+C to exit");

                ticker.tick().await;
            }
        }

        Commands::Logs { identifier, lines, follow } => {
            if let Some(id) = identifier {
                manager.read_logs(&id, Some(lines), follow).await?;
            } else {
                println!("Please specify a process identifier");
                std::process::exit(1);
            }
        }

        Commands::Info { identifier } => {
            let info = manager.get_process_info(&identifier).await?;
            println!("{:#?}", info);
        }

        Commands::Web { port, host } => {
            println!("Starting web server on {}:{}", host, port);
            manager.start_web_server(&host, port).await?;
        }
    }

    Ok(())
}

/// Format duration in human-readable format
fn format_duration(duration: chrono::Duration) -> String {
    let total_seconds = duration.num_seconds();

    if total_seconds < 60 {
        format!("{}s", total_seconds)
    } else if total_seconds < 3600 {
        format!("{}m", total_seconds / 60)
    } else if total_seconds < 86400 {
        format!("{}h", total_seconds / 3600)
    } else {
        format!("{}d", total_seconds / 86400)
    }
}



/// Truncate strings to fit in table columns
fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len-3])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;
    use chrono::Duration;

    #[test]
    fn test_format_duration_seconds() {
        let duration = Duration::seconds(30);
        assert_eq!(format_duration(duration), "30s");
    }

    #[test]
    fn test_format_duration_minutes() {
        let duration = Duration::seconds(90); // 1.5 minutes
        assert_eq!(format_duration(duration), "1m");
    }

    #[test]
    fn test_format_duration_hours() {
        let duration = Duration::seconds(3661); // 1 hour, 1 minute, 1 second
        assert_eq!(format_duration(duration), "1h");
    }

    #[test]
    fn test_format_duration_days() {
        let duration = Duration::seconds(86401); // 1 day, 1 second
        assert_eq!(format_duration(duration), "1d");
    }

    #[test]
    fn test_format_duration_zero() {
        let duration = Duration::seconds(0);
        assert_eq!(format_duration(duration), "0s");
    }

    #[test]
    fn test_format_duration_edge_cases() {
        // Exactly 1 minute
        assert_eq!(format_duration(Duration::seconds(60)), "1m");
        // Exactly 1 hour
        assert_eq!(format_duration(Duration::seconds(3600)), "1h");
        // Exactly 1 day
        assert_eq!(format_duration(Duration::seconds(86400)), "1d");
    }

    #[test]
    fn test_truncate_string_no_truncation() {
        let input = "short";
        assert_eq!(truncate_string(input, 10), "short");
    }

    #[test]
    fn test_truncate_string_exact_length() {
        let input = "exactly10c";
        assert_eq!(truncate_string(input, 10), "exactly10c");
    }

    #[test]
    fn test_truncate_string_needs_truncation() {
        let input = "this is a very long string that needs truncation";
        assert_eq!(truncate_string(input, 10), "this is...");
    }

    #[test]
    fn test_truncate_string_minimum_length() {
        let input = "test";
        assert_eq!(truncate_string(input, 4), "test");

        let long_input = "testing";
        assert_eq!(truncate_string(long_input, 4), "t...");
    }

    #[test]
    fn test_truncate_string_empty() {
        assert_eq!(truncate_string("", 5), "");
    }

    #[test]
    fn test_cli_parsing_basic_commands() {
        // Test basic command parsing
        let cli = Cli::try_parse_from(&["pmdaemon", "list"]).unwrap();
        assert!(!cli.verbose);
        assert!(cli.config.is_none());
        assert!(matches!(cli.command, Commands::List));
    }

    #[test]
    fn test_cli_parsing_verbose_flag() {
        let cli = Cli::try_parse_from(&["pmdaemon", "--verbose", "list"]).unwrap();
        assert!(cli.verbose);
    }

    #[test]
    fn test_cli_parsing_config_flag() {
        let cli = Cli::try_parse_from(&["pmdaemon", "--config", "/path/to/config.json", "list"]).unwrap();
        assert_eq!(cli.config, Some(PathBuf::from("/path/to/config.json")));
    }

    #[test]
    fn test_cli_parsing_start_command_basic() {
        let cli = Cli::try_parse_from(&["pmdaemon", "start", "script.js"]).unwrap();
        if let Commands::Start { script, name, instances, cwd, env, max_memory, port, args } = cli.command {
            assert_eq!(script, Some("script.js".to_string()));
            assert_eq!(name, None);
            assert_eq!(instances, 1);
            assert_eq!(cwd, None);
            assert!(env.is_empty());
            assert_eq!(max_memory, None);
            assert_eq!(port, None);
            assert!(args.is_empty());
        } else {
            panic!("Expected Start command");
        }
    }

    #[test]
    fn test_cli_parsing_start_command_with_options() {
        let cli = Cli::try_parse_from(&[
            "pmdaemon", "start", "app.js",
            "--name", "my-app",
            "--instances", "3",
            "--cwd", "/app",
            "--env", "NODE_ENV=production",
            "--env", "PORT=3000",
            "--max-memory", "512M",
            "--port", "3000-3010",
            "--", "--arg1", "value1"
        ]).unwrap();

        if let Commands::Start { script, name, instances, cwd, env, max_memory, port, args } = cli.command {
            assert_eq!(script, Some("app.js".to_string()));
            assert_eq!(name, Some("my-app".to_string()));
            assert_eq!(instances, 3);
            assert_eq!(cwd, Some(PathBuf::from("/app")));
            assert_eq!(env, vec!["NODE_ENV=production", "PORT=3000"]);
            assert_eq!(max_memory, Some("512M".to_string()));
            assert_eq!(port, Some("3000-3010".to_string()));
            assert_eq!(args, vec!["--arg1", "value1"]);
        } else {
            panic!("Expected Start command");
        }
    }

    #[test]
    fn test_cli_parsing_stop_command() {
        let cli = Cli::try_parse_from(&["pmdaemon", "stop", "my-app"]).unwrap();
        if let Commands::Stop { identifier } = cli.command {
            assert_eq!(identifier, "my-app");
        } else {
            panic!("Expected Stop command");
        }
    }

    #[test]
    fn test_cli_parsing_restart_command() {
        let cli = Cli::try_parse_from(&["pmdaemon", "restart", "my-app", "--port", "4000"]).unwrap();
        if let Commands::Restart { identifier, port } = cli.command {
            assert_eq!(identifier, "my-app");
            assert_eq!(port, Some("4000".to_string()));
        } else {
            panic!("Expected Restart command");
        }
    }

    #[test]
    fn test_cli_parsing_reload_command() {
        let cli = Cli::try_parse_from(&["pmdaemon", "reload", "my-app"]).unwrap();
        if let Commands::Reload { identifier, port } = cli.command {
            assert_eq!(identifier, "my-app");
            assert_eq!(port, None);
        } else {
            panic!("Expected Reload command");
        }
    }

    #[test]
    fn test_cli_parsing_delete_command_basic() {
        let cli = Cli::try_parse_from(&["pmdaemon", "delete", "my-app"]).unwrap();
        if let Commands::Delete { identifier, status, force } = cli.command {
            assert_eq!(identifier, "my-app");
            assert!(!status);
            assert!(!force);
        } else {
            panic!("Expected Delete command");
        }
    }

    #[test]
    fn test_cli_parsing_delete_command_with_flags() {
        let cli = Cli::try_parse_from(&["pmdaemon", "delete", "stopped", "--status", "--force"]).unwrap();
        if let Commands::Delete { identifier, status, force } = cli.command {
            assert_eq!(identifier, "stopped");
            assert!(status);
            assert!(force);
        } else {
            panic!("Expected Delete command");
        }
    }

    #[test]
    fn test_cli_parsing_monit_command() {
        let cli = Cli::try_parse_from(&["pmdaemon", "monit", "--interval", "5"]).unwrap();
        if let Commands::Monit { interval } = cli.command {
            assert_eq!(interval, 5);
        } else {
            panic!("Expected Monit command");
        }
    }

    #[test]
    fn test_cli_parsing_monit_command_default() {
        let cli = Cli::try_parse_from(&["pmdaemon", "monit"]).unwrap();
        if let Commands::Monit { interval } = cli.command {
            assert_eq!(interval, 1); // Default value
        } else {
            panic!("Expected Monit command");
        }
    }

    #[test]
    fn test_cli_parsing_logs_command() {
        let cli = Cli::try_parse_from(&["pmdaemon", "logs", "my-app", "--lines", "50", "--follow"]).unwrap();
        if let Commands::Logs { identifier, lines, follow } = cli.command {
            assert_eq!(identifier, Some("my-app".to_string()));
            assert_eq!(lines, 50);
            assert!(follow);
        } else {
            panic!("Expected Logs command");
        }
    }

    #[test]
    fn test_cli_parsing_logs_command_defaults() {
        let cli = Cli::try_parse_from(&["pmdaemon", "logs"]).unwrap();
        if let Commands::Logs { identifier, lines, follow } = cli.command {
            assert_eq!(identifier, None);
            assert_eq!(lines, 20); // Default value
            assert!(!follow);
        } else {
            panic!("Expected Logs command");
        }
    }

    #[test]
    fn test_cli_parsing_info_command() {
        let cli = Cli::try_parse_from(&["pmdaemon", "info", "my-app"]).unwrap();
        if let Commands::Info { identifier } = cli.command {
            assert_eq!(identifier, "my-app");
        } else {
            panic!("Expected Info command");
        }
    }

    #[test]
    fn test_cli_parsing_web_command() {
        let cli = Cli::try_parse_from(&["pmdaemon", "web", "--port", "8080", "--host", "0.0.0.0"]).unwrap();
        if let Commands::Web { port, host } = cli.command {
            assert_eq!(port, 8080);
            assert_eq!(host, "0.0.0.0");
        } else {
            panic!("Expected Web command");
        }
    }

    #[test]
    fn test_cli_parsing_web_command_defaults() {
        let cli = Cli::try_parse_from(&["pmdaemon", "web"]).unwrap();
        if let Commands::Web { port, host } = cli.command {
            assert_eq!(port, pmdaemon::DEFAULT_WEB_PORT);
            assert_eq!(host, "127.0.0.1");
        } else {
            panic!("Expected Web command");
        }
    }

    #[test]
    fn test_cli_parsing_invalid_command() {
        let result = Cli::try_parse_from(&["pmdaemon", "invalid-command"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_cli_parsing_missing_required_args() {
        // Stop command requires identifier
        let result = Cli::try_parse_from(&["pmdaemon", "stop"]);
        assert!(result.is_err());

        // Info command requires identifier
        let result = Cli::try_parse_from(&["pmdaemon", "info"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_cli_parsing_help() {
        let result = Cli::try_parse_from(&["pmdaemon", "--help"]);
        assert!(result.is_err()); // Help exits with error code but shows help
    }

    #[test]
    fn test_cli_parsing_version() {
        let result = Cli::try_parse_from(&["pmdaemon", "--version"]);
        assert!(result.is_err()); // Version exits with error code but shows version
    }
}
