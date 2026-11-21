//! Upstream `hud/ColorUtil.java`, 1:1.
use super::ping_colors::{COLOR_END, COLOR_MID, COLOR_START};

/// `ColorUtil.interpolate(int, int, float)`. Upstream throws
/// `IllegalArgumentException` for an offset outside 0..1; the port panics with
/// the same message.
pub fn interpolate(color_start: u32, color_end: u32, offset: f32) -> u32 {
    if !(0.0..=1.0).contains(&offset) {
        panic!("Offset must be between 0.0 and 1.0");
    }

    let red_diff = get_red(color_end) as i32 - get_red(color_start) as i32;
    let green_diff = get_green(color_end) as i32 - get_green(color_start) as i32;
    let blue_diff = get_blue(color_end) as i32 - get_blue(color_start) as i32;

    // `Math.round(float)` on non-negative values: floor(x + 0.5).
    let new_red = round(get_red(color_start) as i32 as f32 + (red_diff as f32 * offset));
    let new_green = round(get_green(color_start) as i32 as f32 + (green_diff as f32 * offset));
    let new_blue = round(get_blue(color_start) as i32 as f32 + (blue_diff as f32 * offset));

    ((new_red << 16) | (new_green << 8) | new_blue) as u32
}

/// `java.lang.Math.round(float)`: `floor(x + 0.5)`.
fn round(x: f32) -> u32 {
    (x + 0.5).floor() as u32
}

/// `ColorUtil.getRed(int)` (`@VisibleForTesting`, exercised by upstream's own
/// unit tests, which the port carries).
pub fn get_red(color: u32) -> u32 {
    (color >> 16) & 0xFF
}

/// `ColorUtil.getGreen(int)`.
pub fn get_green(color: u32) -> u32 {
    (color >> 8) & 0xFF
}

/// `ColorUtil.getBlue(int)`.
pub fn get_blue(color: u32) -> u32 {
    color & 0xFF
}

/// Upstream `hud/ColorUtilTest.java`, 1:1.
#[cfg(test)]
mod color_util_test {
    use super::*;

    /// `testExtractComponents`.
    #[test]
    fn test_extract_components() {
        let color = 0xD6C130;
        assert_eq!(0xD6, get_red(color));
        assert_eq!(0xC1, get_green(color));
        assert_eq!(0x30, get_blue(color));
    }

    /// `testInterpolation_zero`.
    #[test]
    fn test_interpolation_zero() {
        let color = interpolate(COLOR_START, COLOR_MID, 0.0);
        assert_eq!(COLOR_START, color);
    }

    /// `testInterpolation_middle`.
    #[test]
    fn test_interpolation_middle() {
        let color = interpolate(COLOR_START, COLOR_MID, 0.5);
        assert_eq!(0x6b, get_red(color));
        assert_eq!(0xda, get_green(color));
        assert_eq!(0x53, get_blue(color));
    }

    /// `testInterpolation_one`.
    #[test]
    fn test_interpolation_one() {
        let color = interpolate(COLOR_START, COLOR_MID, 1.0);
        assert_eq!(COLOR_MID, color);
    }

    /// `testInterpolation_invalidOffset`.
    #[test]
    #[should_panic(expected = "Offset must be between 0.0 and 1.0")]
    fn test_interpolation_invalid_offset() {
        let _ = interpolate(0, 1, -0.1);
    }

    #[test]
    #[should_panic(expected = "Offset must be between 0.0 and 1.0")]
    fn test_interpolation_invalid_offset_high() {
        let _ = interpolate(0, 1, 1.1);
    }
}
