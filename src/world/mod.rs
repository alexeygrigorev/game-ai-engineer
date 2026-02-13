mod player;
mod camera;
mod map;
pub mod npc;

pub use player::{Direction, WorldPlayer};
pub use camera::Camera;
pub use map::{GameMap, Building, BuildingType, Tile, MAP_WIDTH, MAP_HEIGHT};
pub use npc::{Npc, NpcType, get_npcs};

pub const TILE_SIZE: f32 = 32.0;
