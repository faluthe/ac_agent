use agent_utils::Playerent;
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

    let _player1 = {
        let addr = (native_client_addr + 0x1ab4b8) as *const *const Playerent;
        unsafe { &**addr }
    };

    let _players_ref = {
        let addr = (native_client_addr + 0x1ab4c0) as *const *const u64;
        unsafe { &**addr }
    };

    init_hooks(native_client_addr)?;

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
