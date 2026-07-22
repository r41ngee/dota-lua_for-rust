pub mod vector;
pub mod npc;
pub mod entity;
pub mod item;
pub mod modifier;
pub mod manager;
pub mod misc;
pub mod ability;

// Stub types for Lua API
pub struct Any;
pub struct Table;
pub struct QAngle {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
pub struct Function;