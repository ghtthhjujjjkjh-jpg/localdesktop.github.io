use serde::{Deserialize, Serialize};
use std::{
    fs::{self, OpenOptions},
    io::Write,
};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(not(test))]
pub const ARCH_FS_ROOT: &str = "/data/data/app.polarbear/files/arch";
#[cfg(test)]
pub const ARCH_FS_ROOT: &str = "/data/local/tmp/arch";

pub const ARCH_FS_ARCHIVE: &str = "https://github.com/termux/proot-distro/releases/download/v4.29.0/archlinux-aarch64-pd-v4.29.0.tar.xz";

pub const DOCS_HOME_URL: &str = "https://localdesktop.github.io/";

pub fn user_manual_url() -> String {
    format!(
        "https://github.com/localdesktop/localdesktop.github.io/releases/download/v{VERSION}/Local-Desktop-v{VERSION}-User-Manual.pdf"
    )
}

pub const WAYLAND_SOCKET_NAME: &str = "wayland-0";
pub const MAX_PANEL_LOG_ENTRIES: usize = 100;
pub const SENTRY_DSN: &str = "https://d8af27f864ade027ff81ecadea91b02e@o4509548388417536.ingest.de.sentry.io/4509548392480848";
pub const PIPEWIRE_GUEST_RUNTIME_DIR: &str = "/tmp";
pub const PULSE_GUEST_SERVER: &str = "unix:/tmp/pulse/native";
pub const CONFIG_FILE: &str = "/etc/localdesktop/localdesktop.toml";

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct LocalConfig {
    #[serde(default)]
    pub user: UserConfig,
    #[serde(default)]
    pub command: CommandConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserConfig {
    pub username: String,
}

impl Default for UserConfig {
    fn default() -> Self {
        Self {
            username: "root".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommandConfig {
    #[serde(default = "default_check")]
    pub check: String,
    #[serde(default = "default_install")]
    pub install: String,
    #[serde(default = "default_launch")]
    pub launch: String,
}

fn default_check() -> String {
    // KDE Plasma + Gaming + Translators verification
    "pacman -Qg plasma && pacman -Q xdg-desktop-portal-kde && pacman -Q vulkan-tools && pacman -Q wine-mono && pacman -Q fish && pacman -Q bat && pacman -Q exa"
        .to_string()
}

fn default_install() -> String {
    // Gaming-optimized installation for Snapdragon 7 Gen 3 tablet
    "stdbuf -oL pacman -Syu plasma xdg-desktop-portal-kde xdg-desktop-portal vulkan-tools vulkan-icd-loader wine-mono wine-gecko wine-staging winetricks flatpak fish bat exa ripgrep fd fzf tmux helix python-pip pipewire pipewire-pulse pipewire-alsa wireplumber pavucontrol --noconfirm --noprogressbar"
        .to_string()
}

fn default_launch() -> String {
    // KDE Plasma Wayland optimized for Snapdragon 7 Gen 3 (Adreno 720) with Vulkan
    format!("export PIPEWIRE_RUNTIME_DIR={PIPEWIRE_GUEST_RUNTIME_DIR} PULSE_SERVER={PULSE_GUEST_SERVER}; export QT_QPA_PLATFORM=wayland QT_QPA_PLATFORMTHEME=kde KWIN_COMPOSE=O2 GALLIUM_DRIVER=zink VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/adreno.json MESA_SHADER_CACHE_DIR=/tmp/mesa-cache; mkdir -p /tmp/mesa-cache; XDG_RUNTIME_DIR=/tmp WAYLAND_DISPLAY=wayland-0 /usr/lib/plasma-dbus-run-session-if-needed startplasma-wayland 2>&1")
        .to_string()
}

impl Default for CommandConfig {
    fn default() -> Self {
        Self {
            check: default_check(),
            install: default_install(),
            launch: default_launch(),
        }
    }
}

fn process_config_file(full_config_path: String) -> Vec<String> {
    let mut write_back_lines: Vec<String> = vec![];
    let mut effective_config: Vec<String> = vec![];

    if let Ok(content) = fs::read_to_string(&full_config_path) {
        for line in content.lines() {
            let trimmed = line.trim();

            if let Some((key, value)) = trimmed.split_once('=') {
                let key = key.trim();
                let value = value.trim();

                if key.starts_with("try_") {
                    write_back_lines.push(format!("# {}", trimmed));
                    let actual_key = key.trim_start_matches("try_");
                    if let Some(line_index) = effective_config
                        .iter()
                        .position(|line| line.starts_with(&format!("{}=", actual_key)))
                    {
                        effective_config[line_index] = format!("{}={}", actual_key, value);
                    } else {
                        effective_config.push(format!("{}={}", actual_key, value));
                    }
                } else {
                    write_back_lines.push(trimmed.to_string());
                    if !effective_config
                        .iter()
                        .any(|line| line.starts_with(&format!("{}=", key)))
                    {
                        effective_config.push(format!("{}={}", key, value));
                    }
                }
            } else {
                write_back_lines.push(trimmed.to_string());
                effective_config.push(trimmed.to_string());
            }
        }

        let _ = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&full_config_path)
            .and_then(|mut file| {
                for line in &write_back_lines {
                    writeln!(file, "{}", line)?
                }
                Ok(())
            });
    }

    effective_config
}

pub fn parse_config(full_config_path: String) -> LocalConfig {
    let lines = process_config_file(full_config_path);
    let content = lines.join("\n");
    if let Ok(config) = toml::from_str::<LocalConfig>(&content) {
        return config;
    }
    LocalConfig::default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    fn with_config_file(content: &str, f: impl Fn(String)) -> () {
        let dir = tempdir().unwrap();
        let base_dir = dir.path().to_str().unwrap();
        let path = format!("{}/etc/localdesktop", base_dir);
        fs::create_dir_all(&path).unwrap();
        let file_path = format!("{}/localdesktop.toml", path);
        fs::write(&file_path, content).unwrap();
        f(file_path)
    }

    #[test]
    fn should_handle_configs_without_try() {
        with_config_file(
            r#"
                [user]
                username = "alice"

                [command]
                check = "check-cmd"
                install = "install-cmd"
                launch = "launch-cmd"
            "#,
            |full_config_path| {
                let config = parse_config(full_config_path);
                assert_eq!(config.user.username, "alice");
                assert_eq!(config.command.check, "check-cmd");
                assert_eq!(config.command.install, "install-cmd");
                assert_eq!(config.command.launch, "launch-cmd");
            },
        );
    }

    #[test]
    fn should_handle_configs_with_try() {
        with_config_file(
            r#"
                [user]
                username = "root"
                try_username = "testuser"

                [command]
                check = "check-cmd"
                try_check = "try-check"
                install = "install-cmd"
                launch = "launch-cmd"
            "#,
            |full_config_path| {
                let config = parse_config(full_config_path);
                assert_eq!(config.user.username, "testuser");
                assert_eq!(config.command.check, "try-check");
                assert_eq!(config.command.install, "install-cmd")
            },
        );
    }

    #[test]
    fn should_comment_out_try_configs() {
        with_config_file(
            r#"
                username = "root"
                try_username = "commented"

                check = "normal"
                try_check = "try"
            "#,
            |full_config_path| {
                let _ = parse_config(full_config_path.clone());
                let content = fs::read_to_string(full_config_path).unwrap();
                assert!(content.contains("# try_username = \"commented\""));
                assert!(content.contains("# try_check = \"try\""));
            },
        );
    }
}
