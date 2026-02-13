use macroquad::prelude::*;
use crate::graphics::*;
use super::TILE_SIZE;

pub const MAP_WIDTH: usize = 40;
pub const MAP_HEIGHT: usize = 30;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tile {
    Grass,
    Path,
    Building,
    Door,
    Water,
}

#[derive(Debug, Clone)]
pub struct Building {
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub building_type: BuildingType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BuildingType {
    Apartment,
    Library,
    CoffeeShop,
    Company { tier: u8 },
    JobCenter,
    Park,
}

pub struct GameMap {
    pub tiles: [[Tile; MAP_HEIGHT]; MAP_WIDTH],
    pub buildings: Vec<Building>,
}

impl GameMap {
    pub fn new() -> Self {
        let mut tiles = [[Tile::Grass; MAP_HEIGHT]; MAP_WIDTH];
        
        // Main horizontal path (middle)
        for x in 2..MAP_WIDTH-2 {
            tiles[x][MAP_HEIGHT/2] = Tile::Path;
            tiles[x][MAP_HEIGHT/2 + 1] = Tile::Path;
        }
        
        // Top horizontal path (connects all companies)
        for x in 2..MAP_WIDTH-2 {
            tiles[x][8] = Tile::Path;
            tiles[x][9] = Tile::Path;
        }
        
        // Bottom horizontal path (residential)
        for x in 2..MAP_WIDTH-2 {
            tiles[x][MAP_HEIGHT-6] = Tile::Path;
            tiles[x][MAP_HEIGHT-5] = Tile::Path;
        }
        
        // Vertical paths connecting areas
        for y in 8..MAP_HEIGHT-6 {
            tiles[10][y] = Tile::Path;
            tiles[20][y] = Tile::Path;
            tiles[30][y] = Tile::Path;
        }
        
        // Extra vertical paths near buildings
        for y in 8..MAP_HEIGHT/2 {
            tiles[7][y] = Tile::Path;
            tiles[15][y] = Tile::Path;
            tiles[25][y] = Tile::Path;
            tiles[34][y] = Tile::Path;
        }
        
        // Player spawn area path
        for x in 2..10 {
            tiles[x][MAP_HEIGHT-5] = Tile::Path;
        }

        let buildings = vec![
            // === RESIDENTIAL (bottom) ===
            Building {
                name: "Your Apartment".to_string(),
                x: 3,
                y: MAP_HEIGHT as i32 - 10,
                width: 3,
                height: 3,
                building_type: BuildingType::Apartment,
            },
            
            // === DOWNTOWN (center) ===
            Building {
                name: "Library".to_string(),
                x: MAP_WIDTH as i32 / 2 - 2,
                y: MAP_HEIGHT as i32 / 2 - 4,
                width: 4,
                height: 3,
                building_type: BuildingType::Library,
            },
            Building {
                name: "Coffee Shop".to_string(),
                x: MAP_WIDTH as i32 / 2 + 5,
                y: MAP_HEIGHT as i32 / 2 - 3,
                width: 3,
                height: 2,
                building_type: BuildingType::CoffeeShop,
            },
            
            // === TECH DISTRICT (top) ===
            Building {
                name: "DataStartup AI".to_string(),
                x: 5,
                y: 3,
                width: 4,
                height: 4,
                building_type: BuildingType::Company { tier: 0 },
            },
            Building {
                name: "TechCorp Inc".to_string(),
                x: 12,
                y: 3,
                width: 5,
                height: 4,
                building_type: BuildingType::Company { tier: 1 },
            },
            Building {
                name: "MegaTech".to_string(),
                x: 20,
                y: 2,
                width: 6,
                height: 5,
                building_type: BuildingType::Company { tier: 2 },
            },
            Building {
                name: "SearchGiant".to_string(),
                x: 28,
                y: 2,
                width: 7,
                height: 5,
                building_type: BuildingType::Company { tier: 3 },
            },
        ];

        Self { tiles, buildings }
    }

