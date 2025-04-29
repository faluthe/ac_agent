use crate::err::Error;

use crate::hooks::IS_VISIBLE_FUNC;
use crate::hooks::TRACE_LINE_FUNC;

pub static mut PLAYER1_REF: Option<&Playerent> = None;
pub static mut PLAYER1: Option<u64> = None;

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
pub union world_pos {
    pub v: Vec3,
    f: [f32; 3],
    i: [i32; 3],
}

impl Default for world_pos {
    #[inline]
    fn default() -> Self {
        world_pos { v: Vec3::default() }
    }
}

#[repr(C)]
#[derive(Default)]
pub struct TraceresultS {
    pub end: world_pos,
    pub collided: bool,
    _padding: [u8; 3], // Ensure 4-byte alignment
}

#[repr(C)]
#[derive(Clone)]
pub struct Playerent {
    pub pointer: Option<&'static ()>,
    _pad_0x2c: [u8; 0x24],
    pub o: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
    _pad_0x100: [u8; 0xbc],
    pub health: i32,
    _pad_0x320: [u8; 0x21c],
    pub team: i32,
}

// for navigation .. creates rays to find the walls, tbf
pub fn ray_scan(k: u32, phi_min: f32, phi_max: f32) -> Result<Vec<*const TraceresultS>, Error> {
    let mut i = 0;

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

        let from: Vec3 = Vec3 {
            x: player1.o.x,
            y: player1.o.y,
            z: 5.5,
        };

        let world_pos_from: world_pos = world_pos { v: from };

        let to: Vec3 = Vec3 {
            x: from.x + f32::cos(player1.yaw) * ray_magnitude,
            y: from.y + f32::sin(player1.yaw) * ray_magnitude,
            z: from.z,
        };

        let world_pos_to: world_pos = world_pos { v: to };

        let mut tr: TraceresultS = TraceresultS::default();

        unsafe {
            match PLAYER1 {
                Some(p1) => traceline(world_pos_from, world_pos_to, p1, true, &mut tr),
                None => return Err(Error::Player1Error),
            };

            println!("TraceresultS end : {:?}", tr.end.v);
            println!("Collided : {:?}", tr.collided);
        }

        rays.push(&tr);

        i += 1;
    }
    Ok(rays)
}

pub fn is_enemy_visible(player1: &Playerent, player: &Playerent) -> Result<bool, Error> {
    let from: Vec3 = Vec3 {
        x: player1.o.x,
        y: player1.o.y,
        z: player1.o.z,
    };

    let world_pos_from: world_pos = world_pos { v: from };

    let to: Vec3 = Vec3 {
        x: player.o.x,
        y: player.o.y,
        z: player.o.z,
    };

    let world_pos_to: world_pos = world_pos { v: to };

    unsafe {
        match IS_VISIBLE_FUNC {
            Some(func) => return Ok(func(world_pos_from, world_pos_to, 0, false)),
            None => return Err(Error::TraceLineError),
        };
    }
}

pub fn closest_enemy(
    players_list_ptr: *const u64,
    players_length: usize,
    player1: &Playerent,
) -> Result<&Playerent, Error> {
    let a: Vec3 = Vec3 {
        x: player1.o.x,
        y: player1.o.y,
        z: player1.o.z,
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

        let b: Vec3 = Vec3 {
            x: player.o.x,
            y: player.o.y,
            z: player.o.z,
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
