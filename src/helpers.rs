use rand::{distributions::Uniform, Rng};

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Rect {
        Rect {
            x,
            y,
            width,
            height,
        }
    }

    #[allow(dead_code)]
    pub fn contains(&self, (x, y): (i32, i32)) -> bool {
        (x >= self.left() && x <= self.right()) && (y >= self.top() && y <= self.bottom())
    }

    pub fn center(&self) -> (i32, i32) {
        (self.x + self.width / 2, self.y + self.height / 2)
    }

    pub fn intersects(&self, other: &Rect) -> bool {
        !(self.left() > other.right() || self.right() < other.left() || self.top() > other.bottom()
            || self.bottom() < other.top())
    }

    pub fn intersects_with_buffer(&self, other: &Rect, buffer: i32) -> bool {
        self.buffer(buffer).intersects(&other.buffer(buffer))
    }

    pub fn buffer(&self, buffer: i32) -> Rect {
        Rect::new(
            clamp(self.x - buffer, 0, self.x),
            clamp(self.y - buffer, 0, self.y),
            self.width + buffer,
            self.height + buffer,
        )
    }

    pub fn left(&self) -> i32 {
        self.x
    }

    pub fn right(&self) -> i32 {
        self.x + self.width
    }

    pub fn top(&self) -> i32 {
        self.y
    }

    pub fn bottom(&self) -> i32 {
        self.y + self.height
    }

    pub fn random_rect(
        rng: &mut impl Rng,
        x_uniform: &Uniform<i32>,
        y_uniform: &Uniform<i32>,
        w_uniform: &Uniform<i32>,
        h_uniform: &Uniform<i32>,
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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Coords {
    pub x: i32,
    pub y: i32,
}

impl Coords {
    pub fn new(x: i32, y: i32) -> Coords {
        Coords { x, y }
    }
}

impl Coords {
    #[allow(dead_code)]
    fn distance(&self, other: Coords) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl From<(i32, i32)> for Coords {
    fn from((x, y): (i32, i32)) -> Coords {
        Coords { x, y }
    }
}

impl<'a> From<&'a (i32, i32)> for Coords {
    fn from(&(x, y): &(i32, i32)) -> Coords {
        Coords { x, y }
    }
}
