pub struct Circle {
    pub center: super::Point,
    pub radius: f64,
}

impl Circle {
    pub fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    pub fn circumference(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}
