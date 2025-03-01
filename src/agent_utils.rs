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
