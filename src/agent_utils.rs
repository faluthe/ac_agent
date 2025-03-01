#[repr(C)]
pub struct Vec {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
pub struct TraceresultS {
    pub end: Vec,
    pub collided: bool,
}

#[repr(C)]
pub struct Playerent {
    _pad_0x28: [u8; 0x28],
    pub uint32: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
    _pad_0x100: [u8; 0xbc],
    pub health: i32,
    _pad_0x320: [u8; 0x21c],
    pub team: u32,
}
