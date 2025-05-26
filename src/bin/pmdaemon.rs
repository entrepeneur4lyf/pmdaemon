//! PMDaemon CLI
//!
//! Command-line interface for the PMDaemon process manager.

use clap::{Parser, Subcommand};
use pmdaemon::{ProcessManager, ProcessConfig, Result};
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
        /// Script or command to execute
        script: String,

        /// Process name
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
        /// Process name or ID
        identifier: String,
    },

    /// List all processes
    List,

    /// Monitor processes in real-time
    Monit,

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
        .with_env_filter(format!("pmdaemon={},pmdaemon={}", log_level, log_level))
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
                match parse_memory_string(&memory_str) {
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
                if let Some((key, value)) = env_var.split_once('=') {
                    config_builder = config_builder.env(key, value);
                } else {
                    error!("Invalid environment variable format: {}", env_var);
                    std::process::exit(1);
                }
            }

            let config = config_builder.build()?;
            let process_id = manager.start(config).await?;

            println!("Started process with ID: {}", process_id);
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

        Commands::Delete { identifier } => {
            manager.delete(&identifier).await?;
            println!("Deleted process: {}", identifier);
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

        Commands::Monit => {
            use std::io::{self, Write};
            use tokio::time::{interval, Duration};
            use comfy_table::presets::UTF8_FULL;

            println!("Starting real-time monitoring... (Press Ctrl+C to exit)\n");

            let mut ticker = interval(Duration::from_secs(1));

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

/// Parse memory string (e.g., "100M", "1G", "512K") to bytes
fn parse_memory_string(memory_str: &str) -> Result<u64> {
    let memory_str = memory_str.trim().to_uppercase();

    if memory_str.is_empty() {
        return Err(pmdaemon::Error::config("Memory string cannot be empty"));
    }

    let (number_part, unit) = if memory_str.ends_with("K") || memory_str.ends_with("KB") {
        let number_str = memory_str.trim_end_matches("KB").trim_end_matches("K");
        (number_str, 1024u64)
    } else if memory_str.ends_with("M") || memory_str.ends_with("MB") {
        let number_str = memory_str.trim_end_matches("MB").trim_end_matches("M");
        (number_str, 1024u64 * 1024)
    } else if memory_str.ends_with("G") || memory_str.ends_with("GB") {
        let number_str = memory_str.trim_end_matches("GB").trim_end_matches("G");
        (number_str, 1024u64 * 1024 * 1024)
    } else if memory_str.ends_with("B") {
        let number_str = memory_str.trim_end_matches("B");
        (number_str, 1u64)
    } else {
        // Assume bytes if no unit specified
        (memory_str.as_str(), 1u64)
    };

    let number: f64 = number_part.parse()
        .map_err(|_| pmdaemon::Error::config(format!("Invalid memory number: {}", number_part)))?;

    if number < 0.0 {
        return Err(pmdaemon::Error::config("Memory size cannot be negative"));
    }

    Ok((number * unit as f64) as u64)
}

/// Format memory in human-readable format
fn format_memory(bytes: u64) -> String {
    if bytes == 0 {
        return "-".to_string();
    }

    let kb = bytes as f64 / 1024.0;
    let mb = kb / 1024.0;
    let gb = mb / 1024.0;

    if gb >= 1.0 {
        format!("{:.1}GB", gb)
    } else if mb >= 1.0 {
        format!("{:.1}MB", mb)
    } else if kb >= 1.0 {
        format!("{:.1}KB", kb)
    } else {
        format!("{}B", bytes)
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
