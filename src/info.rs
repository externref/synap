#[derive(Debug)]
pub struct MemoryInfo {
    ram_max: i32,
    ram_used: i32,

    swap_max: i32,
    swap_used: i32,
}

impl MemoryInfo {
    fn create() -> Self {
        let mem_data = std::fs::read_to_string("/proc/meminfo").unwrap();

        let parse_int = |x: &str, y: usize| {
            x.split_at(y)
                .1
                .strip_suffix(" kB")
                .unwrap()
                .trim()
                .parse::<i32>()
                .unwrap()
        };
        let mut mem_info = MemoryInfo {
            ram_max: 0,
            ram_used: 0,
            swap_max: 0,
            swap_used: 0,
        };
        for line in mem_data.lines() {
            if line.starts_with("MemTotal") {
                mem_info.ram_max = parse_int(line, 9);
            } else if line.starts_with("MemAvailable") {
                mem_info.ram_used = mem_info.ram_max - parse_int(line, 13);
            } else if line.starts_with("SwapTotal") {
                mem_info.swap_max = parse_int(line, 10);
            } else if line.starts_with("SwapFree") {
                mem_info.swap_used = mem_info.swap_max - parse_int(line, 9)
            }
        }
        mem_info
    }
}

impl Default for MemoryInfo {
    fn default() -> Self {
        Self::create()
    }
}

#[derive(Debug)]
pub struct SystemInfo {
    os_name: String,
    kernel_version: String,
    host_name: String,
    uptime: String,
    desktop_environment: String,
}

impl SystemInfo {
    fn whoami()-> String{
        let command = std::process::Command::new("whoami").output().unwrap();
        return std::str::from_utf8(&command.stdout).unwrap().to_string();
    }
    pub fn create() -> Self {
        let os_info = std::fs::read_to_string("/usr/lib/os-release").unwrap();
        let mut data: std::collections::HashMap<&str, &str> = std::collections::HashMap::new();
        for line in os_info.lines() {
            let row: Vec<&str> = line.split('=').collect();
            data.insert(row.get(0).unwrap(), row.get(1).unwrap());
        }
        let mut sysinfo = SystemInfo {
            os_name: String::default(),
            kernel_version: String::default(),
            host_name: String::default(),
            uptime: String::default(),
            desktop_environment: String::default(),
        };
        sysinfo.os_name = format!(
            "{} ({})",
            data.get("PRETTY_NAME").unwrap(),
            data.get("id").unwrap_or(&"-")
        );
        sysinfo.host_name = SystemInfo::whoami();
        sysinfo
    }
}

impl Default for SystemInfo {
    fn default() -> Self {
        Self::create()
    }
}
