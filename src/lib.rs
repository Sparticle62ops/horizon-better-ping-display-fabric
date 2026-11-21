//! better-ping-display (Horizon Rust port) — crate root.
//!
//! The 1:1 port lives in [`better_ping_display`]; this root carries the mod
//! API wiring: the mod's settings page, mirroring upstream's config fields as
//! Horizon settings with upstream's lang keys. Upstream's fourth field
//! (`pingTextFormatString`) has no Horizon setting kind (there is no string
//! setting), so it stays in the config file the mod keeps, validated the way
//! upstream validates it (it must contain `%d`).

use mod_api::{RegistrationContext, Registrar};
use mod_api::SettingSpec;

pub mod better_ping_display;

#[derive(Default)]
struct Mod;

#[mod_api::horizon_mod]
impl mod_api::Hooks for Mod {
    /// Upstream's config fields as the settings page: autoColorPingText,
    /// renderPingBars, pingTextColor, with upstream's label/description keys.
    fn register(&self, registrar: &mut dyn Registrar, _context: &RegistrationContext) {
        let _ = registrar.register_setting(
            SettingSpec::toggle(
                "autoColorPingText",
                "betterpingdisplay.settings.autoColorPingText",
                true,
            )
            .tooltip("betterpingdisplay.settings.autoColorPingText.description"),
        );
        let _ = registrar.register_setting(
            SettingSpec::toggle(
                "renderPingBars",
                "betterpingdisplay.settings.renderPingBars",
                false,
            )
            .tooltip("betterpingdisplay.settings.renderPingBars.description"),
        );
        let _ = registrar.register_setting(
            SettingSpec::color(
                "pingTextColor",
                "betterpingdisplay.settings.pingTextColor",
                0xA0A0A0,
            )
            .tooltip("betterpingdisplay.settings.pingTextColor.description"),
        );
    }
}
