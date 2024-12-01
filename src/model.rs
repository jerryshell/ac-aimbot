use crate::{offset, util};

#[derive(Default)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct Entity {
    pub base_ptr: *const u32,
    pub head_position: Vec3,
    pub health: i32,
    pub distance_to_player: f32,
}

impl Entity {
    pub fn new(base_ptr: *const u32) -> Entity {
        let head_position = Vec3 {
            x: util::read_memory::<f32>(base_ptr, offset::HEAD_POSITION_X),
            y: util::read_memory::<f32>(base_ptr, offset::HEAD_POSITION_Y),
            z: util::read_memory::<f32>(base_ptr, offset::HEAD_POSITION_Z),
        };

        let health = util::read_memory::<i32>(base_ptr, offset::HEALTH);

        Entity {
            base_ptr,
            head_position,
            health,
            distance_to_player: 0.0,
        }
    }

    pub fn update_distance_to_player(&mut self, player_head_position: &Vec3) {
        self.distance_to_player = ((self.head_position.x - player_head_position.x).powi(2)
            + (self.head_position.y - player_head_position.y).powi(2)
            + (self.head_position.z - player_head_position.z).powi(2))
        .sqrt();
    }
}
