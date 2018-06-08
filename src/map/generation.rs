use helpers::*;
use map::*;
use rand::{distributions::Uniform, thread_rng};
use std::ops::Range;

pub fn generate_map<T: MapGen>(opts: MapGenOptions) -> Map {
    T::gen(opts)
}

#[derive(Debug, Clone)]
pub struct MapGenOptions {
    map_width: u32,
    map_height: u32,
    room_width: Range<u32>,
    room_height: Range<u32>,
    outside_buffer: u32,
    room_buffer: u32,
}

impl MapGenOptions {
    pub fn new(
        map_width: u32,
        map_height: u32,
        room_width: Range<u32>,
        room_height: Range<u32>,
        outside_buffer: u32,
        room_buffer: u32,
    ) -> MapGenOptions {
        MapGenOptions {
            map_width,
            map_height,
            room_width,
            room_height,
            outside_buffer,
            room_buffer,
        }
    }
}

pub trait MapGen {
    fn gen(options: MapGenOptions) -> Map;
}

pub struct Nystrom;

impl Nystrom {
    fn gen_rooms(options: MapGenOptions) -> Vec<Tile> {
        const MAX_TRIES: u32 = 200;

        let mut rooms = Vec::new();
        let mut tiles = Vec::with_capacity((options.map_height * options.map_width) as usize);

        for x in 0..options.map_width {
            for y in 0..options.map_height {
                tiles.push(Tile::new(TileType::Empty, (x, y), None));
            }
        }

        let mut rng = thread_rng();

        let x_uniform = Uniform::new(
            0 + options.outside_buffer,
            options.map_width - options.outside_buffer - options.room_width.end,
        );

        let y_uniform = Uniform::new(
            0 + options.outside_buffer,
            options.map_height - options.outside_buffer - options.room_height.end,
        );

        let w_uniform = Uniform::new(options.room_width.start, options.room_width.end);
        let h_uniform = Uniform::new(options.room_height.start, options.room_height.end);

        let mut tries = 0;

        debugln!("Entering generation...");

        while tries < MAX_TRIES {
            let room = Rect::random_rect(&mut rng, &x_uniform, &y_uniform, &w_uniform, &h_uniform);
            if !rooms
                .iter()
                .any(|r: &Rect| r.intersects_with_buffer(&room, options.room_buffer))
            {
                rooms.push(room);

                for tile in Tile::from_room_rect(room) {
                    tiles[(tile.pos.y * options.map_width + tile.pos.x) as usize] = tile;
                }

                tries = 0;
            } else {
                tries += 1;
            }
        }

        tiles
    }

