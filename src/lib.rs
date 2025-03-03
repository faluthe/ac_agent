use agent_utils::{Playerent, TraceresultS, closest_enemy, ray_scan};
use err::Error;
use hooks::{find_base_address, init_hooks};

mod agent_utils;
mod err;
mod hooks;
mod sdl;

#[used]
#[unsafe(link_section = ".init_array")]
static INIT: extern "C" fn() = {
    extern "C" fn init_wrapper() {
        match init() {
            Err(e) => eprintln!("Error during initialization: {:?}", e),
            _ => (),
        }
    }
    init_wrapper
};

fn init() -> Result<(), Error> {
    let native_client_addr: u64 = find_base_address()?;
    println!("native client handle {:?}", native_client_addr);

    let player1 = {
        let addr = (native_client_addr + 0x1ab4b8) as *const *const Playerent;
        unsafe { &**addr }
    };

    let players_ptr = {
        let addr = (native_client_addr + 0x1ab4c0) as *const *const u64;
        addr
    };

    if (unsafe { *players_ptr }).is_null() {
        return Err(Error::PlayersListError(
            "failed to locate players list .. no players".to_string(),
        ));
    }

    let players_length: usize = {
        let length_ptr = (players_ptr as u64 + 0xC) as *const u32;
        unsafe { *length_ptr as usize }
    };

    init_hooks(native_client_addr)?;

    println!("players {:?}", players_length);
    let closest_enemy = closest_enemy(unsafe { *players_ptr }, players_length, player1)?;

    println!("closest enemy index is {:?}", closest_enemy.health);

    //let scan_result = ray_scan(8, 0.0, 360.0, player1)?;
    //println!("how many rays were drawn: {:?}", scan_result.len());

    Ok(())
}

#[used]
#[unsafe(link_section = ".fini_array")]
static FINI: extern "C" fn() = {
    extern "C" fn fini() {
        println!("goodbye world");
    }
    fini
};
