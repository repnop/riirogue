use ggez::graphics::Color;
use helpers::*;
use map::*;
//use pathfinding::prelude::*;
use rand::{distributions::Uniform, thread_rng};
use std::ops::Range;

pub fn generate_map<T: MapGen>(opts: MapGenOptions) -> Map {
    T::gen(opts)
}

#[derive(Debug, Clone)]
pub struct MapGenOptions {
    pub map_width: i32,
    pub map_height: i32,
    pub room_width: Range<i32>,
    pub room_height: Range<i32>,
    pub outside_buffer: i32,
    pub room_buffer: i32,
}

pub trait MapGen {
    fn gen(options: MapGenOptions) -> Map;
}

pub struct Simple;

impl MapGen for Simple {
    fn gen(options: MapGenOptions) -> Map {
        use rand::Rng;

        let mut rooms = Vec::new();

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

        let mut map = Map::new(options.map_width, options.map_height);
        let mut doors = Vec::new();

        for _ in 0..=100 {
            let room = Rect::random_rect(&mut rng, &x_uniform, &y_uniform, &w_uniform, &h_uniform);

            if !rooms
                .iter()
                .any(|r: &Rect| r.intersects_with_buffer(&room, options.room_buffer))
            {
                for tile in Tile::from_room_rect(room) {
                    map[(tile.pos.y * options.map_width + tile.pos.x) as usize] = tile;
                }

                let center = room.center();

                let door_options = [
                    (center.0, room.y),
                    (center.0, room.y + room.height),
                    (room.x, center.1),
                    (room.x + room.width, center.1),
                ];

                let choice = door_options[thread_rng().gen_range(0, 4)];

                map[(choice.1 * options.map_width + choice.0) as usize] =
                    Tile::new(TileType::Door, choice, None);

                doors.push(choice);

                rooms.push(room);
            }
        }

        for (door1, door2) in doors
            .iter()
            .zip(doors.iter().skip(1))
            .map(|(&(x1, y1), &(x2, y2))| ((x1 as i32, y1 as i32), (x2 as i32, y2 as i32)))
        {
            let path = pathfinding::ortho_star(
                door1,
                door2,
                |x, y| {
                    let t = map.tile_at((x as i32, y as i32));

                    if let Some(t) = t {
                        !t.tile_type.is_room_tile()
                    } else {
                        false
                    }
                },
                |_, _| 1,
            );

            let color = Color::from_rgb(20, 20, 20);

            if let Some(path) = path {
                for (x, y) in path {
                    let (x, y) = (x as i32, y as i32);

                    if !doors.iter().any(|&coord| coord == (x, y)) {
                        map[(y * options.map_width + x) as usize] =
                            Tile::new(TileType::Pathway, (x, y), Some(color));
                    }
                }
            }
        }

        map
    }
}
