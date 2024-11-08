mod sys_linux;

use serde_json;

fn struct_to_map(info: &sys_linux::LinuxSystemInfo) -> std::collections::HashMap<String, String> {
    let json_value = serde_json::to_value(info).expect("Failed to convert struct to JSON");

    if let serde_json::Value::Object(map) = json_value {
        map.into_iter()
            .filter_map(|(k, v)| match v {
                serde_json::Value::String(s) => Some((k, s.replace('\n', ""))),
                serde_json::Value::Number(n) => {
                    if n.is_f64() {
                        Some((k, format!("{:.2}", n.as_f64().unwrap())))
                    } else {
                        Some((k, format!("{}", n.as_i64().unwrap())))
                    }
                }
                _ => None,
            })
            .collect()
    } else {
        std::collections::HashMap::new()
    }
}
macro_rules! format_template {
    ($template:expr, $info:expr) => {{
        let mut output = $template.to_string();
        let mut values_map = struct_to_map(&$info);
        println!("{:?}", values_map);
        values_map.insert("reset".to_string(), "\x1b[0m".to_string());
        values_map.insert("bold".to_string(), "\x1b[1m".to_string());
        values_map.insert("dim".to_string(), "\x1b[2m".to_string());
        values_map.insert("italic".to_string(), "\x1b[3m".to_string());
        values_map.insert("underline".to_string(), "\x1b[4m".to_string());
        values_map.insert("blink".to_string(), "\x1b[5m".to_string());
        values_map.insert("reverse".to_string(), "\x1b[7m".to_string());
        values_map.insert("hidden".to_string(), "\x1b[8m".to_string());

        values_map.insert("black".to_string(), "\x1b[30m".to_string());
        values_map.insert("red".to_string(), "\x1b[31m".to_string());
        values_map.insert("green".to_string(), "\x1b[32m".to_string());
        values_map.insert("yellow".to_string(), "\x1b[33m".to_string());
        values_map.insert("blue".to_string(), "\x1b[34m".to_string());
        values_map.insert("magenta".to_string(), "\x1b[35m".to_string());
        values_map.insert("cyan".to_string(), "\x1b[36m".to_string());
        values_map.insert("white".to_string(), "\x1b[37m".to_string());

        values_map.insert("bg_black".to_string(), "\x1b[40m".to_string());
        values_map.insert("bg_red".to_string(), "\x1b[41m".to_string());
        values_map.insert("bg_green".to_string(), "\x1b[42m".to_string());
        values_map.insert("bg_yellow".to_string(), "\x1b[43m".to_string());
        values_map.insert("bg_blue".to_string(), "\x1b[44m".to_string());
        values_map.insert("bg_magenta".to_string(), "\x1b[45m".to_string());
        values_map.insert("bg_cyan".to_string(), "\x1b[46m".to_string());
        values_map.insert("bg_white".to_string(), "\x1b[47m".to_string());
        for (key, value) in &values_map {
            let placeholder = format!("{{{{{}}}}}", key);
            output = output.replace(&placeholder, value);
        }

        println!("{}", output);
    }};
}

static DEFAULT_CONFIG: &str = "
-> {{cyan}}{{username}}@{{hostname}}{{reset}}
{{green}}====================================={{reset}}
{{red}}{{bold}}OS{{reset}}: {{os_name}} ({{os_id}}){{os_version}}
{{red}}{{bold}}Kernel{{reset}}: {{kernel}} {{kernel_version}}
{{red}}{{bold}}Memory{{reset}}: {{used_memory}}/{{total_memory}} GiB ({{memory_usage_percent}}%)
";

fn display_linux() {
    let home_path = std::path::PathBuf::from(env!("HOME"));
    let valid_paths = [
        home_path.join(".config/synap/.configrc"),
        home_path.join(".config/.synaprc"),
        home_path.join(".synaprc"),
    ];
    let mut config_str = String::new();
    for path in valid_paths {
        println!("{:?}", path);
        if path.exists() && path.is_file() {
            config_str = std::fs::read_to_string(path).unwrap();
            break;
        }
    }
    let data = sys_linux::LinuxSystemInfo::new();
    format_template!(
        if !config_str.is_empty() {
            config_str
        } else {
            DEFAULT_CONFIG.to_string()
        },
        data
    );
}
fn display_windows() {}
fn main() {
    match std::env::consts::OS {
        "linux" => display_linux(),
        "windows" => display_windows(),
        _ => std::process::exit(0),
    }
}