    fn gen_maze(options: MapGenOptions, map: &mut [Tile]) {
        let map_width = options.map_width;
        let map_height = options.map_height;

        while let Some(start) = find_start(map, map_height, map_width) {
            let mut next_tiles = Vec::new();

            next_tiles.push(start);

            while !next_tiles.is_empty() {
                let tile = next_tiles.pop().unwrap();
                dig(map, map_width, tile);

                for dir in vec![
                    Direction::North,
                    Direction::South,
                    Direction::East,
                    Direction::West,
                ] {
                    let (x, y) = tile;

                    if dir == Direction::North || dir == Direction::South {
                        let ecoord = Direction::East.apply_direction((x, y));
                        let wcoord = Direction::West.apply_direction((x, y));
                        let eidx = (ecoord.1 * map_width + ecoord.0) as usize;
                        let widx = (wcoord.1 * map_width + wcoord.0) as usize;
                        if let Some(Tile {
                            tile_type: TileType::Empty,
                            ..
                        }) = map.get(eidx)
                        {
                            if let Some(Tile {
                                tile_type: TileType::Empty,
                                ..
                            }) = map.get(widx)
                            {
                                if can_dig(map, map_width, tile, dir) {
                                    next_tiles.push(dir.apply_direction(tile));
                                }
                            }
                        }
                    } else {
                        let ncoord = Direction::North.apply_direction((x, y));
                        let scoord = Direction::South.apply_direction((x, y));
                        let nidx = (ncoord.1 * map_width + ncoord.0) as usize;
                        let sidx = (scoord.1 * map_width + scoord.0) as usize;
                        if let Some(Tile {
                            tile_type: TileType::Empty,
                            ..
                        }) = map.get(nidx)
                        {
                            if let Some(Tile {
                                tile_type: TileType::Empty,
                                ..
                            }) = map.get(sidx)
                            {
                                if can_dig(map, map_width, tile, dir) {
                                    next_tiles.push(dir.apply_direction(tile));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn apply_direction(&self, (x, y): (u32, u32)) -> (u32, u32) {
        match self {
            Direction::North => (x, y - 1),
            Direction::East => (x + 1, y),
            Direction::South => (x, y + 1),
            Direction::West => (x - 1, y),
        }
    }

    fn chance_turn(self) -> Direction {
        use rand::Rng;

        let v = thread_rng().gen_range(0, 4);

        match self {
            Direction::North => if v == 0 {
                Direction::East
            } else {
                self
            },
            Direction::East => if v == 0 {
                Direction::South
            } else {
                self
            },
            Direction::South => if v == 0 {
                Direction::West
            } else {
                self
            },
            Direction::West => if v == 0 {
                Direction::North
            } else {
                self
            },
        }
    }
}

fn can_dig(map: &[Tile], map_width: u32, pos: (u32, u32), dir: Direction) -> bool {
    let (x, y) = dir.apply_direction(pos);
    let idx = (y * map_width + x) as usize;

    if idx < map.len() {
        if let Some(Tile {
            tile_type: TileType::Empty,
            ..
        }) = map.get(idx)
        {
            let (x2, y2) = dir.apply_direction(dir.apply_direction(pos));
            let idx2 = (y2 * map_width + x2) as usize;

            if let Some(Tile {
                tile_type: TileType::Empty,
                ..
            }) = map.get(idx2)
            {
                true
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    }
}

fn dig(map: &mut [Tile], map_width: u32, (x, y): (u32, u32)) {
    let idx = (y * map_width + x) as usize;

    map[idx] = Tile::new(TileType::Pathway, (x, y), None);
}

fn find_start(map: &[Tile], map_height: u32, map_width: u32) -> Option<(u32, u32)> {
    let mut pos = None;

    'lp: for x in 0..map_width {
        for y in 0..map_height {
            let ncoord = Direction::North.apply_direction((x, y));
            let scoord = Direction::South.apply_direction((x, y));
            let ecoord = Direction::East.apply_direction((x, y));
            let wcoord = Direction::West.apply_direction((x, y));

            let nidx = (ncoord.1 * map_width + ncoord.0) as usize;
            let sidx = (scoord.1 * map_width + scoord.0) as usize;
            let eidx = (ecoord.1 * map_width + ecoord.0) as usize;
            let widx = (wcoord.1 * map_width + wcoord.0) as usize;

            if let Some(Tile {
                tile_type: TileType::Empty,
                ..
            }) = map.get(nidx)
            {
                if let Some(Tile {
                    tile_type: TileType::Empty,
                    ..
                }) = map.get(sidx)
                {
                    if let Some(Tile {
                        tile_type: TileType::Empty,
                        ..
                    }) = map.get(eidx)
                    {
                        if let Some(Tile {
                            tile_type: TileType::Empty,
                            ..
                        }) = map.get(widx)
                        {
                            if can_dig(map, map_width, (x, y), Direction::North)
                                || can_dig(map, map_width, (x, y), Direction::South)
                                || can_dig(map, map_width, (x, y), Direction::East)
                                || can_dig(map, map_width, (x, y), Direction::West)
                            {
                                pos = Some((x, y));
                                break 'lp;
                            }
                        }
                    }
                }
            }
        }
    }

    pos
}

impl MapGen for Nystrom {
    fn gen(options: MapGenOptions) -> Map {
        let mut tiles = Nystrom::gen_rooms(options.clone());
        Nystrom::gen_maze(options.clone(), &mut tiles);

        Map {
            width: options.map_width,
            height: options.map_height,
            tiles,
        }
    }
}