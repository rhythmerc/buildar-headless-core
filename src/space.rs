//! The runtime's world (its stage, the player's place and focus: metres, y up, right-handed) in a game's own space -
//! its units and its axes - and back. A rigid map, scaled: the world's axes turned onto the game's (a rotation, never
//! a mirror, so that the game's left is still the player's left), a world point at a game point.
use engine_core::Stage;
use engine_core::glam::{Mat3, Mat4, Vec3};

/// Where the runtime's world is in a game's space.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GameSpace {
    /// Game units a world metre.
    pub units: f32,
    /// The world's axes as the game's (a rotation): a world direction `d` is `axes * d` in the game.
    pub axes: Mat3,
    /// The world point at the game's `game_origin`.
    pub world_origin: Vec3,
    pub game_origin: Vec3,
}

/// A game's axes as the world's: the same (y up).
pub const Y_UP: Mat3 = Mat3::IDENTITY;
/// A game's axes with y down and z the other way (the world turned half round its x): a game whose y grows
/// downwards, as many consoles' do.
pub const Y_DOWN: Mat3 = Mat3::from_cols(Vec3::X, Vec3::NEG_Y, Vec3::NEG_Z);

impl GameSpace {
    /// `units` game units a metre, the world's axes as `axes`, the world's origin at the game's.
    pub fn new(units: f32, axes: Mat3) -> Self {
        Self { units, axes, world_origin: Vec3::ZERO, game_origin: Vec3::ZERO }
    }

    /// The same, with `world` at the game's `game`.
    pub fn at(self, world: Vec3, game: Vec3) -> Self {
        Self { world_origin: world, game_origin: game, ..self }
    }

    pub fn point_to_game(&self, p: Vec3) -> Vec3 {
        self.game_origin + self.axes * ((p - self.world_origin) * self.units)
    }

    pub fn point_to_world(&self, g: Vec3) -> Vec3 {
        self.world_origin + self.axes.transpose() * (g - self.game_origin) / self.units
    }

    /// A direction (no scale).
    pub fn direction_to_game(&self, d: Vec3) -> Vec3 {
        self.axes * d
    }

    pub fn direction_to_world(&self, d: Vec3) -> Vec3 {
        self.axes.transpose() * d
    }

    /// The world from the game's space: a matrix taking a game point (or a model made in game units, posed by the
    /// game) into the world.
    pub fn world_from_game(&self) -> Mat4 {
        let turn = Mat4::from_mat3(self.axes.transpose() / self.units);
        Mat4::from_translation(self.world_origin) * turn * Mat4::from_translation(-self.game_origin)
    }

    /// A frame the game gives (its axes and position, in its space) in the world.
    pub fn frame_from_game(&self, frame: Mat4) -> Mat4 {
        self.world_from_game() * frame
    }

    /// The stage's triangles (room and placed objects) in the game's space, each wound as it was.
    pub fn triangles(&self, stage: &Stage) -> Vec<[Vec3; 3]> {
        stage.triangles().map(|t| t.map(|p| self.point_to_game(p))).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn near(a: Vec3, b: Vec3) -> bool {
        (a - b).length() < 1e-3
    }

    #[test]
    fn y_down_is_a_turn_not_a_mirror() {
        assert!((Y_DOWN.determinant() - 1.0).abs() < 1e-6);
        let s = GameSpace::new(100.0, Y_DOWN);
        assert!(near(s.point_to_game(Vec3::new(1.0, 2.0, 3.0)), Vec3::new(100.0, -200.0, -300.0)));
    }

    #[test]
    fn points_go_and_come_back() {
        let s = GameSpace::new(4000.0, Y_DOWN).at(Vec3::new(1.0, 0.0, -2.0), Vec3::new(500.0, 0.0, 70.0));
        for p in [Vec3::ZERO, Vec3::new(0.3, 1.2, -4.0), Vec3::new(-7.0, 0.1, 2.0)] {
            assert!(near(s.point_to_world(s.point_to_game(p)), p));
            assert!(near(s.world_from_game().transform_point3(s.point_to_game(p)), p));
        }
        assert!(near(s.point_to_game(Vec3::new(1.0, 0.0, -2.0)), Vec3::new(500.0, 0.0, 70.0)));
    }

    #[test]
    fn a_frame_keeps_its_handedness() {
        let s = GameSpace::new(10.0, Y_DOWN);
        let frame = Mat4::from_translation(Vec3::new(10.0, -20.0, 0.0));
        let world = s.frame_from_game(frame);
        assert!(near(world.transform_point3(Vec3::ZERO), Vec3::new(1.0, 2.0, 0.0)));
        assert!(world.determinant() > 0.0);
    }

    #[test]
    fn a_stage_in_the_games_space() {
        let stage = Stage { revision: 1, room_quads: vec![[Vec3::ZERO, Vec3::X, Vec3::new(1.0, 0.0, 1.0), Vec3::Z]], ..Default::default() };
        let tris = GameSpace::new(2.0, Y_DOWN).triangles(&stage);
        assert_eq!(tris.len(), 2);
        assert!(near(tris[0][2], Vec3::new(2.0, 0.0, -2.0)));
    }
}
