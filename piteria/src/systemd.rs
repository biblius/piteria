use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemdConfig {
    /// Absolute path to the systemd service file.
    ///
    /// By default this should be in /etc/systemd/system/multi-user.target.wants
    pub file_location: String,

    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#%5BUnit%5D%20Section%20Options
    pub unit: SysdUnitConfig,

    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html#Options
    pub service: SysdServiceConfig,

    /// https://www.freedesktop.org/software/systemd/man/latest/systemd.unit.html#%5BInstall%5D%20Section%20Options
    pub install: SysdInstallConfig,
}

impl Display for SystemdConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.unit)?;
        writeln!(f, "{}", self.service)?;
        writeln!(f, "{}", self.install)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SysdUnitConfig {
    /// Parameters under the \[Unit\] directive.
    pub params: HashMap<String, String>,
}

impl Default for SysdUnitConfig {
    fn default() -> Self {
        Self {
            params: HashMap::from([(
                "Description".to_string(),
                "My super awesome application".to_string(),
            )]),
        }
    }
}

impl Display for SysdUnitConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[Unit]")?;
        for (key, value) in self.params.iter() {
            writeln!(f, "{key}={value}")?;
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SysdInstallConfig {
    /// Parameters under the \[Install\] directive.
    pub params: HashMap<String, String>,
}

impl Default for SysdInstallConfig {
    fn default() -> Self {
        Self {
            params: HashMap::from([("WantedBy".to_string(), "multi-user.target".to_string())]),
        }
    }
}

impl Display for SysdInstallConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[Install]")?;
        for (key, value) in self.params.iter() {
            writeln!(f, "{key}={value}")?;
        }
        Ok(())
    }
}
/// Configuration for systemd.
#[derive(Debug, Serialize, Deserialize)]
pub struct SysdServiceConfig {
    params: HashMap<String, String>,
    env: HashMap<String, String>,
}

impl Default for SysdServiceConfig {
    fn default() -> Self {
        Self {
            params: HashMap::from([
                ("ExecStart".to_string(), "echo 'Hello World'".to_string()),
                ("Restart".to_string(), RestartOption::Always.to_string()),
                ("User".to_string(), "root".to_string()),
                ("Group".to_string(), "root".to_string()),
                (
                    "WorkingDirectory".to_string(),
                    "/path/to/my-app".to_string(),
                ),
            ]),
            env: HashMap::from([("MyKey".to_string(), "MyValue".to_string())]),
        }
    }
}

impl Display for SysdServiceConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[Service]")?;
        for (key, value) in self.params.iter() {
            writeln!(f, "{key}={value}")?;
        }
        for (key, value) in self.env.iter() {
            writeln!(f, "Environment={key}={value}")?;
        }
        Ok(())
    }
}

/// Systemd service restart options
#[derive(Debug)]
pub enum RestartOption {
    No,
    OnSuccess,
    OnFailure,
    OnAbnormal,
    OnWatchdog,
    OnAbort,
    Always,
}

impl Display for RestartOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RestartOption::No => write!(f, "no"),
            RestartOption::OnSuccess => write!(f, "no"),
            RestartOption::OnFailure => write!(f, "on-failure"),
            RestartOption::OnAbnormal => write!(f, "on-abnormal"),
            RestartOption::OnWatchdog => write!(f, "on-watchdog"),
            RestartOption::OnAbort => write!(f, "on-abort"),
            RestartOption::Always => write!(f, "always"),
        }
    }
}
