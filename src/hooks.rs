use crate::agent_utils::{TraceresultS, get_player1_info, ray_scan, world_pos};
use crate::err::Error;
use crate::sdl::SDL_event;

use libc::{RTLD_LAZY, c_char, c_int, dl_iterate_phdr, dl_phdr_info, dlopen, dlsym, size_t};
use std::ffi::c_void;
use std::mem::transmute;
use std::ptr::read_unaligned;

type SdlPushEventFn = unsafe extern "C" fn(*mut SDL_event) -> i32;
type SdlGetMouseStateFn = unsafe extern "C" fn(*const i32, *const i32) -> u32;
type SdlGLSwapWindowInnerFn = unsafe extern "C" fn(*const c_void);

type TracelineFn = unsafe extern "C" fn(world_pos, world_pos, u64, bool, *const TraceresultS);
type IsVisibleFn = unsafe extern "C" fn(world_pos, world_pos, u64, bool) -> bool;

pub static mut SDL_PUSHEVENT: Option<SdlPushEventFn> = None;
pub static mut SDL_GETMOUSESTATE: Option<SdlGetMouseStateFn> = None;
pub static mut TRACE_LINE_FUNC: Option<TracelineFn> = None;
pub static mut IS_VISIBLE_FUNC: Option<IsVisibleFn> = None;

static mut MUTABLE_INNER_FUNC_PTR: Option<*mut unsafe extern "C" fn(*const c_void)> = None;
static mut HOOK_ORIGINAL_INNER_FUNC: Option<SdlGLSwapWindowInnerFn> = None;

macro_rules! cstr_static {
    ($s:expr) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

unsafe extern "C" fn hook_func(window: *const c_void) {
    let result = get_player1_info();
    if !result {
        println!("unable to dereference player1 its a null ptr");
    }

    ray_scan(2, 0.0, 360.0).expect("ray tracing error");

    unsafe {
        match HOOK_ORIGINAL_INNER_FUNC {
            Some(func) => func(window),
            None => (),
        }
    }
}

pub fn sdl_gl_swap_window_hook(sdl_gl_swap_window_handle: *mut c_void) -> Result<(), Error> {
    unsafe {
        let wrapper_offset_location = sdl_gl_swap_window_handle as u64 + 0x4 + 0x2;

        let offset = read_unaligned(wrapper_offset_location as *const u32);

        MUTABLE_INNER_FUNC_PTR = Some(
            (sdl_gl_swap_window_handle as u64 + 0xa + offset as u64)
                as *mut unsafe extern "C" fn(*const c_void),
        );

        match MUTABLE_INNER_FUNC_PTR {
            Some(ptr) => {
                HOOK_ORIGINAL_INNER_FUNC = Some(*ptr);
                *(ptr) = hook_func;
            }
            None => return Err(Error::SDLHookError),
        };
    }
    Ok(())
}

pub fn sdl_gl_swap_window_recover() -> Result<(), Error> {
    unsafe {
        match MUTABLE_INNER_FUNC_PTR {
            Some(ptr) => {
                *(ptr) = match HOOK_ORIGINAL_INNER_FUNC {
                    Some(ptr) => ptr,
                    None => return Err(Error::SDLHookError),
                }
            }
            None => return Err(Error::SDLHookError),
        };
        println!("unhooked successfully");
        Ok(())
    }
}

pub fn init_hooks(native_client_addr: u64) -> Result<(), Error> {
    unsafe {
        let sdl_lib_handle: *mut c_void = dlopen(cstr_static!("libSDL2-2.0.so"), RTLD_LAZY);

        if sdl_lib_handle.is_null() {
            return Err(Error::DlOpenError);
        }

        let sdl_gl_swap_window_handle = dlsym(sdl_lib_handle, cstr_static!("SDL_GL_SwapWindow"));
        sdl_gl_swap_window_hook(sdl_gl_swap_window_handle)?;

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

        let trace_line_addr = (native_client_addr + 0x134520) as usize;
        TRACE_LINE_FUNC = Some(transmute::<usize, TracelineFn>(trace_line_addr));

        let is_visible_addr = (native_client_addr + 0x2F288000) as usize;
        IS_VISIBLE_FUNC = Some(transmute::<usize, IsVisibleFn>(is_visible_addr));

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
