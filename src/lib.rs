use agent_utils::{PLAYER1_REF, Playerent, closest_enemy};
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

    let player1 = {
        let addr = (native_client_addr + 0x1ab4b8) as *const *const Playerent;
        unsafe { &**addr }
    };

    unsafe { PLAYER1_REF = Some(player1) };

    let players_ptr = {
        let addr = (native_client_addr + 0x1ab4c0) as *const *const u64;
        addr
    };

    if (unsafe { *players_ptr }).is_null() {
        return Err(Error::PlayersListError);
    }

    let players_length: usize = {
        let length_ptr = (players_ptr as u64 + 0xC) as *const u32;
        unsafe { *length_ptr as usize }
    };

    init_hooks(native_client_addr)?;

    let closest_enemy = closest_enemy(unsafe { *players_ptr }, players_length, player1)?;

    println!("closest enemy index is {:?}", closest_enemy.health);

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
