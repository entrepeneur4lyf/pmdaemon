use pmdaemon::config::{EcosystemConfig, PortConfig};
use std::io::Write;
use tempfile::NamedTempFile;

#[tokio::test]
async fn test_json_config_format() {
    let json_content = r#"
{
  "apps": [
    {
      "name": "test-json-app",
      "script": "node",
      "args": ["server.js"],
      "instances": 2,
      "port": "3000-3001",
      "max_memory_restart": "512M",
      "env": {
        "NODE_ENV": "production",
        "PORT": "3000"
      },
      "autorestart": true,
      "max_restarts": 10,
      "namespace": "test"
    },
    {
      "name": "simple-app",
      "script": "echo",
      "args": ["hello"]
    }
  ]
}
"#;

    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(json_content.as_bytes()).unwrap();
    temp_file.flush().unwrap();

    let config = EcosystemConfig::from_file(temp_file.path()).await.unwrap();

    assert_eq!(config.apps.len(), 2);

    // Test first app (complex config)
    let app1 = &config.apps[0];
    assert_eq!(app1.name, "test-json-app");
    assert_eq!(app1.script, "node");
    assert_eq!(app1.args, vec!["server.js"]);
    assert_eq!(app1.instances, 2);
    assert_eq!(app1.port, Some(PortConfig::Range(3000, 3001)));
    assert_eq!(app1.max_memory_restart, Some(512 * 1024 * 1024)); // 512MB in bytes
    assert_eq!(app1.env.get("NODE_ENV"), Some(&"production".to_string()));
    assert_eq!(app1.env.get("PORT"), Some(&"3000".to_string()));
    assert!(app1.autorestart);
    assert_eq!(app1.max_restarts, 10);
    assert_eq!(app1.namespace, "test");

    // Test second app (minimal config with defaults)
    let app2 = &config.apps[1];
    assert_eq!(app2.name, "simple-app");
    assert_eq!(app2.script, "echo");
    assert_eq!(app2.args, vec!["hello"]);
    assert_eq!(app2.instances, 1); // default
    assert_eq!(app2.port, None); // default
    assert_eq!(app2.max_memory_restart, None); // default
    assert!(app2.autorestart); // default
    assert_eq!(app2.max_restarts, 16); // default
    assert_eq!(app2.namespace, "default"); // default
}

#[tokio::test]
async fn test_yaml_config_format() {
    let yaml_content = r#"
apps:
  - name: test-yaml-app
    script: python
    args:
      - "-m"
      - uvicorn
      - "main:app"
    instances: 3
    port: "auto:8000-8100"
    max_memory_restart: "1G"
    env:
      PYTHONPATH: /opt/app
      DATABASE_URL: postgres://localhost/db
    cwd: /opt/myapp
    min_uptime: 5000
    restart_delay: 1000
    kill_timeout: 3000

  - name: worker-yaml
    script: node
    args:
      - worker.js
    max_memory_restart: "256M"
    env:
      REDIS_URL: redis://localhost:6379
"#;

    let mut temp_file = NamedTempFile::with_suffix(".yaml").unwrap();
    temp_file.write_all(yaml_content.as_bytes()).unwrap();
    temp_file.flush().unwrap();

    let config = EcosystemConfig::from_file(temp_file.path()).await.unwrap();

    assert_eq!(config.apps.len(), 2);

    // Test first app (complex YAML config)
    let app1 = &config.apps[0];
    assert_eq!(app1.name, "test-yaml-app");
    assert_eq!(app1.script, "python");
    assert_eq!(app1.args, vec!["-m", "uvicorn", "main:app"]);
    assert_eq!(app1.instances, 3);
    assert_eq!(app1.port, Some(PortConfig::Auto(8000, 8100)));
    assert_eq!(app1.max_memory_restart, Some(1024 * 1024 * 1024)); // 1GB in bytes
    assert_eq!(app1.env.get("PYTHONPATH"), Some(&"/opt/app".to_string()));
    assert_eq!(
        app1.env.get("DATABASE_URL"),
        Some(&"postgres://localhost/db".to_string())
    );
    assert_eq!(app1.cwd, Some(std::path::PathBuf::from("/opt/myapp")));
    assert_eq!(app1.min_uptime, 5000);
    assert_eq!(app1.restart_delay, 1000);
    assert_eq!(app1.kill_timeout, 3000);

    // Test second app
    let app2 = &config.apps[1];
    assert_eq!(app2.name, "worker-yaml");
    assert_eq!(app2.script, "node");
    assert_eq!(app2.args, vec!["worker.js"]);
    assert_eq!(app2.max_memory_restart, Some(256 * 1024 * 1024)); // 256MB in bytes
    assert_eq!(
        app2.env.get("REDIS_URL"),
        Some(&"redis://localhost:6379".to_string())
    );
}

