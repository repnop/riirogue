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
const ROOM_SIZE_X: std::ops::Range<u32> = 4..8;
const ROOM_SIZE_Y: std::ops::Range<u32> = 4..8;

struct Tile {
    pos: (u32, u32),
    tile_id: &'static str,
    color: Option<graphics::Color>,
}

impl Tile {
    fn new(tile_id: &'static str, pos: (u32, u32), color: Option<graphics::Color>) -> Tile {
        Tile {
            pos,
            tile_id,
            color,
        }
    }

    fn from_rect(room: Rect) -> Vec<Tile> {
        use room_gen::Rng;

        let mut tiles = Vec::with_capacity(room.x as usize * room.y as usize);

        for i in room.x..=room.x + room.width {
            for j in room.y..=room.y + room.height {
                if i == room.x && j == room.y {
                    tiles.push(Tile::new("wall", (i, j), None));
                } else if i == room.x + room.width && j == room.y {
                    tiles.push(Tile::new("wall", (i, j), None));
                } else if i == room.x && j == room.y + room.height {
                    tiles.push(Tile::new("wall", (i, j), None));
                } else if i == room.x + room.width && j == room.y + room.height {
                    tiles.push(Tile::new("wall", (i, j), None));
                } else if i == room.x || i == room.x + room.width {
                    tiles.push(Tile::new("wall", (i, j), None));
                } else if j == room.y || j == room.y + room.height {
                    tiles.push(Tile::new("wall", (i, j), None));
                } else {
                    let n = room_gen::thread_rng().gen_range(0, 20);
                    tiles.push(Tile::new(
                        if n < 8 && n >= 3 {
                            "floor_scatter_light"
                        } else if n < 3 {
                            "floor_scatter_heavy"
                        } else if n < 11 && n >= 8 {
                            "grass"
                        } else {
                            " "
                        },
                        (i, j),
                        None,
                    ));
                }
            }
        }

        tiles
    }
}

struct GameState {
    ts: TileSet,
    tiles: Vec<Tile>,
    menu_on: bool,
    menu_cursor_y: u32,
}

