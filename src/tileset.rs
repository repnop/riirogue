use ggez::{
    graphics::{self, spritebatch::SpriteBatch, DrawParam, Image, Point2, Rect}, Context, GameResult,
};

use std::collections::HashMap;

pub struct TileSet {
    tile_size: (f32, f32),
    dimensions: (f32, f32),
    tile_names: HashMap<&'static str, (f32, f32)>,
    scale_factor: f32,
    sprite_batch: SpriteBatch,
}

impl TileSet {
    pub fn new(
        image: Image,
        dimensions: (i32, i32),
        tile_size: (i32, i32),
        scale_factor: f32,
    ) -> TileSet {
        let sprite_batch = SpriteBatch::new(image);

        TileSet {
            tile_size: (tile_size.0 as f32, tile_size.1 as f32),
            dimensions: (dimensions.0 as f32, dimensions.1 as f32),
            tile_names: HashMap::new(),
            scale_factor,
            sprite_batch,
        }
    }

    pub fn register_tile(&mut self, name: &'static str, coords: (i32, i32)) -> Result<(), ()> {
        if coords.0 as f32 > self.dimensions.0 || coords.1 as f32 > self.dimensions.1 {
            return Err(());
        }

        self.tile_names
            .insert(name, (coords.0 as f32, coords.1 as f32));

        Ok(())
    }

    pub fn queue_tile<'a>(
        &mut self,
        name: &'a str,
        coords: (i32, i32),
        color: Option<graphics::Color>,
    ) -> Result<(), &'a str> {
        let tile = self.tile_names.get(name).ok_or(name)?;
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
                coords.0 as f32 * self.tile_size.0 * self.scale_factor,
                coords.1 as f32 * self.tile_size.1 * self.scale_factor,
            ),
            scale: Point2::new(self.scale_factor, self.scale_factor),
            color,
            ..Default::default()
        });

        Ok(())
    }

    pub fn queue_rect<'a>(
        &mut self,
        name: &'a str,
        origin: (i32, i32),
        size: (i32, i32),
        color: Option<graphics::Color>,
    ) -> Result<(), &'a str> {
        let tile = self.tile_names.get(name).ok_or(name)?;
        let size_tile_x = 1.0 / self.dimensions.0;
        let size_tile_y = 1.0 / self.dimensions.1;

        for i in 0..=size.0 {
            for j in 0..=size.1 {
                self.sprite_batch.add(DrawParam {
                    src: Rect::new(
                        size_tile_x * tile.0,
                        size_tile_y * tile.1,
                        size_tile_x,
                        size_tile_y,
                    ),
                    dest: Point2::new(
                        (origin.0 + i) as f32 * self.tile_size.0 * self.scale_factor,
                        (origin.1 + j) as f32 * self.tile_size.1 * self.scale_factor,
                    ),
                    scale: Point2::new(self.scale_factor, self.scale_factor),
                    color,
                    ..Default::default()
                });
            }
        }

        Ok(())
    }

    pub fn clear_queue(&mut self) {
        self.sprite_batch.clear();
    }

    pub fn render(&self, ctx: &mut Context) -> GameResult<()> {
        graphics::draw_ex(ctx, &self.sprite_batch, Default::default())
    }
}
