use crate::{model, offset};
use std::f32::consts::PI;

pub fn build_ptr(base: u32, offset: u32) -> *const u32 {
    (base + offset) as *const u32
}

pub fn build_entity_base_ptr(entity_list_base_ptr: *const u32, offset: u32) -> *const u32 {
    unsafe {
        let entity_list_base_ptr_deref = *entity_list_base_ptr;
        build_ptr(entity_list_base_ptr_deref, offset)
    }
}

pub fn read_memory<T>(base_ptr: *const u32, offset: u32) -> T
where
    T: Copy,
{
    unsafe {
        let base_ptr_deref = *base_ptr;
        let data_ptr = (base_ptr_deref + offset) as *const T;
        *data_ptr
    }
}

pub fn write_memory<T>(base_ptr: *const u32, offset: u32, value: T) {
    unsafe {
        let base_ptr_deref = *base_ptr;
        let data_ptr = (base_ptr_deref + offset) as *mut T;
        *data_ptr = value
    }
}

pub fn read_player_count(module_base_addr: u32) -> u32 {
    let player_count_ptr = build_ptr(module_base_addr, offset::PLAYER_COUNT);
    unsafe { *player_count_ptr }
}

pub fn calculate_angle(local_player: &model::Entity, target_entity: &model::Entity) -> model::Vec2 {
    let delta_x = target_entity.head_position.x - local_player.head_position.x;
    let delta_y = target_entity.head_position.y - local_player.head_position.y;

    let x = delta_y.atan2(delta_x) * 180.0 / PI + 90.0;

    let delta_z = target_entity.head_position.z - local_player.head_position.z;

    let dist = ((local_player.head_position.x - target_entity.head_position.x).powi(2)
        + (local_player.head_position.y - target_entity.head_position.y).powi(2))
    .sqrt();

    let y = delta_z.atan2(dist) * 180.0 / PI;

    model::Vec2 { x, y }
}

pub fn aim(local_player: &model::Entity, target_view_angle: &model::Vec2) {
    write_memory::<f32>(
        local_player.base_ptr,
        offset::VIEW_ANGLE_X,
        target_view_angle.x,
    );

    write_memory::<f32>(
        local_player.base_ptr,
        offset::VIEW_ANGLE_Y,
        target_view_angle.y,
    );
}
