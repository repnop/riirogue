use ggez::{
    graphics::{self, spritebatch::SpriteBatch, DrawParam, Image, Point2, Rect}, Context, GameResult,
};

use std::collections::HashMap;

pub struct TileSet {
    tile_size: (f32, f32),
    dimensions: (f32, f32),
    tile_names: HashMap<&'static str, (f32, f32)>,
    sprite_batch: SpriteBatch,
}

impl TileSet {
    pub fn new(image: Image, dimensions: (u32, u32), tile_size: (u32, u32)) -> TileSet {
        let sprite_batch = SpriteBatch::new(image);

        TileSet {
            tile_size: (tile_size.0 as f32, tile_size.1 as f32),
            dimensions: (dimensions.0 as f32, dimensions.1 as f32),
            tile_names: HashMap::new(),
            sprite_batch,
        }
    }

    pub fn register_tile(&mut self, name: &'static str, coords: (u32, u32)) -> Result<(), ()> {
        if coords.0 as f32 > self.dimensions.0 || coords.1 as f32 > self.dimensions.1 {
            return Err(());
        }

        self.tile_names
            .insert(name, (coords.0 as f32, coords.1 as f32));

        Ok(())
    }

    pub fn queue_tile(&mut self, name: &'static str, coords: (u32, u32)) -> Result<(), ()> {
        let tile = self.tile_names.get(name).ok_or(())?;
        let size_tile_x = 1.0 / self.dimensions.0;
        let size_tile_y = 1.0 / self.dimensions.1;

        self.sprite_batch.add(DrawParam {
            src: Rect::new(
                size_tile_x * tile.0,
                size_tile_y * tile.1,
                size_tile_x,
                size_tile_y,
            ),
            dest: Point2::new(
                coords.0 as f32 * self.tile_size.0,
                coords.1 as f32 * self.tile_size.1,
            ),
            ..Default::default()
        });

        Ok(())
    }

    pub fn clear_queue(&mut self) {
        self.sprite_batch.clear();
    }

    pub fn render(&self, ctx: &mut Context) -> GameResult<()> {
        graphics::draw_ex(ctx, &self.sprite_batch, Default::default())
    }
}
