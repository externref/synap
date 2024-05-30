use serde::{Deserialize, Serialize};
use serde_json;

fn get_true() -> bool {
    true
}
fn get_false() -> bool {
    false
}

#[derive(Deserialize, Debug, Serialize)]
pub struct InfoConfigs {
    #[serde(default = "get_true")]
    os_name: bool,
    #[serde(default = "get_true")]
    kernel: bool,
    #[serde(default = "get_true")]
    host: bool,
    #[serde(default = "get_true")]
    uptime: bool,
    #[serde(default = "get_true")]
    packages: bool,
    #[serde(default = "get_false")]
    display_size: bool,
    #[serde(default = "get_false")]
    desktop_environment: bool,
    #[serde(default = "get_false")]
    cpu_usage: bool,
    #[serde(default = "get_true")]
    memory_usage: bool,
    #[serde(default = "get_true")]
    disk_usage: bool,
    #[serde(default = "get_false")]
    battery: bool,
    #[serde(default = "get_true")]
    local_ip: bool,
}
fn get_config_str() -> Option<String> {
    let home = std::env::var("HOME").unwrap();
    let possible_paths = [
        &format!("{}/synap.json", home),
        &format!("{}/.config/synap/config.json", home),
        &format!("./config.json"),
    ];
    for path_str in possible_paths {
        let path = std::path::Path::new(path_str);
        if path.exists() && path.is_file() {
            return Some(std::fs::read_to_string(path).unwrap().to_string());
        }
    }
    return None;
}
impl InfoConfigs {
    fn new() -> Self {
        let binding = get_config_str().unwrap_or("{}".to_string());
        let file_content = binding.as_str();
        let configs: InfoConfigs = serde_json::from_str(file_content).unwrap();
        configs
    }
}

impl Default for InfoConfigs {
    fn default() -> Self {
        InfoConfigs::new()
    }
}
