use serde;
use std::fmt::Display;

struct PackageCounts {
    dpkg: Option<usize>,
    pacman: Option<usize>,
    snap: Option<usize>,
    flatpak: Option<usize>,
    apt: Option<usize>,
    dnf: Option<usize>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct LinuxSystemInfo {
    hostname: String,
    username: String,
    os_name: String,
    os_version: String,
    os_id: String,
    os_id_like: String,
    os_pretty_name: String,
    kernel: String,
    kernel_version: String,
    system_architecture: String,

    desktop_environment: String,
    gtk_theme: String,
    qt_theme: String,
    shell: String,
    terminal: String,

    packages: i32,
    apt_packages: i32,
    dpkg_packages: i32,
    pacman_packages: i32,
    dnf_packages: i32,
    snap_packages: i32,
    flatpak_packages: i32,

    total_memory_kb: i64,
    total_memory_mb: i32,
    total_memory: f32,
    used_memory_kb: i64,
    used_memory_mb: i32,
    used_memory: f32,
    available_memory_kb: i64,
    available_memory_mb: i32,
    available_memory: f32,
    memory_usage_percent: f32,
}

impl Display for LinuxSystemInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())
    }
}

impl LinuxSystemInfo {
    fn run_in_shell(cmd: &str) -> String {
        let command = std::process::Command::new(cmd).output().unwrap();
        return std::str::from_utf8(&command.stdout).unwrap().to_string();
    }

    fn get_de() -> String {
        let desktop_env_vars = vec![
            "XDG_CURRENT_DESKTOP",
            "DESKTOP_SESSION",
            "GNOME_DESKTOP_SESSION_ID",
            "KDE_FULL_SESSION",
        ];

        for var in &desktop_env_vars {
            if let Ok(value) = std::env::var(var) {
                return value;
            }
        }
        "Unknown".to_string()
    }

    pub fn get_gtk_theme() -> String {
        if let Ok(output) = std::process::Command::new("gsettings")
            .arg("get")
            .arg("org.gnome.desktop.interface")
            .arg("gtk-theme")
            .output()
        {
            if output.status.success() {
                let theme = String::from_utf8_lossy(&output.stdout);
                return theme.trim().replace("'", "");
            }
        }
        let gtk3_path = format!("{}/.config/gtk-3.0/settings.ini", env!("HOME"));
        if let Ok(content) = std::fs::read_to_string(&gtk3_path) {
            for line in content.lines() {
                if line.contains("gtk-theme-name") {
                    if let Some(theme) = line.split('=').nth(1) {
                        return theme.trim().to_string();
                    }
                }
            }
        }
        let gtk2_path = format!("{}/.gtkrc-2.0", env!("HOME"));
        if let Ok(content) = std::fs::read_to_string(&gtk2_path) {
            for line in content.lines() {
                if line.contains("gtk-theme-name") {
                    if let Some(theme) = line.split('=').nth(1) {
                        return theme.trim().to_string();
                    }
                }
            }
        }

        "Unknown".to_string()
    }

    fn get_qt_theme() -> String {
        let qt5ct_path = format!("{}/.config/qt5ct/qt5ct.conf", env!("HOME"));
        if let Ok(content) = std::fs::read_to_string(&qt5ct_path) {
            for line in content.lines() {
                if line.contains("style=") {
                    if let Some(theme) = line.split('=').nth(1) {
                        return theme.trim().to_string();
                    }
                }
            }
        }

        let kdeglobals_path =
            std::path::PathBuf::from(format!("{}/.config/kdeglobals", env!("HOME")));
        if let Ok(content) = std::fs::read_to_string(&kdeglobals_path) {
            for line in content.lines() {
                if line.contains("widgetStyle=") {
                    if let Some(theme) = line.split('=').nth(1) {
                        return theme.trim().to_string();
                    }
                }
            }
        }

        "Unknown".to_string()
    }

    fn get_package_count(command: &str, args: &[&str], keyword: &str) -> Option<usize> {
        if let Ok(output) = std::process::Command::new(command).args(args).output() {
            if output.status.success() {
                let output_str = std::str::from_utf8(&output.stdout).ok()?;
                return Some(output_str.matches(keyword).count());
            }
        }
        None
    }