#[tokio::test]
async fn test_toml_config_format() {
    let toml_content = r#"
[[apps]]
name = "test-toml-app"
script = "cargo"
args = ["run", "--release"]
instances = 1
port = "9090"
max_memory_restart = "128M"
autorestart = true
max_restarts = 5
namespace = "rust-apps"

[apps.env]
RUST_LOG = "debug"
CARGO_TARGET_DIR = "/tmp/target"

[[apps]]
name = "static-toml"
script = "python"
args = ["-m", "http.server", "8080"]
port = "8080"
cwd = "/var/www"

[apps.env]
PYTHONUNBUFFERED = "1"
"#;

    let mut temp_file = NamedTempFile::with_suffix(".toml").unwrap();
    temp_file.write_all(toml_content.as_bytes()).unwrap();
    temp_file.flush().unwrap();

    let config = EcosystemConfig::from_file(temp_file.path()).await.unwrap();

    assert_eq!(config.apps.len(), 2);

    // Test first app (TOML config)
    let app1 = &config.apps[0];
    assert_eq!(app1.name, "test-toml-app");
    assert_eq!(app1.script, "cargo");
    assert_eq!(app1.args, vec!["run", "--release"]);
    assert_eq!(app1.instances, 1);
    assert_eq!(app1.port, Some(PortConfig::Single(9090)));
    assert_eq!(app1.max_memory_restart, Some(128 * 1024 * 1024)); // 128MB in bytes
    assert!(app1.autorestart);
    assert_eq!(app1.max_restarts, 5);
    assert_eq!(app1.namespace, "rust-apps");
    assert_eq!(app1.env.get("RUST_LOG"), Some(&"debug".to_string()));
    assert_eq!(
        app1.env.get("CARGO_TARGET_DIR"),
        Some(&"/tmp/target".to_string())
    );

    // Test second app
    let app2 = &config.apps[1];
    assert_eq!(app2.name, "static-toml");
    assert_eq!(app2.script, "python");
    assert_eq!(app2.args, vec!["-m", "http.server", "8080"]);
    assert_eq!(app2.port, Some(PortConfig::Single(8080)));
    assert_eq!(app2.cwd, Some(std::path::PathBuf::from("/var/www")));
    assert_eq!(app2.env.get("PYTHONUNBUFFERED"), Some(&"1".to_string()));
}

