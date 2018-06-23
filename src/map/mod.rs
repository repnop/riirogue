pub mod generation;
mod pathfinding;

use constants::*;
use entities;
use ggez::graphics::Color;
use helpers::{Coords, Rect};
use rand::{
    distributions::{Distribution, Weighted, WeightedChoice}, thread_rng,
};
use std::ops::{Deref, DerefMut};

pub struct Map {
    tiles: Vec<Tile>,
    width: i32,
    pub items: Vec<(Coords, Box<entities::Item>)>,
    pub monsters: Vec<Box<entities::Creature>>,
}

impl Map {
    pub fn new(width: i32, height: i32) -> Map {
        Map {
            tiles: (0..=width)
                .flat_map(|x| {
                    (0..=height).map(move |y| Tile {
                        pos: Coords { x, y },
                        tile_type: TileType::Empty,
                        color: None,
                    })
                })
                .collect(),
            width,
            items: Vec::new(),
            monsters: Vec::new(),
        }
    }

    pub fn add_item<I: entities::Item + 'static>(&mut self, coords: Coords, item: I) {
        self.items.push((coords, Box::new(item)));
    }

    pub fn add_creature<C: entities::Creature + 'static>(&mut self, creature: C) {
        self.monsters.push(Box::new(creature));
    }

    pub fn draw<F>(
        &self,
        tileset: &mut super::tileset::TileSet,
        filter: F,
        camera: Coords,
    ) -> Result<(), &str>
    where
        for<'r> F: FnMut(&'r &super::map::Tile) -> bool,
    {
        for tile in self.tiles.iter().filter(filter) {
            let draw_x = tile.pos.x - camera.x;
            let draw_y = tile.pos.y - camera.y;

            tileset.queue_tile(tile.tile_type.name(), (draw_x, draw_y), tile.color)?;
        }

        for (pos, item) in self.items.iter().filter(|(pos, _)| pos >= &camera) {
            let draw_x = pos.x - camera.x;
            let draw_y = pos.y - camera.y;

            tileset.queue_tile(
                item.tile_name(),
                (draw_x, draw_y),
                Some(Color::from_rgb(229, 191, 0)),
            )?;
        }

        for monster in self.monsters.iter().filter(|m| m.pos() >= camera) {
            let pos = monster.pos();
            let draw_x = pos.x - camera.x;
            let draw_y = pos.y - camera.y;

            tileset.queue_tile_with_background(
                "solid",
                monster.tile_name(),
                (draw_x, draw_y),
                Some(Color::from_rgb(0, 0, 0)),
                Some(Color::from_rgb(127, 255, 0)),
            )?;
        }

        Ok(())
    }

    pub fn tile_at<T: Into<Coords>>(&self, coords: T) -> Option<Tile> {
        let coords = coords.into();

        self.tiles
            .get((coords.y * self.width + coords.x) as usize)
            .map(|&t| t)
    }
}

impl Deref for Map {
    type Target = [Tile];

    fn deref(&self) -> &[Tile] {
        &self.tiles
    }
}

impl DerefMut for Map {
    fn deref_mut(&mut self) -> &mut [Tile] {
        &mut self.tiles
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileType {
    Empty,
    BlankRoomFloor,
    Grass,
    HeavyScatterRoomFloor,
    LightScatterRoomFloor,
    Pathway,
    Wall,
    Door,
}

impl TileType {
    pub fn is_room_tile(&self) -> bool {
        use self::TileType::*;

        match self {
            BlankRoomFloor | HeavyScatterRoomFloor | LightScatterRoomFloor | Grass | Wall => true,
            _ => false,
        }
    }

    pub fn is_walkable_tile(&self) -> bool {
        use self::TileType::*;

        match self {
            BlankRoomFloor
            | HeavyScatterRoomFloor
            | LightScatterRoomFloor
            | Grass
            | Pathway
            | Door => true,
            _ => false,
        }
    }

    pub fn is_empty(&self) -> bool {
        use self::TileType::*;

        match self {
            Empty => true,
            _ => false,
        }
    }

    pub fn name(&self) -> &'static str {
        use self::TileType::*;

        match self {
            Empty => TILE_SPACE.name,
            BlankRoomFloor => TILE_SPACE.name,
            Grass => TILE_ROOM_GRASS.name,
            HeavyScatterRoomFloor => TILE_ROOM_FLRSCHVY.name,
            LightScatterRoomFloor => TILE_ROOM_FLRSCLGT.name,
            Pathway => TILE_PATH.name,
            Wall => TILE_ROOM_WALL.name,
            Door => TILE_DOOR.name,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Tile {
    pub pos: Coords,
    pub tile_type: TileType,
    pub color: Option<Color>,
}

impl Tile {
    fn new<T: Into<Coords>>(tile_type: TileType, pos: T, color: Option<Color>) -> Tile {
        Tile {
            pos: pos.into(),
            tile_type,
            color,
        }
    }

    fn from_room_rect(room: Rect) -> Vec<Tile> {
        let mut tiles = Vec::with_capacity(room.x as usize * room.y as usize);

        let mut tile_types = vec![
            Weighted {
                weight: 2,
                item: TileType::Grass,
            },
            Weighted {
                weight: 4,
                item: TileType::LightScatterRoomFloor,
            },
            Weighted {
                weight: 2,
                item: TileType::HeavyScatterRoomFloor,
            },
            Weighted {
                weight: 8,
                item: TileType::BlankRoomFloor,
            },
        ];

        let tile_picker = WeightedChoice::new(&mut tile_types);
        let mut rng = thread_rng();

        for x in room.left()..=room.right() {
            for y in room.top()..=room.bottom() {
                if x == room.left() || x == room.right() || y == room.top() || y == room.bottom() {
                    tiles.push(Tile::new(TileType::Wall, (x, y), None));
                } else {
                    let tile = tile_picker.sample(&mut rng);

                    if tile == TileType::Grass {
                        tiles.push(Tile::new(tile, (x, y), Some(Color::from_rgb(0, 127, 0))));
                    } else {
                        tiles.push(Tile::new(tile, (x, y), None));
                    }
                }
            }
        }

        tiles
    }
}
