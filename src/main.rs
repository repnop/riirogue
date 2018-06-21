extern crate ggez;
extern crate pathfinding;
extern crate rand;

macro_rules! debugln {
    () => (#[cfg(debug_assertions)] print!("\n"));
    ($fmt:expr) => (#[cfg(debug_assertions)] print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (#[cfg(debug_assertions)] print!(concat!($fmt, "\n"), $($arg)*));
}

mod constants;
mod helpers;
mod map;
mod tileset;

use ggez::{
    conf::{self, WindowMode}, event, graphics, Context, GameResult,
};

use helpers::{clamp, Coords};
use map::{
    generation::{generate_map, MapGenOptions, Simple}, Map,
};
use std::{env, path};
use tileset::TileSet;

const TILES_X: i32 = 50;
const TILES_Y: i32 = 40;
const TILE_SIZE: i32 = 16;
const ROOM_WIDTH: std::ops::Range<i32> = 4..8;
const ROOM_HEIGHT: std::ops::Range<i32> = 4..8;
const SCALE_FACTOR: f32 = 0.5;
const DISPLAY_SCALE_FACTOR: f32 = 1.5;
const DISPLAY_MAP_WIDTH: i32 = (TILES_X as f32 / DISPLAY_SCALE_FACTOR) as i32;
const DISPLAY_MAP_HEIGHT: i32 = (TILES_Y as f32 / DISPLAY_SCALE_FACTOR) as i32;
const MAP_WIDTH: i32 = (TILES_X as f32 / SCALE_FACTOR) as i32;
const MAP_HEIGHT: i32 = (TILES_Y as f32 / SCALE_FACTOR) as i32;
const MAP_GEN_OPTIONS: MapGenOptions = MapGenOptions {
    map_width: MAP_WIDTH,
    map_height: MAP_HEIGHT,
    room_width: ROOM_WIDTH,
    room_height: ROOM_HEIGHT,
    outside_buffer: 2,
    room_buffer: 2,
};

struct GameState {
    ts: TileSet,
    map: Map,
    menu_on: bool,
    menu_cursor_y: i32,
    player_position: Coords,
}

impl GameState {
    fn new(ctx: &mut Context) -> GameResult<GameState> {
        let image = graphics::Image::new(ctx, "/font_16.png")?;
        let mut ts = TileSet::new(image, (32, 8), (16, 16), DISPLAY_SCALE_FACTOR);

        constants::register_tiles(&mut ts).unwrap();
        let map = generate_map::<Simple>(MAP_GEN_OPTIONS);
        let player_position = (|| {
            for tile in map.iter() {
                if tile.tile_type.is_walkable_tile() {
                    return Coords::new(tile.pos.x, tile.pos.y);
                }
            }

            Coords::new(0, 0)
        })();

        Ok(GameState {
            ts,
            map,
            menu_on: false,
            menu_cursor_y: 0,
            player_position,
        })
    }

    fn draw_string(&mut self, text: &str, origin: (i32, i32), color: Option<graphics::Color>) {
        let mut s = String::with_capacity(1);
        let mut origin_offset = 0;

        for c in text.chars() {
            s.push(c);

            if let Err(s) = self.ts
                .queue_tile(&s, (origin.0 + origin_offset, origin.1), color)
            {
                println!("`{}` not found", s);
            }

            origin_offset += 1;
            s.pop();
        }
    }

    fn draw_menu(&mut self) {
        let center = (0, (TILES_Y as f32 / SCALE_FACTOR) as i32 / 2);
        let top_left = (
            center.0,
            center.1 - (TILES_Y as f32 / SCALE_FACTOR) as i32 / 4,
        );

        self.ts
            .queue_rect(
                "solid",
                top_left,
                (TILES_X, TILES_Y / 2),
                Some(graphics::Color::from_rgba(0x00, 0x00, 0x00, 0xFA)),
            )
            .unwrap();

        let cursor_offset = self.menu_cursor_y;

        self.draw_string(
            ">",
            (5, top_left.1 + 5 + cursor_offset),
            Some(graphics::Color::from_rgba(0xFF, 0xFF, 0xFF, 0xFF)),
        );

        self.draw_string(
            "1. Menu Text",
            (6, top_left.1 + 5),
            Some(graphics::Color::from_rgba(0xFF, 0xFF, 0xFF, 0xFF)),
        );
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let player_position = self.player_position;

        let camera_position = Coords::new(
            clamp(
                player_position.x - DISPLAY_MAP_WIDTH / 2,
                0,
                DISPLAY_MAP_WIDTH * 2,
            ),
            clamp(
                player_position.y - DISPLAY_MAP_HEIGHT / 2,
                0,
                DISPLAY_MAP_HEIGHT * 2,
            ),
        );

        let player_position = Coords::new(
            player_position.x - camera_position.x,
            player_position.y - camera_position.y,
        );

        for tile in self.map.iter().filter(|t| {
            t.pos.x >= camera_position.x && t.pos.y >= camera_position.y && !t.tile_type.is_empty()
        }) {
            let draw_x = tile.pos.x - camera_position.x;
            let draw_y = tile.pos.y - camera_position.y;

            if tile.tile_type == map::TileType::Pathway
                || Coords::new(draw_x, draw_y) != player_position
            {
                if let Err(e) =
                    self.ts
                        .queue_tile(tile.tile_type.name(), (draw_x, draw_y), tile.color)
                {
                    println!("Tile \"{}\" not found", e);
                }
            }
        }

        self.ts
            .queue_tile(
                constants::TILE_CAP_P.name,
                (player_position.x, player_position.y),
                None,
            )
            .unwrap();

        if self.menu_on {
            self.draw_menu();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::set_background_color(ctx, graphics::Color::from_rgba(0, 0, 0, 1));
        graphics::clear(ctx);
        self.ts.render(ctx)?;
        self.ts.clear_queue();
        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: event::Keycode,
        _: event::Mod,
        repeat: bool,
    ) {
        if !repeat {
            if let event::Keycode::R = keycode {
                #[cfg(debug_assertions)]
                let time = std::time::Instant::now();

                self.map = generate_map::<Simple>(MAP_GEN_OPTIONS);

                debugln!("Generation took: {} ms", time.elapsed().subsec_millis());
            } else if let event::Keycode::M = keycode {
                self.menu_on = !self.menu_on;
            } else if let event::Keycode::Down = keycode {
                if self.menu_on {
                    if self.menu_cursor_y + 1 > 5 {
                        self.menu_cursor_y = 0;
                    } else {
                        self.menu_cursor_y = clamp(self.menu_cursor_y + 1, 0, 5);
                    }
                }
            } else if let event::Keycode::Up = keycode {
                if self.menu_on {
                    self.menu_cursor_y = clamp(self.menu_cursor_y - 1, 0, 5);
                }
            }
        }

        if let event::Keycode::Left = keycode {
            if let Some(tile) = self.map
                .tile_at((self.player_position.x - 1, self.player_position.y))
            {
                if tile.tile_type.is_walkable_tile() {
                    self.player_position.x -= 1;
                }
            }
        } else if let event::Keycode::Right = keycode {
            if let Some(tile) = self.map
                .tile_at((self.player_position.x + 1, self.player_position.y))
            {
                if tile.tile_type.is_walkable_tile() {
                    self.player_position.x += 1;
                }
            }
        } else if let event::Keycode::Up = keycode {
            if let Some(tile) = self.map
                .tile_at((self.player_position.x, self.player_position.y - 1))
            {
                if tile.tile_type.is_walkable_tile() {
                    self.player_position.y -= 1;
                }
            }
        } else if let event::Keycode::Down = keycode {
            if let Some(tile) = self.map
                .tile_at((self.player_position.x, self.player_position.y + 1))
            {
                if tile.tile_type.is_walkable_tile() {
                    self.player_position.y += 1;
                }
            }
        }
    }
}

fn main() {
    let c = conf::Conf {
        window_mode: WindowMode {
            width: (TILES_X * TILE_SIZE) as u32,
            height: (TILES_Y * TILE_SIZE) as u32,
            borderless: false,
            fullscreen_type: conf::FullscreenType::Off,
            vsync: true,
            max_width: (TILES_X * TILE_SIZE) as u32,
            max_height: (TILES_Y * TILE_SIZE) as u32,
            min_width: (TILES_X * TILE_SIZE) as u32,
            min_height: (TILES_Y * TILE_SIZE) as u32,
        },
        window_setup: conf::WindowSetup::default(),
        backend: conf::Backend::OpenGL { major: 3, minor: 2 },
    };

    let ctx = &mut Context::load_from_conf("spritebatch", "ggez", c).unwrap();

    graphics::set_default_filter(ctx, graphics::FilterMode::Nearest);
    // We add the CARGO_MANIFEST_DIR/resources do the filesystems paths so
    // we we look in the cargo project for files.
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx.filesystem.mount(&path, true);
    }

    let state = &mut GameState::new(ctx).unwrap();

    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}