    pub fn draw(&self, cam_x: f32, cam_y: f32) {
        let start_x = (cam_x / TILE_SIZE) as i32 - 1;
        let start_y = (cam_y / TILE_SIZE) as i32 - 1;
        let end_x = start_x + (screen_width() / TILE_SIZE) as i32 + 2;
        let end_y = start_y + (screen_height() / TILE_SIZE) as i32 + 2;

        for x in start_x.max(0)..end_x.min(MAP_WIDTH as i32) {
            for y in start_y.max(0)..end_y.min(MAP_HEIGHT as i32) {
                let world_x = x as f32 * TILE_SIZE;
                let world_y = y as f32 * TILE_SIZE;
                let screen_x = world_x - cam_x;
                let screen_y = world_y - cam_y;
                
                match self.tiles[x as usize][y as usize] {
                    Tile::Grass => draw_grass_tile(screen_x, screen_y),
                    Tile::Path => draw_path_tile(screen_x, screen_y),
                    Tile::Water => draw_rectangle(screen_x, screen_y, TILE_SIZE, TILE_SIZE, Color::from_rgba(65, 105, 225, 255)),
                    _ => {}
                }
            }
        }

        for building in &self.buildings {
            let world_x = building.x as f32 * TILE_SIZE;
            let world_y = building.y as f32 * TILE_SIZE;
            let screen_x = world_x - cam_x;
            let screen_y = world_y - cam_y;
            
            match building.building_type {
                BuildingType::Apartment => draw_apartment(screen_x, screen_y),
                BuildingType::Library => draw_library(screen_x, screen_y),
                BuildingType::CoffeeShop => draw_coffee_shop(screen_x, screen_y),
                BuildingType::Company { tier } => draw_company(screen_x, screen_y, &building.name, tier),
                BuildingType::JobCenter => draw_building(screen_x, screen_y, building.width, building.height, &building.name, Color::from_rgba(150, 150, 200, 255)),
                BuildingType::Park => draw_park(screen_x, screen_y, building.width, building.height),
            }
        }
    }

    pub fn collides(&self, x: f32, y: f32, width: f32, height: f32) -> bool {
        let left = ((x - width/2.0) / TILE_SIZE) as i32;
        let right = ((x + width/2.0) / TILE_SIZE) as i32;
        let top = ((y - height/2.0) / TILE_SIZE) as i32;
        let bottom = ((y + height/2.0) / TILE_SIZE) as i32;

        for bx in left..=right {
            for by in top..=bottom {
                if bx < 0 || by < 0 || bx >= MAP_WIDTH as i32 || by >= MAP_HEIGHT as i32 {
                    return true;
                }
                for building in &self.buildings {
                    let b_right = building.x + building.width as i32;
                    let b_bottom = building.y + building.height as i32;
                    
                    if bx >= building.x && bx < b_right && by >= building.y && by < b_bottom {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn get_building_near(&self, x: f32, y: f32, radius: f32) -> Option<&Building> {
        let player_tile_x = (x / TILE_SIZE) as i32;
        let player_tile_y = (y / TILE_SIZE) as i32;
        
        let mut closest: Option<(&Building, f32)> = None;
        
        for building in &self.buildings {
            let building_center_x = (building.x + building.width as i32 / 2) as f32 * TILE_SIZE;
            let building_bottom_y = (building.y + building.height as i32) as f32 * TILE_SIZE;
            
            let dx = x - building_center_x;
            let dy = y - building_bottom_y;
            let dist = (dx * dx + dy * dy).sqrt();
            
            if dist < radius {
                match closest {
                    None => closest = Some((building, dist)),
                    Some((_, prev_dist)) if dist < prev_dist => {
                        closest = Some((building, dist));
                    }
                    _ => {}
                }
            }
        }
        
        closest.map(|(b, _)| b)
    }

    pub fn get_building_at(&self, x: f32, y: f32) -> Option<&Building> {
        self.get_building_near(x, y, 80.0)
    }
}
