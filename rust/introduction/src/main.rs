use intro::shape::{Shape, Rect, Triangle, Circle};
use intro::ownership::try_ownership;

fn main() {
    // ownership move borrowing
    try_ownership();

    // struct trait impl
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Rect {width: 10.0, height: 5.0}),
        Box::new(Triangle {base: 10.0, height: 5.0}),
        Box::new(Circle {radius: 5.0}),
    ];
    for shape in shapes {
        println!("area={}", shape.area());
    }
}
