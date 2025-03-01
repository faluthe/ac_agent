use crate::agent_utils::{TraceresultS, Vec};
use crate::sdl::SDL_event;
use libc::{RTLD_LAZY, c_int, dl_iterate_phdr, dl_phdr_info, dlopen, dlsym, size_t};
use std::ffi::CString;
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

pub unsafe fn init_hooks(native_client_addr: u64) {
    unsafe {
        let sdl_lib_handle = dlopen(CString::new("libSDL2-2.0.so").unwrap().as_ptr(), RTLD_LAZY);

        SDL_PUSHEVENT = transmute(dlsym(
            sdl_lib_handle as *mut c_void,
            CString::new("SDL_PushEvent").unwrap().as_ptr(),
        ));

        SDL_GETMOUSESTATE = transmute(dlsym(
            sdl_lib_handle as *mut c_void,
            CString::new("SDL_GetMouseState").unwrap().as_ptr(),
        ));

        TRACE_LINE = transmute((native_client_addr as *mut u64).offset(0x134520));

        CHECK_INPUT_ADDR = Some((native_client_addr as *mut u64).offset(0x772d0));
    }
}
