use bevy::prelude::*;

pub const MAP_SPRITE_SHEET_PATH: &str = "env_tiles.png";
pub const MAP_TILE_SIZE: Vec2 = Vec2 { x: 8., y: 8. };
pub const MAP_TILES: Tiles = Tiles { rows: 8, cols: 8 };
pub const OBJ_SPRITE_SHEET_PATH: &str = "obj_tiles.png";
pub const OBJ_TILE_SIZE: Vec2 = Vec2 { x: 16., y: 16. };
pub const OBJ_TILES: Tiles = Tiles { rows: 21, cols: 21 };
pub const OBJ_TILE_PADDING: Vec2 = Vec2 { x: 8., y: 8. };
pub const OBJ_TILE_OFFSET: Vec2 = Vec2 { x: 8., y: 8. };

pub struct Tiles {
    pub rows: usize,
    pub cols: usize,
}
#[derive(Resource, Default)]
pub struct MapSpriteSheet {
    pub map_tiles: Handle<TextureAtlas>,
}

#[derive(Resource, Default)]
pub struct ObjectsSpriteSheet {
    pub obj_tiles: Handle<TextureAtlas>,
}