impl GameState {
    fn new(ctx: &mut Context) -> GameResult<GameState> {
        let image = graphics::Image::new(ctx, "/font_16.png")?;
        let mut ts = TileSet::new(image, (32, 8), (16, 16));

        ts.register_tile("smiley", (2, 0)).unwrap();
        ts.register_tile(" ", (0, 0)).unwrap();

        // Register alphabet
        ts.register_tile("A", (1, 2)).unwrap();
        ts.register_tile("B", (2, 2)).unwrap();
        ts.register_tile("C", (3, 2)).unwrap();
        ts.register_tile("D", (4, 2)).unwrap();
        ts.register_tile("E", (5, 2)).unwrap();
        ts.register_tile("F", (6, 2)).unwrap();
        ts.register_tile("G", (7, 2)).unwrap();
        ts.register_tile("H", (8, 2)).unwrap();
        ts.register_tile("I", (9, 2)).unwrap();
        ts.register_tile("J", (10, 2)).unwrap();
        ts.register_tile("K", (11, 2)).unwrap();
        ts.register_tile("L", (12, 2)).unwrap();
        ts.register_tile("M", (13, 2)).unwrap();
        ts.register_tile("N", (14, 2)).unwrap();
        ts.register_tile("O", (15, 2)).unwrap();
        ts.register_tile("P", (16, 2)).unwrap();
        ts.register_tile("Q", (17, 2)).unwrap();
        ts.register_tile("R", (18, 2)).unwrap();
        ts.register_tile("S", (19, 2)).unwrap();
        ts.register_tile("T", (20, 2)).unwrap();
        ts.register_tile("U", (21, 2)).unwrap();
        ts.register_tile("V", (22, 2)).unwrap();
        ts.register_tile("W", (23, 2)).unwrap();
        ts.register_tile("X", (24, 2)).unwrap();
        ts.register_tile("Y", (25, 2)).unwrap();
        ts.register_tile("Z", (26, 2)).unwrap();

        ts.register_tile("a", (1, 3)).unwrap();
        ts.register_tile("b", (2, 3)).unwrap();
        ts.register_tile("c", (3, 3)).unwrap();
        ts.register_tile("d", (4, 3)).unwrap();
        ts.register_tile("e", (5, 3)).unwrap();
        ts.register_tile("f", (6, 3)).unwrap();
        ts.register_tile("g", (7, 3)).unwrap();
        ts.register_tile("h", (8, 3)).unwrap();
        ts.register_tile("i", (9, 3)).unwrap();
        ts.register_tile("j", (10, 3)).unwrap();
        ts.register_tile("k", (11, 3)).unwrap();
        ts.register_tile("l", (12, 3)).unwrap();
        ts.register_tile("m", (13, 3)).unwrap();
        ts.register_tile("n", (14, 3)).unwrap();
        ts.register_tile("o", (15, 3)).unwrap();
        ts.register_tile("p", (16, 3)).unwrap();
        ts.register_tile("q", (17, 3)).unwrap();
        ts.register_tile("r", (18, 3)).unwrap();
        ts.register_tile("s", (19, 3)).unwrap();
        ts.register_tile("t", (20, 3)).unwrap();
        ts.register_tile("u", (21, 3)).unwrap();
        ts.register_tile("v", (22, 3)).unwrap();
        ts.register_tile("w", (23, 3)).unwrap();
        ts.register_tile("x", (24, 3)).unwrap();
        ts.register_tile("y", (25, 3)).unwrap();
        ts.register_tile("z", (26, 3)).unwrap();

        // Numbers
        ts.register_tile("0", (16, 1)).unwrap();
        ts.register_tile("1", (17, 1)).unwrap();
        ts.register_tile("2", (18, 1)).unwrap();
        ts.register_tile("3", (19, 1)).unwrap();
        ts.register_tile("4", (20, 1)).unwrap();
        ts.register_tile("5", (21, 1)).unwrap();
        ts.register_tile("6", (22, 1)).unwrap();
        ts.register_tile("7", (23, 1)).unwrap();
        ts.register_tile("8", (24, 1)).unwrap();
        ts.register_tile("9", (25, 1)).unwrap();

        // Specials
        ts.register_tile("!", (1, 1)).unwrap();
        ts.register_tile("\"", (2, 1)).unwrap();
        ts.register_tile("#", (3, 1)).unwrap();
        ts.register_tile("$", (4, 1)).unwrap();
        ts.register_tile("%", (5, 1)).unwrap();
        ts.register_tile("&", (6, 1)).unwrap();
        ts.register_tile("'", (7, 1)).unwrap();
        ts.register_tile("(", (8, 1)).unwrap();
        ts.register_tile(")", (9, 1)).unwrap();
        ts.register_tile("*", (10, 1)).unwrap();
        ts.register_tile("+", (11, 1)).unwrap();
        ts.register_tile(",", (12, 1)).unwrap();
        ts.register_tile("-", (13, 1)).unwrap();
        ts.register_tile(".", (14, 1)).unwrap();
        ts.register_tile("/", (15, 1)).unwrap();
        ts.register_tile(":", (26, 1)).unwrap();
        ts.register_tile(";", (27, 1)).unwrap();
        ts.register_tile("<", (28, 1)).unwrap();
        ts.register_tile("=", (29, 1)).unwrap();
        ts.register_tile(">", (30, 1)).unwrap();
        ts.register_tile("?", (31, 1)).unwrap();

        // Room Characters
        ts.register_tile("room_bottom_left", (8, 6)).unwrap();
        ts.register_tile("room_bottom_right", (28, 5)).unwrap();
        ts.register_tile("room_side_lr", (26, 5)).unwrap();
        ts.register_tile("room_side_tb", (13, 6)).unwrap();
        ts.register_tile("room_top_left", (27, 5)).unwrap();
        ts.register_tile("room_top_right", (9, 6)).unwrap();
        ts.register_tile("wall", (17, 5)).unwrap();
        ts.register_tile("floor_scatter_light", (13, 7)).unwrap();
        ts.register_tile("floor_scatter_heavy", (14, 7)).unwrap();
        ts.register_tile("grass", (27, 7)).unwrap();
        ts.register_tile("solid_block", (27, 6)).unwrap();

        let tiles = room_gen::gen_rooms(
            (TILES_X as usize, TILES_Y as usize),
            ROOM_SIZE_X,
            ROOM_SIZE_Y,
        );

        Ok(GameState {
            ts,
            tiles: tiles
                .into_iter()
                .map(|r| Tile::from_rect(r))
                .fold(Vec::new(), |mut v, t| {
                    v.extend(t);
                    v
                }),
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
        let center = (0, TILES_Y / 2);
        let top_left = (center.0, center.1 - TILES_Y / 4);

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
        for tile in &self.tiles {
            self.ts
                .queue_tile(tile.tile_id, tile.pos, tile.color)
                .unwrap();
        }

        self.draw_string(
            "w e w l a d  T E X T  D R A W I N G",
            (3, 5),
            Some(graphics::Color::from_rgba(0xF8, 0xD7, 0x90, 0xFF)),
        );

        self.draw_string(
            "now with more special characters!",
            (1, 24),
            Some(graphics::Color::from_rgba(0xF8, 0xD7, 0x90, 0xFF)),
        );

        self.draw_string(
            "!\"#$%&'()*+,-./0123456789:;<=>?",
            (1, 25),
            Some(graphics::Color::from_rgba(0xF8, 0xD7, 0x90, 0xFF)),
        );

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
        _: bool,
    ) {
        if let event::Keycode::R = keycode {
            self.tiles = room_gen::gen_rooms(
                (TILES_X as usize, TILES_Y as usize),
                ROOM_SIZE_X,
                ROOM_SIZE_Y,
            ).into_iter()
                .map(|r| Tile::from_rect(r))
                .fold(Vec::new(), |mut v, t| {
                    v.extend(t);
                    v
                });
        } else if let event::Keycode::M = keycode {
            self.menu_on = !self.menu_on;
        } else if let event::Keycode::Down = keycode {
            if self.menu_cursor_y + 1 > 5 {
                self.menu_cursor_y = 0;
            } else {
                self.menu_cursor_y = room_gen::clamp(self.menu_cursor_y + 1, 0, 5);
            }
        } else if let event::Keycode::Up = keycode {
            self.menu_cursor_y = room_gen::clamp(self.menu_cursor_y - 1, 0, 5);
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
