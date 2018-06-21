pub mod generation;
mod pathfinding;

use constants::*;
use ggez::{
    error::GameResult, graphics::{Color, DrawParam}, Context,
};
use helpers::Rect;
use pathfinding::utils::absdiff;
use rand::{
    distributions::{Distribution, Weighted, WeightedChoice}, thread_rng,
};
use std::ops::{Add, Deref, DerefMut, Div, Mul, Sub};

pub struct Map {
    tiles: Vec<Tile>,
    width: u32,
    height: u32,
}

impl Map {
    pub fn new(width: u32, height: u32) -> Map {
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
            height,
        }
    }

    pub fn tile_at<T: Into<Coords<u32>>>(&self, coords: T) -> Option<Tile> {
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
            BlankRoomFloor | HeavyScatterRoomFloor | LightScatterRoomFloor | Grass | Pathway => {
                true
            }
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
            Door => TILE_CAP_R.name,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coords<T> {
    pub x: T,
    pub y: T,
}

impl<T> Coords<T> {
    pub fn new(x: T, y: T) -> Coords<T> {
        Coords { x, y }
    }
}

impl<
        T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy + PartialOrd,
    > Coords<T>
{
    fn distance(&self, other: Coords<T>) -> T {
        absdiff(self.x, other.x) + absdiff(self.y, other.y)
    }
}

impl<T> From<(T, T)> for Coords<T> {
    fn from((x, y): (T, T)) -> Coords<T> {
        Coords { x, y }
    }
}

impl<'a, T: Copy> From<&'a (T, T)> for Coords<T> {
    fn from(&(x, y): &(T, T)) -> Coords<T> {
        Coords { x, y }
    }
}

#[derive(Clone, Copy)]
pub struct Tile {
    pub pos: Coords<u32>,
    pub tile_type: TileType,
    pub color: Option<Color>,
}

impl Tile {
    fn new<T: Into<Coords<u32>>>(tile_type: TileType, pos: T, color: Option<Color>) -> Tile {
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
                    tiles.push(Tile::new(tile_picker.sample(&mut rng), (x, y), None));
                }
            }
        }

        tiles
    }
}
