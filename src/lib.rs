mod model;
mod offset;
mod util;

use anyhow::Result;
use std::{
    thread,
    time::{Duration, Instant},
};
use windows::{
    core::s,
    Win32::{
        Foundation::HINSTANCE,
        System::{LibraryLoader::GetModuleHandleA, SystemServices::DLL_PROCESS_ATTACH},
        UI::Input::KeyboardAndMouse::GetAsyncKeyState,
    },
};

const FRAME_RATE: u64 = 60;

const VKEY_F: i32 = 0x46;

fn run() -> Result<()> {
    let module_base_addr = unsafe { GetModuleHandleA(s!("ac_client.exe")).map(|h| h.0 as u32) }?;
    let local_player_base_ptr = util::build_ptr(module_base_addr, offset::LOCAL_PLAYER);
    let entity_list_base_ptr = util::build_ptr(module_base_addr, offset::ENTITY_LIST);

    let mut aimbot_enable_flag = false;

    let tick_rate = Duration::from_millis(1000 / FRAME_RATE);
    let mut last_tick = Instant::now();

    loop {
        unsafe {
            if GetAsyncKeyState(VKEY_F) & 0x1 == 1 {
                aimbot_enable_flag = !aimbot_enable_flag;
            }
        };

        if aimbot_enable_flag {
            sleep(&tick_rate, &last_tick);
            continue;
        }

        let player_count = util::read_player_count(module_base_addr);

        let local_player = model::Entity::new(local_player_base_ptr);

        let mut entity_list = (1..player_count)
            .filter_map(|i| {
                let entity_base_ptr = util::build_entity_base_ptr(entity_list_base_ptr, i * 0x4);

                let mut entity = model::Entity::new(entity_base_ptr);

                if entity.health <= 0 {
                    return None;
                }

                entity.update_distance_to_player(&local_player.head_position);

                Some(entity)
            })
            .collect::<Vec<model::Entity>>();

        if entity_list.is_empty() {
            sleep(&tick_rate, &last_tick);
            continue;
        }

        entity_list.sort_by(|a, b| a.distance_to_player.total_cmp(&b.distance_to_player));

        let target_entity = entity_list.first().unwrap();

        let angle = util::calculate_angle(&local_player, target_entity);

        util::aim(&local_player, &angle);

        sleep(&tick_rate, &last_tick);
        last_tick = Instant::now();
    }
}

fn sleep(tick_rate: &Duration, last_tick: &Instant) {
    let timeout = tick_rate.saturating_sub(last_tick.elapsed());
    thread::sleep(timeout);
}

#[no_mangle]
extern "system" fn DllMain(_dll_module: HINSTANCE, call_reason: u32, _reserved: *mut ()) -> bool {
    if call_reason == DLL_PROCESS_ATTACH {
        thread::spawn(move || {
            let _ = run();
        });
    }
    true
}
