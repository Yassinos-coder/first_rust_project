#[derive(Debug)]
enum Shape {
    Circle(f64),
    Square(f64),
    Triangle(f64, f64),
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Square(length) => length * length,
            Shape::Triangle(base, height) => 0.5 * base * height,
        }
    }
}

fn take_vector(shapes: &[Shape]) -> f64 {
    shapes.iter().map(Shape::area).sum()
}

fn largest_shape(shapes: Vec<Shape>) -> Option<Shape> {
    shapes.into_iter().max_by(|first, second| {
        first.area().total_cmp(&second.area())
    })
}

fn main() {
    let shapes = vec![Shape::Circle(5.0), Shape::Square(3.0), Shape::Triangle(4.0, 6.0)];

    let total_area = take_vector(&shapes);

    println!("Total area: {} sq. units", total_area);

    if let Some(shape) = largest_shape(shapes) {
        println!("Largest shape: {:?}, area: {} sq. units", shape, shape.area());
    }
}
