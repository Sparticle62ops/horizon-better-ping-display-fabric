//! Upstream `mixin/PlayerListHudMixin.java` and
//! `mixin/PlayerListHudInvoker.java`, 1:1.
//!
//! The mixin's injections into the vanilla player list — the slot-width
//! constant bump (`13 -> 13 + 45`) and the redirect of the
//! `renderLatencyIcon` call to [`super::custom_player_list_hud`] — have no
//! Horizon mod API surface yet: the package declares the module and the
//! handler bodies, and the injection waits on the surface (see the README and
//! the tracked issue on horizon-rust/horizon).

use super::custom_player_list_hud::{render_ping_display, CustomPlayerListHudEnv, PlayerListEntry};

/// Upstream `@Unique @Final private static final int PLAYER_SLOT_EXTRA_WIDTH`.
pub const PLAYER_SLOT_EXTRA_WIDTH: i32 = 45;

/// `PlayerListHudMixin.modifySlotWidthConstant(int)`: the
/// `@ModifyConstant(method = "render", constant = @Constant(intValue = 13))`
/// handler.
pub fn modify_slot_width_constant(original: i32) -> i32 {
    original + PLAYER_SLOT_EXTRA_WIDTH
}

/// `PlayerListHudMixin.redirectRenderLatencyIconCall(...)`: the `@Redirect`
/// handler body, which is the whole call to
/// [`super::custom_player_list_hud::render_ping_display`].
pub fn redirect_render_latency_icon_call(
    env: &mut dyn CustomPlayerListHudEnv,
    config: &super::config::Config,
    width: i32,
    x: i32,
    y: i32,
    entry: &PlayerListEntry,
) {
    // `CustomPlayerListHud.renderPingDisplay(client, instance, context,
    // width, x, y, entry)`
    render_ping_display(env, config, width, x, y, entry)
}

/// Upstream `PlayerListHudInvoker`: the `@Invoker("renderLatencyIcon")`
/// accessor, as the trait the render seam implements for the real HUD.
pub trait PlayerListHudInvoker {
    /// `invokeRenderLatencyIcon(DrawContext, int, int, int, PlayerListEntry)`.
    fn invoke_render_latency_icon(&mut self, width: i32, x: i32, y: i32, entry: &PlayerListEntry);
}
