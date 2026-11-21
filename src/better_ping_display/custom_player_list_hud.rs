//! Upstream `hud/CustomPlayerListHud.java`, 1:1. The Minecraft client,
//! `DrawContext` and `PlayerListHudInvoker` surfaces the render touches live
//! on the [`CustomPlayerListHudEnv`] seam; the body below is upstream's
//! `renderPingDisplay` verbatim.
use super::config::Config;
use super::ping_colors::get_color;

pub const PING_TEXT_RENDER_OFFSET: i32 = -13;
pub const PING_BARS_WIDTH: i32 = 11;

/// `net.minecraft.client.network.PlayerListEntry#getLatency()`.
pub struct PlayerListEntry {
    pub latency: i32,
}

/// The client surfaces `renderPingDisplay` reads and writes. In upstream the
/// parameters are the live `MinecraftClient`, the `PlayerListHud` cast to its
/// invoker mixin and the frame's `DrawContext`; the engine seam supplies them.
pub trait CustomPlayerListHudEnv {
    /// `client.textRenderer.getWidth(String)`.
    fn text_width(&self, text: &str) -> i32;
    /// `context.drawTextWithShadow(textRenderer, String, int, int, int)`.
    fn draw_text_with_shadow(&mut self, text: &str, x: i32, y: i32, color: u32);
    /// `((PlayerListHudInvoker) hud).invokeRenderLatencyIcon(...)` and the
    /// `PlayerListHud#render` call frame the mixin redirected.
    fn invoke_render_latency_icon(&mut self, width: i32, x: i32, y: i32, player: &PlayerListEntry);
}

/// `CustomPlayerListHud.renderPingDisplay(client, hud, context, width, x, y,
/// player)`.
pub fn render_ping_display(
    env: &mut dyn CustomPlayerListHudEnv,
    config: &Config,
    width: i32,
    x: i32,
    y: i32,
    player: &PlayerListEntry,
) {
    let ping_string = format_ping(config.get_text_format_string(), player.latency);
    let ping_string_width = env.text_width(&ping_string);
    let ping_text_color = if config.should_auto_color_ping_text() {
        get_color(player.latency)
    } else {
        config.get_text_color().get_rgb()
    };
    let mut text_x = width + x - ping_string_width + PING_TEXT_RENDER_OFFSET;

    if !config.should_render_ping_bars() {
        text_x += PING_BARS_WIDTH;
    }

    // Draw the ping text for the given player
    env.draw_text_with_shadow(&ping_string, text_x, y, ping_text_color);

    if config.should_render_ping_bars() {
        env.invoke_render_latency_icon(width, x, y, player);
    }
}

/// Java's `String.format(format, latency)` over upstream's format string. The
/// settings screen only accepts formats containing `%d`, so the `%d`
/// substitution covers the accepted domain.
fn format_ping(format: &str, latency: i32) -> String {
    format.replacen("%d", &latency.to_string(), 1)
}
