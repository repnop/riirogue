extern crate rand;

pub use rand::{distributions::Uniform, thread_rng, Rng};
use std::ops::Range;

#[derive(Debug, Clone, Copy)]
pub enum TileType {
    Path,
    Room,
    Door,
    Wall,
}

pub fn clamp<T: PartialOrd>(x: T, min: T, max: T) -> T {
    if x > max {
        max
    } else if x < min {
        min
    } else {
        x
    }
}

/// Based on the SDL2 `Rect` struct.
#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Rect {
        Rect {
            x,
            y,
            width,
            height,
        }
    }

    pub fn intersects(&self, other: &Rect) -> bool {
        !(self.left() > other.right() || self.right() < other.left() || self.top() > other.bottom()
            || self.bottom() < other.top())
    }

    pub fn intersects_with_buffer(&self, other: &Rect, buffer: u32) -> bool {
        self.buffer(buffer).intersects(&other.buffer(buffer))
    }

    pub fn buffer(&self, buffer: u32) -> Rect {
        Rect::new(
            clamp(self.x - buffer, 0, self.x),
            clamp(self.y - buffer, 0, self.y),
            self.width + buffer,
            self.height + buffer,
        )
    }

    pub fn x(&self) -> u32 {
        self.x
    }

    pub fn left(&self) -> u32 {
        self.x
    }

    pub fn right(&self) -> u32 {
        self.x + self.width
    }

    pub fn y(&self) -> u32 {
        self.y
    }

    pub fn top(&self) -> u32 {
        self.y
    }

    pub fn bottom(&self) -> u32 {
        self.y + self.height
    }

    pub fn random_rect(
        rng: &mut impl Rng,
        x_uniform: &Uniform<u32>,
        y_uniform: &Uniform<u32>,
        w_uniform: &Uniform<u32>,
        h_uniform: &Uniform<u32>,
    ) -> Rect {
        use rand::distributions::{Distribution, Normal};

        Rect {
            x: x_uniform.sample(rng),
            y: y_uniform.sample(rng),
            width: w_uniform.sample(rng),
            height: h_uniform.sample(rng),
        }
    }
}

pub type Room = Rect;
pub type Rooms = Vec<Room>;

pub fn gen_rooms(
    map_size: (usize, usize),
    width_range: Range<u32>,
    height_range: Range<u32>,
    scale_factor: f32,
) -> Rooms {
    const MAX_TRIES: usize = 10000;

    let mut current_tries = 0;

    let mut rooms = Vec::new();

    let true_size = (
        (map_size.0 as f32 / scale_factor) as usize,
        (map_size.1 as f32 / scale_factor) as usize,
    );

    let mut avg_x = 0.0f32;
    let mut avg_y = 0.0f32;
    let mut count = 0.0f32;
    let offset = 2;

    let x_uniform = Uniform::new(offset, true_size.0 as u32 - offset - width_range.end);
    let y_uniform = Uniform::new(offset, true_size.1 as u32 - offset - height_range.end);
    let w_uniform = Uniform::new(width_range.start, width_range.end);
    let h_uniform = Uniform::new(height_range.start, height_range.end);
    let mut rng = thread_rng();

    while current_tries < MAX_TRIES {
        //let room = Rect::random_rect(2, true_size, width_range.clone(), height_range.clone());
        let room = Rect::random_rect(&mut rng, &x_uniform, &y_uniform, &w_uniform, &h_uniform);

        if rooms
            .iter()
            .any(|r: &Rect| r.intersects_with_buffer(&room, 2))
        {
            count += 1.0;
            avg_x += room.x as f32;
            avg_y += room.y as f32;

            //println!("{:?} COLLISION", room);
            current_tries += 1;
        } else {
            current_tries = 0;
            rooms.push(room);
        }
    }

    println!("Avg. X: {}, Avg. Y: {}", avg_x / count, avg_y / count);

    rooms
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersects() {
        let rect = Rect::new(0, 0, 5, 5);
        assert!(rect.intersects(&rect));
        assert!(rect.intersects(&Rect::new(2, 2, 5, 5)));
        assert!(Rect::new(0, 0, 10, 10).intersects(&Rect::new(2, 2, 10, 10)));
    }
}
