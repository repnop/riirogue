extern crate ggez;
extern crate room_gen;

mod tileset;

use ggez::{
    conf::{self, WindowMode}, event, graphics, Context, GameResult,
};

use room_gen::Rect;
use std::{env, path};
use tileset::TileSet;

const TILES_X: u32 = 50;
const TILES_Y: u32 = 40;
const TILE_SIZE: u32 = 16;

struct GameState {
    ts: TileSet,
    tiles: Vec<Rect>,
}

impl GameState {
    fn new(ctx: &mut Context) -> GameResult<GameState> {
        let image = graphics::Image::new(ctx, "/font_16.png")?;
        let mut ts = TileSet::new(image, (32, 8), (16, 16));

        ts.register_tile("smiley", (2, 0)).unwrap();
        ts.register_tile("a", (1, 3)).unwrap();

        let tiles = room_gen::gen_rooms((TILES_X as usize, TILES_Y as usize), 6..10, 6..10);

        Ok(GameState { ts, tiles })
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        use room_gen::Rng;

        for tile in self.tiles.iter() {
            for i in tile.x..=tile.x + tile.width {
                for j in tile.y..=tile.y + tile.height {
                    let n = room_gen::thread_rng().gen_range(0, 2);

                    self.ts
                        .queue_tile(
                            if n == 1 { "smiley" } else { "a" },
                            (i, j),
                            Some(graphics::Color::from_rgba(
                                room_gen::thread_rng().gen_range(0, 255),
                                room_gen::thread_rng().gen_range(0, 255),
                                room_gen::thread_rng().gen_range(0, 255),
                                255,
                            )),
                        )
                        .unwrap();
                }
            }
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
        _: bool,
    ) {
        if let event::Keycode::R = keycode {
            self.tiles = room_gen::gen_rooms((TILES_X as usize, TILES_Y as usize), 6..10, 6..10);
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
