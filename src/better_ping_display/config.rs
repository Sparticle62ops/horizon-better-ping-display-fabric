//! Upstream `Config.java`, 1:1.
use std::path::Path;

use serde::{Deserialize, Serialize};

/// `java.awt.Color` as upstream stores it: ARGB packed in an int. Upstream's
/// Gson adapter serializes it as `#rrggbb` and parses `#rrggbb` back (alpha
/// reset to opaque), so the config round-trip carries the colour's 24 bits.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Color(pub u32);

impl Color {
    /// `new Color(int)` with the alpha as given.
    pub fn argb(argb: u32) -> Self {
        Color(argb)
    }

    /// `Color.getRed()`.
    pub fn get_red(&self) -> u32 {
        (self.0 >> 16) & 0xFF
    }

    /// `Color.getGreen()`.
    pub fn get_green(&self) -> u32 {
        (self.0 >> 8) & 0xFF
    }

    /// `Color.getBlue()`.
    pub fn get_blue(&self) -> u32 {
        self.0 & 0xFF
    }

    /// `Color.getRGB()`.
    pub fn get_rgb(&self) -> u32 {
        self.0
    }
}

/// Upstream `ColorJsonAdapter`: `#rrggbb` both ways.
mod color_json {
    use super::Color;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(color: &Color, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&format!(
            "#{:02x}{:02x}{:02x}",
            color.get_red(),
            color.get_green(),
            color.get_blue()
        ))
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Color, D::Error> {
        let str = String::deserialize(d)?;
        // `Integer.parseInt(str.substring(1), 16)` then `new Color(int)`:
        // alpha defaults opaque.
        let rgb = u32::from_str_radix(str.get(1..).unwrap_or(""), 16)
            .map_err(serde::de::Error::custom)?;
        Ok(Color(0xFF00_0000 | (rgb & 0xFFFFFF)))
    }
}

/// Upstream defaults.
pub const DEFAULT_PING_TEXT_COLOR: u32 = 0xFFA0A0A0;
pub const DEFAULT_PING_TEXT_FORMAT: &str = "%dms";

/// Upstream `Config.ConfigData`: the serialized shape, same field names.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigData {
    #[serde(rename = "autoColorPingText", default = "default_auto_color")]
    pub auto_color_ping_text: bool,
    #[serde(rename = "renderPingBars", default)]
    pub render_ping_bars: bool,
    #[serde(
        rename = "pingTextColor",
        default = "default_ping_text_color",
        with = "color_json"
    )]
    pub ping_text_color: Color,
    #[serde(
        rename = "pingTextFormatString",
        default = "default_ping_text_format"
    )]
    pub ping_text_format_string: String,
}

fn default_auto_color() -> bool {
    true
}

fn default_ping_text_color() -> Color {
    Color::argb(DEFAULT_PING_TEXT_COLOR)
}

fn default_ping_text_format() -> String {
    DEFAULT_PING_TEXT_FORMAT.to_string()
}

impl Default for ConfigData {
    fn default() -> Self {
        ConfigData {
            auto_color_ping_text: true,
            render_ping_bars: false,
            ping_text_color: Color::argb(DEFAULT_PING_TEXT_COLOR),
            ping_text_format_string: DEFAULT_PING_TEXT_FORMAT.to_string(),
        }
    }
}

/// Upstream `Config`: wraps `ConfigData`, refuses a text format without `%d`.
#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    data: ConfigData,
}

impl Config {
    fn new(config_data: ConfigData) -> Self {
        let mut config_data = config_data;
        if !config_data.ping_text_format_string.contains("%d") {
            config_data.ping_text_format_string = DEFAULT_PING_TEXT_FORMAT.to_string();
        }
        Config { data: config_data }
    }

    /// `Config.getTextColor()`.
    pub fn get_text_color(&self) -> Color {
        self.data.ping_text_color
    }

    /// `Config.setTextColor(Color)`.
    pub fn set_text_color(&mut self, color: Color) {
        self.data.ping_text_color = color;
    }

    /// `Config.getTextFormatString()`.
    pub fn get_text_format_string(&self) -> &str {
        &self.data.ping_text_format_string
    }

    /// `Config.setTextFormatString(String)`.
    pub fn set_text_format_string(&mut self, text_format_string: &str) {
        self.data.ping_text_format_string = text_format_string.to_string();
    }

    /// `Config.shouldAutoColorPingText()`.
    pub fn should_auto_color_ping_text(&self) -> bool {
        self.data.auto_color_ping_text
    }

    /// `Config.setShouldAutoColorPingText(boolean)`.
    pub fn set_should_auto_color_ping_text(&mut self, should_auto_color_ping_text: bool) {
        self.data.auto_color_ping_text = should_auto_color_ping_text;
    }

    /// `Config.shouldRenderPingBars()`.
    pub fn should_render_ping_bars(&self) -> bool {
        self.data.render_ping_bars
    }

    /// `Config.setShouldRenderPingBars(boolean)`.
    pub fn set_should_render_ping_bars(&mut self, should_render_ping_bars: bool) {
        self.data.render_ping_bars = should_render_ping_bars;
    }

    /// `Config.writeToFile(File)`: Gson pretty printing (two-space indent).
    pub fn write_to_file(&self, file: &Path) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(&self.data)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        std::fs::write(file, json)
    }

    /// `Config.fromDefault()`.
    pub fn from_default() -> Self {
        Config::new(ConfigData::default())
    }

    /// `Config.fromFile(File)`.
    pub fn from_file(file: &Path) -> std::io::Result<Self> {
        let text = std::fs::read_to_string(file)?;
        let data: ConfigData = serde_json::from_str(&text)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        Ok(Config::new(data))
    }
}
