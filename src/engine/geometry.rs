#[derive(Clone, Copy, Default)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}

impl Point {
    pub fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }
}

#[derive(Default)]
pub struct Rect {
    pub origin: Point,
    pub width: i16,
    pub height: i16,
}

impl Rect {
    pub const fn new(origin: Point, width: i16, height: i16) -> Self {
        Rect {
            origin,
            width,
            height,
        }
    }

    pub const fn new_from_x_y(x: i16, y: i16, width: i16, height: i16) -> Self {
        Rect::new(Point { x, y }, width, height)
    }

    pub fn intersects(&self, rect: &Rect) -> bool {
        (self.x() < (rect.x() + rect.width))
            && (rect.x() < (self.x() + self.width))
            && (self.y() < (rect.y() + rect.height))
            && (rect.y() < (self.y() + self.height))
    }

    pub fn x(&self) -> i16 {
        self.origin.x
    }

    pub fn y(&self) -> i16 {
        self.origin.y
    }

    pub fn set_x(&mut self, x: i16) {
        self.origin.x = x
    }
    pub fn set_y(&mut self, y: i16) {
        self.origin.y = y
    }

    pub fn left(&self) -> i16 {
        self.origin.x
    }

    pub fn top(&self) -> i16 {
        self.origin.y
    }

    pub fn right(&self) -> i16 {
        self.origin.x + self.width
    }

    pub fn bottom(&self) -> i16 {
        self.origin.y + self.height
    }
}
