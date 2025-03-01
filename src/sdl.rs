pub union SDL_event {
    pub event_type: u32,
    /**< Event type, shared with all events */
    pub key: SDL_KeyboardEvent,
    /**< Keyboard event data */
    pub motion: SDL_MouseMotionEvent,
    /**< Keyboard event data */
    pub button: SDL_MouseButtonEvent,
    /**< Keyboard event data */
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
    /**< In milliseconds, populated using SDL_GetTicks() */
    pub window_id: u32,
    /**< The window with mouse focus, if any */
    pub which: u32,
    /**< The mouse instance id, or SDL_TOUCH_MOUSEID */
    pub state: u32,
    /**< The current button state */
    pub x: i32,
    /**< X coordinate, relative to window */
    pub y: i32,
    /**< Y coordinate, relative to window */
    pub xrel: i32,
    /**< The relative motion in the X direction */
    pub yrel: i32,
}

#[derive(Copy, Clone)]
pub struct SDL_MouseButtonEvent {
    pub event_type: u32,
    /**< ::SDL_MOUSEBUTTONDOWN or ::SDL_MOUSEBUTTONUP */
    pub timestamp: u32,
    /**< In milliseconds, populated using SDL_GetTicks() */
    pub window_id: u32,
    /**< The window with mouse focus, if any */
    pub which: u32,
    /**< The mouse instance id, or SDL_TOUCH_MOUSEID */
    pub button: u32,
    /**< The mouse button index */
    pub state: u32,
    /**< ::SDL_PRESSED or ::SDL_RELEASED */
    pub clicks: u32,
    /**< 1 for single-click, 2 for double-click, etc. */
    pub padding1: u32,
    pub x: i32,
    /**< X coordinate, relative to window */
    pub y: i32,
}
