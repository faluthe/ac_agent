use rand::Rng;
use std::f32::consts::PI;

use crate::err::Error;

use crate::hooks::TRACE_LINE_FUNC;

pub static mut PLAYER1_REF: Option<&Playerent> = None;
pub static mut PLAYER1: Option<u64> = None;

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct AcVec {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct TraceresultS {
    pub end: AcVec,
    pub collided: bool,
    _padding: [u8; 3], // Ensure 4-byte alignment
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

pub fn get_player1_info() -> bool {
    match unsafe { PLAYER1_REF } {
        Some(p1) => {
            println!("health {:?}", p1.health);
            println!("x {:?} y {:?} z {:?}", p1.x, p1.y, p1.z);
            true
        }
        None => false,
    }
}

pub fn ray_scan(k: u32, phi_min: f32, phi_max: f32) -> Result<Vec<*const TraceresultS>, Error> {
    let mut i = 0;
    let mut rng = rand::thread_rng();

    let ray_magnitude: f32 = 100.0;

    let mut rays: Vec<*const TraceresultS> = vec![];

    let player1 = match unsafe { PLAYER1_REF } {
        Some(p1) => p1,
        None => return Err(Error::Player1Error),
    };

    let traceline = match unsafe { TRACE_LINE_FUNC } {
        Some(func) => func,
        None => return Err(Error::TraceLineError),
    };

    loop {
        if i == k {
            break;
        }

        let rand_yaw = rng.gen_range(phi_min..phi_max) * (PI / 180.0);

        let from: AcVec = AcVec {
            x: player1.x,
            y: player1.y,
            z: 5.5,
        };

        let ray_target: AcVec = AcVec {
            x: from.x + f32::cos(rand_yaw) * ray_magnitude,
            y: from.y + f32::sin(rand_yaw) * ray_magnitude,
            z: from.z,
        };

        let mut tr = TraceresultS::default();

        unsafe {
            println!("TRACE_LINE_FUNC: {:X}", traceline as u64);
            println!("from: {:?}", from);
            println!("ray_target: {:?}", ray_target);

            match PLAYER1 {
                Some(p1) => traceline(from, ray_target, p1, true, &mut tr),
                None => return Err(Error::Player1Error),
            };

            println!("TraceresultS end : {:?}", tr.end);
            println!("Collided : {:?}", tr.collided);
        }

        rays.push(&tr);

        i += 1;
    }
    Ok(rays)
}

pub fn closest_enemy(
    players_list_ptr: *const u64,
    players_length: usize,
    player1: &Playerent,
) -> Result<&Playerent, Error> {
    let a: AcVec = AcVec {
        x: player1.x,
        y: player1.y,
        z: player1.z,
    };

    if players_list_ptr.is_null() {
        return Err(Error::PlayersListError);
    }

    let mut min_dist = f32::MAX;
    let mut closest_enemy: Option<&Playerent> = None; // player1 is 0

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

        let b: AcVec = AcVec {
            x: player.x,
            y: player.y,
            z: player.z,
        };
        let distance =
            f32::sqrt(f32::powi(a.x - b.x, 2) + f32::powi(a.y - b.y, 2) + f32::powi(a.z - b.z, 2));

        if distance < min_dist {
            min_dist = distance;
            closest_enemy = Some(player);
        }
    }

    match closest_enemy {
        Some(enemy) => Ok(enemy),
        None => Err(Error::PlayersListError),
    }
}
