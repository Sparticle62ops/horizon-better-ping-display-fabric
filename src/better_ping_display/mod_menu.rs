//! Upstream `integ/ModMenuIntegration.java`, 1:1.
//!
//! Mod Menu is a Fabric-side surface; Horizon's settings page takes its
//! place. `getModConfigScreenFactory` keeps upstream's logic: hand back the
//! YACL screen factory when `yet_another_config_lib_v3` is loaded, else log
//! upstream's line and hand back the null factory.
use super::config::Config;
use super::yacl_config_screen::{create, ConfigScreen};

/// `ModMenuIntegration.getModConfigScreenFactory()` and
/// `YaclConfigScreenFactory.create(parent)` joined at the screen the host
/// builds: `Some(spec)` when YACL is loaded, `None` for upstream's
/// `NullScreenFactory<>`.
pub fn get_mod_config_screen_factory(
    yet_another_config_lib_v3_loaded: bool,
    config: &Config,
) -> Option<ConfigScreen> {
    if yet_another_config_lib_v3_loaded {
        return Some(create(config));
    }

    // `LOGGER.info("YACL is not installed, GUI config will not be available")`
    super::better_ping_display_mod::log::log("YACL is not installed, GUI config will not be available");
    None
}
