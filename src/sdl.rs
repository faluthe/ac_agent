#[derive()]
pub union SDL_event {
    pub event_type: u32,
    pub key: SDL_KeyboardEvent,
    pub motion: SDL_MouseMotionEvent,
    pub button: SDL_MouseButtonEvent,
    pub _padding: [u8; 56],
}

#[derive(Copy, Clone)]
pub struct SDL_Keysym {
    pub scancode: u8,
    pub sym: u32,
    pub modifier: u16,
    pub unused: u32,
}

#[derive(Copy, Clone)]
pub struct SDL_KeyboardEvent {
    pub event_type: u32,
    pub timestamp: u32,
    pub window_id: u32,
    pub state: u8,
    pub repeat: u8,
    pub padding2: u8,
    pub padding3: u8,
    pub keysym: SDL_Keysym,
}

#[derive(Copy, Clone)]
pub struct SDL_MouseMotionEvent {
    pub event_type: u32,
    pub timestamp: u32,
    pub window_id: u32,
    pub which: u32,
    pub state: u32,
    pub x: i32,
    pub y: i32,
    pub xrel: i32,
    pub yrel: i32,
}

#[derive(Copy, Clone)]
pub struct SDL_MouseButtonEvent {
    pub event_type: u32,
    pub timestamp: u32,
    pub window_id: u32,
    pub which: u32,
    pub button: u32,
    pub state: u32,
    pub clicks: u32,
    pub padding1: u32,
    pub x: i32,
    pub y: i32,
}
