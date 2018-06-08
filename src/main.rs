extern crate ggez;
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

use helpers::clamp;
use map::{
    generation::{generate_map, MapGenOptions, Nystrom}, Map,
};
use std::{env, path};
use tileset::TileSet;

const TILES_X: u32 = 51;
const TILES_Y: u32 = 41;
const TILE_SIZE: u32 = 16;
const ROOM_WIDTH: std::ops::Range<u32> = 4..8;
const ROOM_HEIGHT: std::ops::Range<u32> = 4..8;
const SCALE_FACTOR: f32 = 1.0;
const MAP_WIDTH: u32 = (TILES_X as f32 / SCALE_FACTOR) as u32;
const MAP_HEIGHT: u32 = (TILES_Y as f32 / SCALE_FACTOR) as u32;

struct GameState {
    ts: TileSet,
    map: Map,
    menu_on: bool,
    menu_cursor_y: u32,
}

impl GameState {
    fn new(ctx: &mut Context) -> GameResult<GameState> {
        let image = graphics::Image::new(ctx, "/font_16.png")?;
        let mut ts = TileSet::new(image, (32, 8), (16, 16), SCALE_FACTOR);

        constants::register_tiles(&mut ts).unwrap();

        Ok(GameState {
            ts,
            map: generate_map::<Nystrom>(MapGenOptions::new(
                MAP_WIDTH,
                MAP_HEIGHT,
                ROOM_WIDTH,
                ROOM_HEIGHT,
                0,
                1,
            )),
            menu_on: false,
            menu_cursor_y: 0,
        })
    }

    fn draw_string(&mut self, text: &str, origin: (u32, u32), color: Option<graphics::Color>) {
        let mut s = String::with_capacity(1);
        let mut origin_offset = 0;

        for c in text.chars() {
            s.push(c);

            if let Err(s) = self.ts
                .queue_tile(&s, (origin.0 + origin_offset, origin.1), color)
            {
                println!("`{}` not found", s);
            }
            //.unwrap();

            origin_offset += 1;
            s.pop();
        }
    }

    fn draw_menu(&mut self) {
        let center = (0, (TILES_Y as f32 / SCALE_FACTOR) as u32 / 2);
        let top_left = (
            center.0,
            center.1 - (TILES_Y as f32 / SCALE_FACTOR) as u32 / 4,
        );

        self.ts
            .queue_rect(
                "solid_block",
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
        for tile in self.map.iter().filter(|t| !t.tile_type.is_empty()) {
            if let Err(e) =
                self.ts
                    .queue_tile(tile.tile_type.name(), (tile.pos.x, tile.pos.y), tile.color)
            {
                println!("Tile \"{}\" not found", e);
            }
        }

        if self.menu_on {
            self.draw_menu();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
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

                self.map = generate_map::<Nystrom>(MapGenOptions::new(
                    MAP_WIDTH,
                    MAP_HEIGHT,
                    ROOM_WIDTH,
                    ROOM_HEIGHT,
                    0,
                    1,
                ));

                debugln!("Generation took: {} μs", time.elapsed().subsec_micros());
            } else if let event::Keycode::M = keycode {
                self.menu_on = !self.menu_on;
            } else if let event::Keycode::Down = keycode {
                if self.menu_cursor_y + 1 > 5 {
                    self.menu_cursor_y = 0;
                } else {
                    self.menu_cursor_y = clamp(self.menu_cursor_y + 1, 0, 5);
                }
            } else if let event::Keycode::Up = keycode {
                self.menu_cursor_y = clamp(self.menu_cursor_y - 1, 0, 5);
            }
        }
    }
}

fn main() {
    let c = conf::Conf {
        window_mode: WindowMode {
            width: TILES_X * TILE_SIZE,
            height: TILES_Y * TILE_SIZE,
            borderless: false,
            fullscreen_type: conf::FullscreenType::Off,
            vsync: true,
            max_width: TILES_X * TILE_SIZE,
            max_height: TILES_Y * TILE_SIZE,
            min_width: TILES_X * TILE_SIZE,
            min_height: TILES_Y * TILE_SIZE,
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
