pub struct Rectangle {
    pub bottom_left: super::Point,
    pub top_right: super::Point,
}

impl Rectangle {
    pub fn width(&self) -> f64 {
        self.top_right.0 - self.bottom_left.0
    }

    pub fn height(&self) -> f64 {
        self.top_right.1 - self.bottom_left.1
    }

    pub fn area(&self) -> f64 {
        self.width() * self.height()
    }

    pub fn perimeter(&self) -> f64 {
        self.width() * 2.0 + self.height() * 2.0
    }

    pub fn square(bottom_left: super::Point, size: f64) -> Self {
        let top_right = super::Point(bottom_left.0 + size, bottom_left.1 + size);
        Rectangle {
            bottom_left,
            top_right,
        }
    }
}
