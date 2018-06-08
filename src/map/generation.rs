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
        use rand::Rng;

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

        fn carve(map: &mut [Tile], map_width: u32, (x, y): (u32, u32)) {
            map[(y * map_width + x) as usize] = Tile::new(TileType::Pathway, (x, y), None);
        }

        fn can_carve(map: &mut [Tile], map_width: u32, pos: (u32, u32)) -> bool {
            if let Some(t) = map.get((pos.1 * map_width + pos.0) as usize) {
                if let TileType::Empty = t.tile_type {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        }

        let map_width = options.map_width;
        let map_height = options.map_height;

        while let Some(start) = find_open(map, map_height, map_width) {
            let mut dir = Direction::South;
            let mut cells = Vec::new();

            cells.push(start);

            while !cells.is_empty() {
                let cell = cells.pop().unwrap();
                carve(map, map_width, cell);

                let mut unmade_cells: Vec<Direction> = Vec::new();

                for dir in vec![
                    Direction::North,
                    Direction::East,
                    Direction::South,
                    Direction::West,
                ] {
                    if can_carve(map, map_width, dir.apply_direction(cell)) {
                        unmade_cells.push(dir);
                    }
                }

                if !unmade_cells.is_empty() {
                    let ndir;

                    if unmade_cells.contains(&dir) {
                        ndir = dir.chance_turn();
                    } else {
                        let idx = thread_rng().gen_range(0, unmade_cells.len());
                        ndir = unmade_cells[idx];
                    }

                    let pos = ndir.apply_direction(cell);
                    //carve(map, map_width, ndir.apply_direction((x, y)));

                    cells.push(pos);
                    dir = ndir;
                } else {
                    cells.pop();
                }
            }
        }
    }
}

fn find_open(map: &[Tile], map_height: u32, map_width: u32) -> Option<(u32, u32)> {
    let mut loc = None;

    'lp: for x in 0..map_width {
        for y in 0..map_height {
            if map[(y * map_width + x) as usize].tile_type.is_empty() {
                loc = Some((x, y));
                break 'lp;
            }
        }
    }

    loc
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
