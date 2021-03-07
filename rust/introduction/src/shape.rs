pub trait Shape { fn area(&self) -> f64; }

pub struct Rect {
    pub width: f64,
    pub height: f64
}
impl Shape for Rect {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

pub struct Triangle {
    pub base: f64,
    pub height: f64
}
impl Shape for Triangle {
    fn area(&self) -> f64 {
        self.base * self.height * 0.5
    }
}

pub struct Circle {
    pub radius: f64
}
impl Shape for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }
}
