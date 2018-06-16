use rand::{distributions::Uniform, Rng};

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

    pub fn contains(&self, (x, y): (u32, u32)) -> bool {
        (x >= self.left() && x <= self.right()) && (y >= self.top() && y <= self.bottom())
    }

    pub fn center(&self) -> (u32, u32) {
        (self.x + self.width / 2, self.y + self.height / 2)
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
        use rand::distributions::Distribution;

        Rect {
            x: x_uniform.sample(rng),
            y: y_uniform.sample(rng),
            width: w_uniform.sample(rng),
            height: h_uniform.sample(rng),
        }
    }
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
