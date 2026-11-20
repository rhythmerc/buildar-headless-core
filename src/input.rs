//! The controllers ([`engine_core::TouchState`]) as a game pad's axes and buttons: the number formats pads use, and a
//! trigger read as a button or as how far it is pulled.

/// A stick's axis (-1..1) as a signed byte (-127..127, 0 centred).
pub fn axis_i8(v: f32) -> i8 {
    (v.clamp(-1.0, 1.0) * 127.0).round() as i8
}

/// A stick's axis (-1..1) as an unsigned byte (0..255, 128 centred), `inverted` for a pad whose axis grows the other
/// way (a stick's up as 0).
pub fn axis_u8(v: f32, inverted: bool) -> u8 {
    let v = if inverted { -v } else { v };
    (128.0 + v.clamp(-1.0, 1.0) * 127.0).round().clamp(0.0, 255.0) as u8
}

/// Whether a stick's axis is pushed past `dead` towards its negative and positive ends (a d-pad's two directions).
pub fn axis_pushed(v: f32, dead: f32) -> (bool, bool) {
    (v < -dead, v > dead)
}

/// A trigger (0..1) read as a pull: 0 inside its dead zone, 1 from `full` on, in proportion between.
pub fn pull(trigger: f32, dead: f32, full: f32) -> f32 {
    if trigger <= dead {
        return 0.0;
    }
    ((trigger - dead) / (full - dead).max(f32::EPSILON)).clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn axes_in_a_pads_formats() {
        assert_eq!((axis_i8(-1.0), axis_i8(0.25), axis_i8(0.0), axis_i8(3.0)), (-127, 32, 0, 127));
        assert_eq!((axis_u8(0.0, false), axis_u8(1.0, false), axis_u8(-1.0, false)), (128, 255, 1));
        assert_eq!(axis_u8(1.0, true), 1, "up on a stick whose y grows downwards");
        assert_eq!(axis_pushed(-0.6, 0.5), (true, false));
        assert_eq!(axis_pushed(0.3, 0.5), (false, false));
    }

    #[test]
    fn a_trigger_as_a_pull() {
        assert_eq!(pull(0.04, 0.05, 0.95), 0.0);
        assert!((pull(0.5, 0.05, 0.95) - 0.5).abs() < 1e-6);
        assert_eq!(pull(0.97, 0.05, 0.95), 1.0);
    }
}
