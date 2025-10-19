pub struct Drone {
    x: usize,
    y: usize,
}

impl Drone {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    pub fn x(&self) -> usize {
        self.x
    }
    pub fn y(&self) -> usize {
        self.y
    }
    pub fn set_position(&mut self, x: usize, y: usize) -> &mut Self {
        self.x = x;
        self.y = y;
        self
    }
}
