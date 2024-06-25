enum Shape {
    Circle(f64),
    Square(f64),
    Triangle(f64, f64, f64), // Base, height and angle
}

// a function to display Shape
impl std::fmt::Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Shape::Circle(_radius) => write!(f, "Circle"),
            Shape::Square(_length) => write!(f, "Square"),
            Shape::Triangle(_base, _height, _angle) => write!(f, "Triangle"),
        }
    }
}

// function to calculate the area of a shape
fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Square(length) => length * length,
        Shape::Triangle(base, height, angle) => 0.5 * *base * *height * angle.to_radians().sin(), // angle in radians
    }
}

// function that returns the largest shape in a vector of shapes
fn largest_shape(shapes: &Vec<Shape>) -> &Shape {
    let mut largest = &shapes[0]; // initialize largest with the first shape

    for shape in shapes {
        if area(shape) > area(largest) {
            largest = &shape;
        }
    }

    largest
}

fn main() {
    let shapes = vec![Shape::Circle(5.0), Shape::Square(3.0), Shape::Triangle(2.0, 3.5, 29.2)];

    let total_area: f64 = shapes
        .iter()
        .map(|shape| match shape {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Square(length) => length * length,
            Shape::Triangle(base, height, angle) => 0.5 * *base * *height * angle.to_radians().sin(), // angle in radians
        })
        .sum();

    println!("Total area: {} sq. units", total_area);

    let largest = largest_shape(&shapes);
    println!("{} with area {}", largest, area(largest));
}