#[tokio::test]
async fn test_memory_format_parsing() {
    let json_content = r#"
{
  "apps": [
    {
      "name": "memory-test-k",
      "script": "test",
      "max_memory_restart": "512K"
    },
    {
      "name": "memory-test-m",
      "script": "test",
      "max_memory_restart": "256M"
    },
    {
      "name": "memory-test-g",
      "script": "test",
      "max_memory_restart": "2G"
    },
    {
      "name": "memory-test-bytes",
      "script": "test",
      "max_memory_restart": 1048576
    },
    {
      "name": "memory-test-none",
      "script": "test"
    }
  ]
}
"#;

    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(json_content.as_bytes()).unwrap();
    temp_file.flush().unwrap();

    let config = EcosystemConfig::from_file(temp_file.path()).await.unwrap();

    assert_eq!(config.apps.len(), 5);

    // Test different memory formats
    assert_eq!(config.apps[0].max_memory_restart, Some(512 * 1024)); // 512K
    assert_eq!(config.apps[1].max_memory_restart, Some(256 * 1024 * 1024)); // 256M
    assert_eq!(
        config.apps[2].max_memory_restart,
        Some(2 * 1024 * 1024 * 1024)
    ); // 2G
    assert_eq!(config.apps[3].max_memory_restart, Some(1048576)); // Raw bytes
    assert_eq!(config.apps[4].max_memory_restart, None); // No limit
}

#[tokio::test]
async fn test_port_format_parsing() {
    let yaml_content = r#"
apps:
  - name: single-port
    script: test
    port: "3000"

  - name: port-range
    script: test
    port: "4000-4003"

  - name: auto-port
    script: test
    port: "auto:5000-5100"

  - name: no-port
    script: test
"#;

    let mut temp_file = NamedTempFile::with_suffix(".yaml").unwrap();
    temp_file.write_all(yaml_content.as_bytes()).unwrap();
    temp_file.flush().unwrap();

    let config = EcosystemConfig::from_file(temp_file.path()).await.unwrap();

    assert_eq!(config.apps.len(), 4);

    // Test different port formats
    assert_eq!(config.apps[0].port, Some(PortConfig::Single(3000)));
    assert_eq!(config.apps[1].port, Some(PortConfig::Range(4000, 4003)));
    assert_eq!(config.apps[2].port, Some(PortConfig::Auto(5000, 5100)));
    assert_eq!(config.apps[3].port, None);
}

#[tokio::test]
async fn test_config_validation() {
    // Test duplicate app names
    let invalid_json = r#"
{
  "apps": [
    {
      "name": "duplicate",
      "script": "test1"
    },
    {
      "name": "duplicate",
      "script": "test2"
    }
  ]
}
"#;

    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(invalid_json.as_bytes()).unwrap();
    temp_file.flush().unwrap();

    let result = EcosystemConfig::from_file(temp_file.path()).await;
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Duplicate app name"));
}

#[tokio::test]
async fn test_config_get_app() {
    let json_content = r#"
{
  "apps": [
    {
      "name": "app1",
      "script": "test1"
    },
    {
      "name": "app2",
      "script": "test2"
    }
  ]
}
"#;

    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(json_content.as_bytes()).unwrap();
    temp_file.flush().unwrap();

    let config = EcosystemConfig::from_file(temp_file.path()).await.unwrap();

    // Test get_app method
    assert!(config.get_app("app1").is_some());
    assert_eq!(config.get_app("app1").unwrap().script, "test1");
    assert!(config.get_app("app2").is_some());
    assert_eq!(config.get_app("app2").unwrap().script, "test2");
    assert!(config.get_app("nonexistent").is_none());

    // Test app_names method
    let names = config.app_names();
    assert_eq!(names, vec!["app1", "app2"]);
}

#[tokio::test]
async fn test_file_extension_detection() {
    // Test that file extension determines parser
    let content = r#"{"apps": [{"name": "test", "script": "echo"}]}"#;

    // Test .json extension
    let mut json_file = NamedTempFile::with_suffix(".json").unwrap();
    json_file.write_all(content.as_bytes()).unwrap();
    json_file.flush().unwrap();
    assert!(EcosystemConfig::from_file(json_file.path()).await.is_ok());

    // Test unknown extension defaults to JSON
    let mut unknown_file = NamedTempFile::with_suffix(".unknown").unwrap();
    unknown_file.write_all(content.as_bytes()).unwrap();
    unknown_file.flush().unwrap();
    assert!(EcosystemConfig::from_file(unknown_file.path())
        .await
        .is_ok());
}
