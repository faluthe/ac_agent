use crate::err::Error;

#[repr(C)]
pub struct Vec {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
pub struct TraceresultS {
    pub end: Vec,
    pub collided: bool,
}

#[repr(C)]
#[derive(Clone)]
pub struct Playerent {
    pub pointer: Option<&'static ()>,
    _pad_0x2c: [u8; 0x24],
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
    _pad_0x100: [u8; 0xbc],
    pub health: i32,
    _pad_0x320: [u8; 0x21c],
    pub team: i32,
}

pub fn closest_enemy(
    players_list_ptr: *const u64,
    players_length: usize,
    player1: &Playerent,
) -> Result<u32, Error> {
    let a: Vec = Vec {
        x: player1.x,
        y: player1.y,
        z: player1.z,
    };

    if players_list_ptr.is_null() {
        return Err(Error::PlayersListError(
            "failed to locate players list .. no players".to_string(),
        ));
    }

    let mut min_dist = f32::MAX;
    let mut closest_enemy_index: u32 = 0; // player1 is 0

    let mut i: usize = 0;

    loop {
        let addr = unsafe { *players_list_ptr.offset(i as isize) } as *const Playerent;
        if i == players_length {
            break;
        }
        i += 1;

        if addr.is_null() {
            continue;
        }

        let player: &Playerent = unsafe { &*addr };

        if player.team == player1.team {
            continue;
        }

        let b: Vec = Vec {
            x: player.x,
            y: player.y,
            z: player.z,
        };
        let distance =
            f32::sqrt(f32::powi(a.x - b.x, 2) + f32::powi(a.y - b.y, 2) + f32::powi(a.z - b.z, 2));

        if distance < min_dist {
            min_dist = distance;
            closest_enemy_index = i as u32 - 1;
        }
    }

    Ok(closest_enemy_index)
}
