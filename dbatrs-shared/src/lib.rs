use serde::{Deserialize, Serialize};
use config::{Config, File, FileFormat, ConfigError};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Default)]
pub struct PortalConf {
    pub telnet: String
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Default)]
pub struct TotalConf {
    pub portal: PortalConf

}

// A global that will be set *once*

impl TotalConf {
    pub fn set(mode: &str) -> Result<Self, ConfigError> {
        Config::builder()
            .add_source(File::new("config.default.toml", FileFormat::Toml).required(true))
            .add_source(File::new(&format!("config.{}.toml", mode), FileFormat::Toml).required(true))
            .add_source(File::new("config.user.toml", FileFormat::Toml).required(false))
            .add_source(File::new(&format!("config.user.{}.toml", mode), FileFormat::Toml).required(false))
            .build()?.try_deserialize()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Color {
    NoColor = 0,
    Standard = 1,
    Xterm256 = 2,
    TrueColor = 3
}

impl Default for Color {
    fn default() -> Self {
        Color::NoColor
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct ProtocolCapabilities {
    pub encryption: bool,
    pub client_name: String,
    pub client_version: String,
    pub host_address: String,
    pub host_port: u16,
    pub host_names: Vec<String>,
    pub encoding: String,
    pub utf8: bool,
    pub ansi_color: Color,
    pub width: u16,
    pub height: u16,
    pub gmcp: bool,
    pub msdp: bool,
    pub mssp: bool,
    pub mccp2: bool,
    pub mccp3: bool,
    pub ttype: bool,
    pub naws: bool,
    pub sga: bool,
    pub linemode: bool,
    pub force_endline: bool,
    pub oob: bool,
    pub tls: bool,
    pub screen_reader: bool,
    pub mouse_tracking: bool,
    pub vt100: bool,
    pub osc_color_palette: bool,
    pub proxy: bool,
    pub mnes: bool,
}

impl ProtocolCapabilities {
    pub fn with_custom_defaults() -> Self {
        Self {
            width: 78,
            height: 24,
            client_name: "UNKNOWN".to_string(),
            client_version: "UNKNOWN".to_string(),
            host_address: "UNKNOWN".to_string(),
            // For all other fields, use the derived defaults.
            ..Default::default()
        }
    }
}