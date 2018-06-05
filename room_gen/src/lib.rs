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

    pub fn intersects_with_buffer(
        &self,
        &Rect {
            x,
            y,
            width,
            height,
        }: &Rect,
        buffer: u32,
    ) -> bool {
        let this = Rect::new(
            clamp(self.left() - buffer, 0, self.left()),
            clamp(self.top() - buffer, 0, self.top()),
            self.right() + buffer,
            self.bottom() + buffer,
        );

        let other = Rect::new(
            x,      //clamp(x - buffer as i32, 0, x),
            y,      //clamp(y - buffer as i32, 0, y),
            width,  // + buffer,
            height, // + buffer,
        );

        this.intersects(&other)
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
        offset: u32,
        map_size: (usize, usize),
        width_range: Range<u32>,
        height_range: Range<u32>,
    ) -> Rect {
        let x = thread_rng().sample(Uniform::new(
            offset,
            map_size.0 as u32 - offset - width_range.end,
        ));
        let y = thread_rng().sample(Uniform::new(
            offset,
            map_size.1 as u32 - offset - height_range.end,
        ));

        Rect {
            x,
            y,
            width: thread_rng().sample(Uniform::new(width_range.start, width_range.end)),
            height: thread_rng().sample(Uniform::new(height_range.start, height_range.end)),
        }
    }
}

pub type Room = Rect;
pub type Rooms = Vec<Room>;

pub fn gen_rooms(
    map_size: (usize, usize),
    width_range: Range<u32>,
    height_range: Range<u32>,
) -> Rooms {
    const MAX_TRIES: usize = 10000;

    let mut current_tries = 0;

    let mut rooms = Vec::new();

    while current_tries < MAX_TRIES {
        let room = Rect::random_rect(2, map_size, width_range.clone(), height_range.clone());

        if rooms
            .iter()
            .any(|r: &Rect| r.intersects_with_buffer(&room, 2))
        {
            current_tries += 1;
        } else {
            current_tries = 0;
            rooms.push(room);
        }
    }

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
