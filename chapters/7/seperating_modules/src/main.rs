pub mod shapes;

use shapes::rectangle::Rectangle;
use shapes::circle::Circle;

fn main() {
    let rectangle = Rectangle {
        bottom_left: shapes::Point(0.0, 0.0),
        top_right: shapes::Point(3.0, 4.0),
    };

    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let circle = Circle {
        center: shapes::Point(0.0, 0.0),
        radius: 2.0,
    };

    println!("Circle circumference: {}", circle.circumference());
    println!("Circle area: {}", circle.area());

    let square = Rectangle::square(shapes::Point(0.0, 0.0), 2.0);

    println!("Square perimeter: {}", square.perimeter());
    println!("Square area: {}", square.area());
}
