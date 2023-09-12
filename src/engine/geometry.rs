#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}

#[derive(Clone, Copy, Default)]
pub struct Shape {
    pub width: i16,
    pub height: i16,
}

#[derive(Default, Clone)]
pub struct Rect {
    origin: Point,
    shape: Shape,
}

impl Rect {
    pub const fn new(origin: Point, shape: Shape) -> Self {
        Rect { origin, shape }
    }

    pub const fn new_from_x_y_w_h(x: i16, y: i16, width: i16, height: i16) -> Self {
        Rect::new(Point { x, y }, Shape { width, height })
    }

    pub fn intersects(&self, rect: &Rect) -> bool {
        (self.x() < (rect.right()))
            && (rect.x() < (self.right()))
            && (self.y() < (rect.bottom()))
            && (rect.y() < (self.bottom()))
    }

    pub fn x(&self) -> i16 {
        self.origin.x
    }

    pub fn y(&self) -> i16 {
        self.origin.y
    }

    pub fn width(&self) -> i16 {
        self.shape.width
    }

    pub fn height(&self) -> i16 {
        self.shape.height
    }

    pub fn left(&self) -> i16 {
        self.origin.x
    }

    pub fn top(&self) -> i16 {
        self.origin.y
    }

    pub fn right(&self) -> i16 {
        self.x() + self.width()
    }

    pub fn bottom(&self) -> i16 {
        self.y() + self.height()
    }

    pub fn set_x(&mut self, x: i16) {
        self.origin.x = x
    }
    pub fn set_y(&mut self, y: i16) {
        self.origin.y = y
    }
}
