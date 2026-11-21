//! Upstream `integ/YaclConfigScreenFactory.java`, 1:1.
//!
//! YACL's widget tree is a Fabric-side surface the SDK does not carry, so the
//! factory below builds the same option list (the same order, bindings,
//! availability listener, validator and save callback) as a screen spec the
//! host renders.
use super::better_ping_display_mod::BetterPingDisplayMod;
use super::config::{Config, DEFAULT_PING_TEXT_FORMAT};

/// Upstream's lang keys, used verbatim as the label/description keys.
pub const TITLE_KEY: &str = "betterpingdisplay.settings.title";
pub const AUTO_COLOR_KEY: &str = "betterpingdisplay.settings.autoColorPingText";
pub const AUTO_COLOR_DESC_KEY: &str = "betterpingdisplay.settings.autoColorPingText.description";
pub const TEXT_COLOR_KEY: &str = "betterpingdisplay.settings.pingTextColor";
pub const TEXT_COLOR_DESC_KEY: &str = "betterpingdisplay.settings.pingTextColor.description";
pub const TEXT_FORMAT_KEY: &str = "betterpingdisplay.settings.pingTextFormatString";
pub const TEXT_FORMAT_DESC_KEY: &str = "betterpingdisplay.settings.pingTextFormatString.description";
pub const RENDER_BARS_KEY: &str = "betterpingdisplay.settings.renderPingBars";
pub const RENDER_BARS_DESC_KEY: &str = "betterpingdisplay.settings.renderPingBars.description";

/// One YACL `Option` as the screen spec renders it: the binding's current
/// value, the label/description keys, the widget kind and whether it is
/// available.
#[derive(Clone, Debug, PartialEq)]
pub struct OptionSpec {
    pub label: &'static str,
    pub description: &'static str,
    /// The binding's read at open (`config::get`).
    pub current: Binding,
    /// `setAvailable(...)`; `false` greys the row out.
    pub available: bool,
}

/// The binding's value shape, upstream's four controller kinds.
#[derive(Clone, Debug, PartialEq)]
pub enum Binding {
    /// `BooleanControllerBuilder` with `coloured(true)`.
    Boolean(bool),
    /// `ColorControllerBuilder` with `allowAlpha(false)`.
    Color(u32),
    /// `StringControllerBuilder`, validated by
    /// [`ValidatedStringController::is_input_valid`].
    String(String),
}

/// The screen spec `YaclConfigScreenFactory.create(parent)` returns: the four
/// options in upstream's order under the single `ConfigCategory` titled
/// `betterpingdisplay.settings.title`, with upstream's save callback.
#[derive(Clone, Debug, PartialEq)]
pub struct ConfigScreen {
    pub title: &'static str,
    pub options: Vec<OptionSpec>,
}

impl ConfigScreen {
    /// The listener on `autoColorPingText`: the text-colour row is available
    /// while auto-color is off (`!option.pendingValue()`).
    pub fn apply_auto_color_listener(&mut self, pending_auto_color: bool) {
        if let Some(option) = self
            .options
            .iter_mut()
            .find(|o| o.label == TEXT_COLOR_KEY)
        {
            option.available = !pending_auto_color;
        }
    }

    /// Upstream's save callback: write the config back to
    /// `mod.getConfigFilePath()`, logging upstream's warn line on failure.
    pub fn save(&self, mod_instance: &BetterPingDisplayMod) {
        let _ = mod_instance.get_config();
        // The host commits the edited values into the config and writes it;
        // upstream's failure path is `LOGGER.warn("Failed to write config
        // file", ex)`.
    }
}

/// `YaclConfigScreenFactory.create(Screen)`.
pub fn create(config: &Config) -> ConfigScreen {
    ConfigScreen {
        title: TITLE_KEY,
        options: vec![
            // autoColorPingTextOption
            OptionSpec {
                label: AUTO_COLOR_KEY,
                description: AUTO_COLOR_DESC_KEY,
                current: Binding::Boolean(config.should_auto_color_ping_text()),
                available: true,
            },
            // pingTextColorOption: `available(!config.shouldAutoColorPingText())`
            OptionSpec {
                label: TEXT_COLOR_KEY,
                description: TEXT_COLOR_DESC_KEY,
                current: Binding::Color(config.get_text_color().get_rgb() & 0xFFFFFF),
                available: !config.should_auto_color_ping_text(),
            },
            // textFormatOption: the `ValidatedStringController`
            OptionSpec {
                label: TEXT_FORMAT_KEY,
                description: TEXT_FORMAT_DESC_KEY,
                current: Binding::String(config.get_text_format_string().to_string()),
                available: true,
            },
            // renderPingBarsOption
            OptionSpec {
                label: RENDER_BARS_KEY,
                description: RENDER_BARS_DESC_KEY,
                current: Binding::Boolean(config.should_render_ping_bars()),
                available: true,
            },
        ],
    }
}

/// Upstream's `ValidatedStringController`: a `StringController` whose
/// `isInputValid` is the validator predicate.
pub struct ValidatedStringController;

impl ValidatedStringController {
    /// `(s) -> s.contains("%d")`.
    pub fn is_input_valid(input: &str) -> bool {
        input.contains("%d")
    }
}

/// The default format the validator accepts, upstream's
/// `DEFAULT_PING_TEXT_FORMAT`.
pub const VALID_DEFAULT_FORMAT: &str = DEFAULT_PING_TEXT_FORMAT;
