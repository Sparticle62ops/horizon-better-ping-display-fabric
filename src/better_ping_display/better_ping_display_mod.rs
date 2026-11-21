//! Upstream `BetterPingDisplayMod.java`, 1:1.
use std::path::{Path, PathBuf};

use super::config::Config;

pub const MOD_ID: &str = "betterpingdisplay";
pub const CONFIG_FILE_NAME: &str = "betterpingdisplay.json";

/// The logger upstream gets from Log4j. The port's host seam logs these
/// lines; the strings are upstream's verbatim.
pub struct Logger;

impl Logger {
    pub fn error(&self, message: &str, _ex: &std::io::Error) {
        log::log(&format!("{message}: {_ex}"));
    }
    pub fn warn(&self, message: &str) {
        log::log(message);
    }
    pub fn info(&self, message: &str) {
        log::log(message);
    }
}

/// The mod's lifecycle, upstream's `BetterPingDisplayMod` instance state.
///
/// Upstream keeps the instance in a static and exposes it through
/// `instance()`; the port's lifecycle runs from the host splice, which keeps
/// the state.
pub struct BetterPingDisplayMod {
    config_file_path: Option<PathBuf>,
    config: Config,
}

impl Default for BetterPingDisplayMod {
    fn default() -> Self {
        BetterPingDisplayMod {
            config_file_path: None,
            config: Config::from_default(),
        }
    }
}

impl BetterPingDisplayMod {
    /// `onInitialize()`: load the config file when it exists (and re-write it
    /// to add new fields), else write the default, with upstream's log lines.
    pub fn on_initialize(&mut self, config_dir: &Path) {
        self.config_file_path = Some(config_dir.join(CONFIG_FILE_NAME));
        let config_file = self.config_file_path.as_ref().unwrap().clone();
        let logger = Logger;
        if config_file.exists() {
            match Config::from_file(&config_file) {
                Ok(config) => {
                    self.config = config;
                    if let Err(ex) = self.config.write_to_file(&config_file) {
                        logger.error("Failed to load config file", &ex);
                    }
                }
                Err(ex) => {
                    logger.error("Failed to load config file", &ex);
                }
            }
        } else {
            logger.warn("Could not find config file, creating a default one");
            if let Err(ex) = self.config.write_to_file(&config_file) {
                logger.error("Failed to write default config file", &ex);
            }
        }

        logger.info("BetterPingDisplay mod loaded");
    }

    /// `getConfig()`.
    pub fn get_config(&self) -> &Config {
        &self.config
    }

    /// `getConfigFilePath()`.
    pub fn get_config_file_path(&self) -> Option<&Path> {
        self.config_file_path.as_deref()
    }
}

/// The host-side log sink (`LogManager.getLogger(MODID)`).
pub mod log {
    use std::sync::Mutex;

    static SINK: Mutex<Vec<String>> = Mutex::new(Vec::new());

    pub fn log(message: &str) {
        if let Ok(mut sink) = SINK.lock() {
            sink.push(message.to_string());
        }
    }

    pub fn take() -> Vec<String> {
        SINK.lock().map(|mut s| std::mem::take(&mut *s)).unwrap_or_default()
    }
}
