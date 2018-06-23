extern crate ggez;
extern crate pathfinding;
extern crate rand;

macro_rules! debugln {
    () => (#[cfg(debug_assertions)] print!("\n"));
    ($fmt:expr) => (#[cfg(debug_assertions)] print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (#[cfg(debug_assertions)] print!(concat!($fmt, "\n"), $($arg)*));
}

mod constants;
mod entities;
mod helpers;
mod map;
mod tileset;

use entities::Player;
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
const DISPLAY_SCALE_FACTOR: f32 = 1.25;
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

#[derive(Debug, Clone, Copy)]
enum EventType {
    Combat,
    Healing,
    Item,
}

impl EventType {
    fn icon(&self) -> &'static str {
        use self::EventType::*;

        match self {
            Combat => "axe",
            Healing => "potion",
            Item => "$",
        }
    }

    fn color(&self) -> graphics::Color {
        use self::EventType::*;

        match self {
            Combat => graphics::Color::from_rgb(191, 0, 0),
            Healing => graphics::Color::from_rgb(0, 191, 0),
            Item => graphics::Color::from_rgb(191, 191, 0),
        }
    }
}

#[derive(Debug, Clone)]
struct Event {
    msg: String,
    ty: EventType,
}

impl Event {
    fn new(msg: String, ty: EventType) -> Event {
        Event { msg, ty }
    }
}

struct GameState {
    ts: TileSet,
    map: Map,
    menu_on: bool,
    menu_cursor_y: i32,
    player: Player,
    events: Vec<Event>,
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
            player: Player::new(player_position),
            events: Vec::new(),
        })
    }

    fn draw_string(&mut self, text: &str, origin: (i32, i32), color: Option<graphics::Color>) {
        let mut s = String::with_capacity(1);
        let mut origin_offset = 0;

        for c in text.chars() {
            s.push(c);

            if let Err(s) = self.ts.queue_tile_with_background(
                "solid",
                &s,
                (origin.0 + origin_offset, origin.1),
                Some(graphics::Color::from_rgba(0, 0, 0, 0xFF)),
                color,
            ) {
                println!("`{}` not found", s);
            }

            origin_offset += 1;
            s.pop();
        }
    }

    fn draw_events(&mut self) {
        let events: Vec<_> = self.events
            .iter()
            .rev()
            .take(5)
            .map(|s| s.clone())
            .collect();
        for (i, event) in events.into_iter().enumerate() {
            self.draw_string("[", (0, DISPLAY_MAP_HEIGHT - (5 - i as i32)), None);
            self.ts
                .queue_tile_with_background(
                    "solid",
                    event.ty.icon(),
                    (1, DISPLAY_MAP_HEIGHT - (5 - i as i32)),
                    Some(graphics::Color::from_rgba(0, 0, 0, 0xFF)),
                    Some(event.ty.color()),
                )
                .unwrap();
            self.draw_string("]", (2, DISPLAY_MAP_HEIGHT - (5 - i as i32)), None);

            self.draw_string(&event.msg, (3, DISPLAY_MAP_HEIGHT - (5 - i as i32)), None);
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
        let player_position = self.player.pos;

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

        self.map
            .draw(
                &mut self.ts,
                |t| {
                    t.pos >= camera_position && !t.tile_type.is_empty()
                        && (t.tile_type == map::TileType::Pathway || t.pos != player_position)
                },
                camera_position,
            )
            .unwrap();

        let player_position = Coords::new(
            player_position.x - camera_position.x,
            player_position.y - camera_position.y,
        );

        self.ts
            .queue_tile(
                constants::TILE_SPEC_AT.name,
                (player_position.x, player_position.y),
                None,
            )
            .unwrap();

        let hp = self.player.hp;
        self.draw_string(&format!("HP: {}", hp), (0, 0), None);

        self.draw_events();

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
        use entities::Creature;

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
            } else if let event::Keycode::H = keycode {
                let Coords { x: p_x, y: p_y } = self.player.pos;

                if let Some(tile) = self.map.tile_at((p_x + 1, p_y)) {
                    self.map.add_item(tile.pos, entities::HealingPotion {});
                }
            } else if let event::Keycode::U = keycode {
                if let Some(pos) = self.player.inv.iter().position(|i| i.id() == 0) {
                    let p_hp = self.player.hp;
                    self.player.inv.remove(pos).consume(&mut self.player);
                    self.events.push(Event::new(
                        format!("Player gained {} HP.", self.player.hp - p_hp),
                        EventType::Healing,
                    ));
                }
            } else if let event::Keycode::G = keycode {
                let Coords { x: p_x, y: p_y } = self.player.pos;

                if let Some(tile) = self.map.tile_at((p_x + 1, p_y)) {
                    self.map.add_creature(entities::Goblin::new(tile.pos));
                }
            }
        }

        if let event::Keycode::Left = keycode {
            if let Some(tile) = self.map.tile_at((self.player.pos.x - 1, self.player.pos.y)) {
                if let Some(pos) = self.map
                    .monsters
                    .iter_mut()
                    .position(|m| m.pos() == tile.pos)
                {
                    let dead = {
                        let monster = self.map.monsters.get_mut(pos).unwrap();

                        let m_prev_hp = monster.hp();
                        let p_prev_hp = self.player.hp;

                        self.player.deal_damage(&mut **monster);

                        self.events.push(Event::new(
                            format!("Monster took {} damage.", m_prev_hp - monster.hp()),
                            EventType::Combat,
                        ));
                        monster.deal_damage(&mut self.player);

                        self.events.push(Event::new(
                            format!("Player took {} damage.", p_prev_hp - self.player.hp),
                            EventType::Combat,
                        ));

                        monster.is_dead()
                    };

                    if dead {
                        self.map.monsters.remove(pos);
                        self.events
                            .push(Event::new(String::from("Monster died."), EventType::Combat));
                    }
                } else if tile.tile_type.is_walkable_tile() {
                    self.player.pos.x -= 1;

                    let pos = self.player.pos;

                    while let Some(pos) = self.map.items.iter().position(|(p, _)| p == &pos) {
                        self.player.inv.push(self.map.items.remove(pos).1);
                        self.events
                            .push(Event::new(String::from("Picked up item."), EventType::Item));
                    }
                }
            }
        } else if let event::Keycode::Right = keycode {
            if let Some(tile) = self.map.tile_at((self.player.pos.x + 1, self.player.pos.y)) {
                if let Some(pos) = self.map
                    .monsters
                    .iter_mut()
                    .position(|m| m.pos() == tile.pos)
                {
                    let dead = {
                        let monster = self.map.monsters.get_mut(pos).unwrap();

                        let m_prev_hp = monster.hp();
                        let p_prev_hp = self.player.hp;

                        self.player.deal_damage(&mut **monster);

                        self.events.push(Event::new(
                            format!("Monster took {} damage.", m_prev_hp - monster.hp()),
                            EventType::Combat,
                        ));
                        monster.deal_damage(&mut self.player);

                        self.events.push(Event::new(
                            format!("Player took {} damage.", p_prev_hp - self.player.hp),
                            EventType::Combat,
                        ));
                        monster.is_dead()
                    };

                    if dead {
                        self.map.monsters.remove(pos);
                        self.events
                            .push(Event::new(String::from("Monster died."), EventType::Combat));
                    }
                } else if tile.tile_type.is_walkable_tile() {
                    self.player.pos.x += 1;

                    let pos = self.player.pos;

                    while let Some(pos) = self.map.items.iter().position(|(p, _)| p == &pos) {
                        self.player.inv.push(self.map.items.remove(pos).1);
                        self.events
                            .push(Event::new(String::from("Picked up item."), EventType::Item));
                    }
                }
            }
        } else if let event::Keycode::Up = keycode {
            if let Some(tile) = self.map.tile_at((self.player.pos.x, self.player.pos.y - 1)) {
                if let Some(pos) = self.map
                    .monsters
                    .iter_mut()
                    .position(|m| m.pos() == tile.pos)
                {
                    let dead = {
                        let monster = self.map.monsters.get_mut(pos).unwrap();

                        let m_prev_hp = monster.hp();
                        let p_prev_hp = self.player.hp;

                        self.player.deal_damage(&mut **monster);

                        self.events.push(Event::new(
                            format!("Monster took {} damage.", m_prev_hp - monster.hp()),
                            EventType::Combat,
                        ));
                        monster.deal_damage(&mut self.player);

                        self.events.push(Event::new(
                            format!("Player took {} damage.", p_prev_hp - self.player.hp),
                            EventType::Combat,
                        ));
                        monster.is_dead()
                    };

                    if dead {
                        self.map.monsters.remove(pos);
                        self.events
                            .push(Event::new(String::from("Monster died."), EventType::Combat));
                    }
                } else if tile.tile_type.is_walkable_tile() {
                    self.player.pos.y -= 1;

                    let pos = self.player.pos;

                    while let Some(pos) = self.map.items.iter().position(|(p, _)| p == &pos) {
                        self.player.inv.push(self.map.items.remove(pos).1);
                        self.events
                            .push(Event::new(String::from("Picked up item."), EventType::Item));
                    }
                }
            }
        } else if let event::Keycode::Down = keycode {
            if let Some(tile) = self.map.tile_at((self.player.pos.x, self.player.pos.y + 1)) {
                if let Some(pos) = self.map
                    .monsters
                    .iter_mut()
                    .position(|m| m.pos() == tile.pos)
                {
                    let dead = {
                        let monster = self.map.monsters.get_mut(pos).unwrap();

                        let m_prev_hp = monster.hp();
                        let p_prev_hp = self.player.hp;

                        self.player.deal_damage(&mut **monster);

                        self.events.push(Event::new(
                            format!("Monster took {} damage.", m_prev_hp - monster.hp()),
                            EventType::Combat,
                        ));
                        monster.deal_damage(&mut self.player);

                        self.events.push(Event::new(
                            format!("Player took {} damage.", p_prev_hp - self.player.hp),
                            EventType::Combat,
                        ));
                        monster.is_dead()
                    };

                    if dead {
                        self.map.monsters.remove(pos);
                        self.events
                            .push(Event::new(String::from("Monster died."), EventType::Combat));
                    }
                } else if tile.tile_type.is_walkable_tile() {
                    self.player.pos.y += 1;

                    let pos = self.player.pos;

                    while let Some(pos) = self.map.items.iter().position(|(p, _)| p == &pos) {
                        self.player.inv.push(self.map.items.remove(pos).1);
                        self.events
                            .push(Event::new(String::from("Picked up item."), EventType::Item));
                    }
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
