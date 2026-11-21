//! Upstream `hud/PingColors.java`, 1:1.
use super::color_util::interpolate;

pub const PING_START: i32 = 0;
pub const PING_MID: i32 = 150;
pub const PING_END: i32 = 300;

pub const ALPHA_MASK: u32 = 0xFF00_0000;

pub const COLOR_GREY: u32 = 0x53_5353;
pub const COLOR_START: u32 = 0x00_E676;
pub const COLOR_MID: u32 = 0xD6_CD30;
pub const COLOR_END: u32 = 0xE5_3935;

/// `PingColors.getColor(int)`.
pub fn get_color(ping: i32) -> u32 {
    get_color_internal(ping) | ALPHA_MASK
}

fn get_color_internal(ping: i32) -> u32 {
    if ping < PING_START {
        return COLOR_GREY;
    }

    if ping < PING_MID {
        return interpolate(
            COLOR_START,
            COLOR_MID,
            compute_offset(PING_START, PING_MID, ping),
        );
    }

    interpolate(
        COLOR_MID,
        COLOR_END,
        compute_offset(PING_MID, PING_END, std::cmp::min(ping, PING_END)),
    )
}

/// `computeOffset(int, int, int)` with `MathHelper.clamp` inlined.
fn compute_offset(start: i32, end: i32, value: i32) -> f32 {
    let offset = (value - start) as f32 / (end - start) as f32;
    offset.clamp(0.0, 1.0)
}
