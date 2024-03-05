fn main() {
    let mut rect1 = Rectangle { dimensions: Vector2D(2.0, 3.0), position: Vector2D(0.0, 0.0) };
    rect1.print_area();

    rect1.scale(2.0);
    rect1.print_area();

    rect1.print_position();
    rect1.translate(-1.0, 2.0);
    rect1.print_position();

    let rect2 = Rectangle { dimensions: Vector2D(1.0, 4.0), position: Vector2D(0.0, -1.0) };
    println!("Rectangles intersect: {}", rect1.intersects(&rect2));

    println!("Rectangle 1 can hold Rectangle 2?: {}", rect1.can_hold(&rect2));

    let rect3 = Rectangle::square(2.5);
    println!("Area of square: {:.1}", rect3.area());
}

struct Rectangle {
    dimensions: Vector2D,
    position: Vector2D,
}

struct Vector2D(f32, f32);

impl Rectangle {
    fn area(&self) -> f32 {
        self.dimensions.0 * self.dimensions.1
    }

    fn scale(&mut self, factor: f32) {
        self.dimensions.0 *= factor;
        self.dimensions.1 *= factor;
    }

    fn translate(&mut self, x: f32, y: f32) {
        self.position.0 += x;
        self.position.1 += y;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.dimensions.0 > other.dimensions.0 && self.dimensions.1 > other.dimensions.1
    }

    fn intersects(&self, other: &Rectangle) -> bool {
        self.position.0 < other.position.0 + (other.dimensions.0 as f32) &&
            self.position.0 + (self.dimensions.0 as f32) > other.position.0 &&
            self.position.1 < other.position.1 + (other.dimensions.1 as f32) &&
            self.position.1 + (self.dimensions.1 as f32) > other.position.1
    }

    // Self is just an alias for the implemented type
    fn square(size: f32) -> Self {
        Self { dimensions: Vector2D(size, size), position: Vector2D(0.0, 0.0) }
    }

    fn print_area(&self) {
        println!("The area of the rectangle is {}.", self.area());
    }

    fn print_position(&self) {
        println!(
            "The position of the rectangle is x: {}, y: {}.",
            self.position.0,
            self.position.1
        );
    }
}
