use rand::Rng;
use std::f32::consts::PI;
use std::ptr;

use crate::err::Error;

use crate::hooks::TRACE_LINE;

pub static mut PLAYER1_REF: Option<&Playerent> = None;

#[derive(Default)]
#[repr(C)]
pub struct AcVec {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Default)]
#[repr(C)]
pub struct TraceresultS {
    pub end: AcVec,
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

    let ray_magnitude: f32 = 1000.0;

    let mut rays: Vec<*const TraceresultS> = vec![];

    let player1 = match unsafe { PLAYER1_REF } {
        Some(p1) => p1,
        None => return Err(Error::Player1Error),
    };

    loop {
        if i == k {
            break;
        }

        let rand_yaw = rng.gen_range(phi_min..phi_max) * (PI / 180.0);

        let ray_target: AcVec = AcVec {
            x: f32::cos(rand_yaw) * ray_magnitude,
            y: f32::sin(rand_yaw) * ray_magnitude,
            z: 5.5,
        };

        let from: AcVec = AcVec {
            x: player1.x,
            y: player1.y,
            z: 5.5,
        };

        let mut tr = TraceresultS::default();

        match unsafe { TRACE_LINE } {
            Some(trace) => unsafe {
                trace(
                    from,
                    ray_target,
                    player1 as *const Playerent as u64,
                    true,
                    &mut tr,
                );
            },
            None => return Err(Error::TraceLineError),
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