    fn get_packages() -> PackageCounts {
        let dpkg = LinuxSystemInfo::get_package_count("dpkg-query", &["-f", ".", "-W"], ".");
        let pacman = LinuxSystemInfo::get_package_count("pacman", &["-Q"], "\n");
        let snap = LinuxSystemInfo::get_package_count("snap", &["list"], "\n")
            .map(|count| count.saturating_sub(1));
        let flatpak = LinuxSystemInfo::get_package_count("flatpak", &["list"], "\n");
        let apt = LinuxSystemInfo::get_package_count("apt", &["list", "--installed"], "\n")
            .map(|count| count.saturating_sub(1));
        let dnf = LinuxSystemInfo::get_package_count("dnf", &["list", "installed"], "\n")
            .map(|count| count.saturating_sub(1));

        PackageCounts {
            dpkg,
            pacman,
            snap,
            flatpak,
            apt,
            dnf,
        }
    }
    fn get_terminal_name() -> String {
        let mut pid = std::process::id();

        let known_terminals = [
            "gnome-terminal",
            "xterm",
            "konsole",
            "alacritty",
            "tilix",
            "urxvt",
            "terminator",
            "xfce4-terminal",
            "kitty",
            "lxterminal",
            "st",
            "mate-terminal",
            "deepin-terminal",
        ];

        loop {
            let comm_path = std::path::Path::new("/proc")
                .join(pid.to_string())
                .join("comm");

            if let Ok(content) = std::fs::read_to_string(&comm_path) {
                let process_name = content.trim().to_string();

                if known_terminals.contains(&process_name.as_str()) {
                    return process_name;
                }

                // Move to the parent process by reading /proc/[pid]/stat
                let stat_path = std::path::Path::new("/proc")
                    .join(pid.to_string())
                    .join("stat");
                if let Ok(stat_content) = std::fs::read_to_string(&stat_path) {
                    let fields: Vec<&str> = stat_content.split_whitespace().collect();
                    if let Some(ppid_str) = fields.get(3) {
                        if let Ok(ppid) = ppid_str.parse::<u32>() {
                            pid = ppid;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        "Unknown".to_string()
    }

    pub fn new() -> Self {
        let hostname = LinuxSystemInfo::run_in_shell("hostname");
        let username = LinuxSystemInfo::run_in_shell("whoami");
        let kernel = LinuxSystemInfo::run_in_shell("uname");
        let cmd = std::process::Command::new("uname")
            .arg("-r")
            .output()
            .unwrap();
        let kernel_version = std::str::from_utf8(&cmd.stdout).unwrap().to_string();
        let os_info = std::fs::read_to_string("/usr/lib/os-release").unwrap();
        let meminfo = std::fs::read_to_string("/proc/meminfo").unwrap();
        let mut data: std::collections::HashMap<&str, &str> = std::collections::HashMap::new();
        for line in os_info.lines() {
            let row: Vec<&str> = line.split('=').collect();
            data.insert(row.get(0).unwrap(), row.get(1).unwrap().trim_matches('"'));
        }
        for line in meminfo.lines() {
            let row: Vec<&str> = line.split(':').collect();
            let required_opts = [
                "MemTotal",
                "MemFree",
                "MemAvailable",
                "SwapTotal",
                "SwapFree",
            ];
            if required_opts.contains(row.get(0).unwrap()) {
                data.insert(
                    row.get(0).unwrap(),
                    row.get(1)
                        .unwrap()
                        .trim()
                        .split(" ")
                        .collect::<Vec<&str>>()
                        .get(0)
                        .unwrap(),
                );
            }
        }
        let total_memory_kb: i64 = data.get("MemTotal").unwrap().to_string().parse().unwrap();
        let available_memory_kb: i64 = data
            .get("MemAvailable")
            .unwrap()
            .to_string()
            .parse()
            .unwrap();
        let used_memory_kb: i64 = total_memory_kb - available_memory_kb;

        let (gtk_theme, qt_theme) = (
            LinuxSystemInfo::get_gtk_theme(),
            LinuxSystemInfo::get_qt_theme(),
        );
        let packagesc = LinuxSystemInfo::get_packages();
        let dnf_packages = packagesc.dnf.unwrap_or(0) as i32;
        let apt_packages = packagesc.apt.unwrap_or(0) as i32;
        let pacman_packages = packagesc.pacman.unwrap_or(0) as i32;
        let dpkg_packages = packagesc.dpkg.unwrap_or(0) as i32;
        let snap_packages = packagesc.snap.unwrap_or(0) as i32;
        let flatpak_packages = packagesc.flatpak.unwrap_or(0) as i32;
        let packages = dnf_packages
            + apt_packages
            + pacman_packages
            + dpkg_packages
            + snap_packages
            + flatpak_packages;

        LinuxSystemInfo {
            hostname,
            username,
            kernel,
            kernel_version,
            total_memory_kb,
            used_memory_kb,
            available_memory_kb,
            gtk_theme,
            qt_theme,
            packages,
            apt_packages,
            dnf_packages,
            dpkg_packages,
            pacman_packages,
            snap_packages,
            flatpak_packages,
            shell: std::env::var("SHELL")
                .unwrap_or("Unknown".to_string())
                .to_string(),
            terminal: LinuxSystemInfo::get_terminal_name(),
            system_architecture: std::env::consts::ARCH.to_string(),
            os_name: data.get("NAME").unwrap_or(&"").to_string(),
            os_pretty_name: data.get("PRETTY_NAME").unwrap_or(&"").to_string(),
            os_version: data.get("VERSION").unwrap_or(&"").to_string(),
            os_id: data.get("ID").unwrap_or(&"").to_string(),
            os_id_like: data.get("ID_LIKE").unwrap_or(&"").to_string(),
            desktop_environment: LinuxSystemInfo::get_de(),
            total_memory_mb: total_memory_kb as i32 / 1024,
            used_memory_mb: used_memory_kb as i32 / 1024,
            available_memory_mb: available_memory_kb as i32 / 1024,
            total_memory: total_memory_kb as f32 / 1024.0 / 1024.0,
            used_memory: used_memory_kb as f32 / 1024.0 / 1024.0,
            available_memory: available_memory_kb as f32 / 1024.0 / 1024.0,
            memory_usage_percent: (used_memory_kb as f32 / total_memory_kb as f32) * 100.0,
        }
    }
}
