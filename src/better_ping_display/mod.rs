//! better-ping-display (Horizon Rust port), 1:1.
//!
//! Upstream: https://github.com/vladmarica/better-ping-display-fabric (MIT).
//! Copyright (c) 2018 Vlad Marica. Translation by sparticle; the port carries
//! the upstream MIT license and credit (LICENSE).
//!
//! Module map (upstream -> here):
//!   BetterPingDisplayMod.java        -> better_ping_display_mod
//!   Config.java                      -> config
//!   hud/ColorUtil.java               -> color_util
//!   hud/PingColors.java              -> ping_colors
//!   hud/CustomPlayerListHud.java     -> custom_player_list_hud
//!   mixin/PlayerListHudMixin.java    -> mixin_seams
//!   mixin/PlayerListHudInvoker.java  -> mixin_seams
//!   integ/YaclConfigScreenFactory.java -> yacl_config_screen
//!   integ/ModMenuIntegration.java    -> mod_menu
//!   (upstream's ModMenuIntegration/YACL depend on the Fabric Mod Menu and
//!   YACL libs; those screens map to Horizon's settings page below)
//!
//! Host seams for Minecraft types live in the module-local env traits. The
//! one engine seam: the vanilla player-list render injection (`PlayerListHud.
//! render` slot width + latency-icon call) has no Horizon mod API surface yet,
//! so the numeric ping has nothing to attach to; see the README and the
//! tracked issue on horizon-rust/horizon.

pub mod better_ping_display_mod;
pub mod color_util;
pub mod config;
pub mod custom_player_list_hud;
pub mod mixin_seams;
pub mod mod_menu;
pub mod ping_colors;
pub mod yacl_config_screen;
