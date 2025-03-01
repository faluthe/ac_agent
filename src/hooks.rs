use crate::agent_utils::{TraceresultS, Vec};
use crate::err::Error;
use crate::sdl::SDL_event;

use libc::{RTLD_LAZY, c_char, c_int, dl_iterate_phdr, dl_phdr_info, dlopen, dlsym, size_t};
use std::ffi::c_void;
use std::mem::transmute;

type SdlPushEventFn = unsafe extern "C" fn(*mut SDL_event) -> i32;
type SdlGetMouseStateFn = unsafe extern "C" fn(*const i32, *const i32) -> u32;
type TraceLineFn = unsafe extern "C" fn(Vec, Vec, u64, bool, TraceresultS) -> *mut std::ffi::c_void;

static mut SDL_PUSHEVENT: Option<SdlPushEventFn> = None;
static mut SDL_GETMOUSESTATE: Option<SdlGetMouseStateFn> = None;
static mut TRACE_LINE: Option<TraceLineFn> = None;

static mut CHECK_INPUT_ADDR: Option<*mut u64> = None;
static mut _HOOK_ORIGINAL_INSTR_ADDR: Option<*mut c_void> = None;

macro_rules! cstr_static {
    ($s:expr) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

pub fn init_hooks(native_client_addr: u64) -> Result<(), Error> {
    unsafe {
        let sdl_lib_handle: *mut c_void = dlopen(cstr_static!("libSDL2-2.0.so"), RTLD_LAZY);

        if sdl_lib_handle.is_null() {
            return Err(Error::DlOpenError);
        }

        let sdl_push_event_handle = dlsym(sdl_lib_handle, cstr_static!("SDL_PushEvent"));

        if sdl_push_event_handle.is_null() {
            return Err(Error::DlSymError);
        }

        SDL_PUSHEVENT = transmute(sdl_push_event_handle);

        let sdl_get_mouse_state_handle = dlsym(sdl_lib_handle, cstr_static!("SDL_GetMouseState"));

        if sdl_push_event_handle.is_null() {
            return Err(Error::DlSymError);
        }

        SDL_GETMOUSESTATE = transmute(sdl_get_mouse_state_handle);

        TRACE_LINE = transmute((native_client_addr as *mut u64).offset(0x134520));

        CHECK_INPUT_ADDR = Some((native_client_addr as *mut u64).offset(0x772d0));

        Ok(())
    }
}

pub fn find_base_address() -> Result<u64, Error> {
    extern "C" fn callback(
        info: *mut dl_phdr_info,
        _size: size_t,
        data: *mut libc::c_void,
    ) -> c_int {
        let base_address = data as *mut u64;

        unsafe {
            let info = &*info;
            if info.dlpi_name.is_null() || *info.dlpi_name == 0 {
                *base_address = info.dlpi_addr;
                1
            } else {
                0
            }
        }
    }

    let mut base_address: u64 = 0;
    unsafe {
        dl_iterate_phdr(Some(callback), &mut base_address as *mut u64 as *mut c_void);
    }

    match base_address {
        0 => Err(Error::FindBaseAddrError),
        _ => Ok(base_address),
    }
}
