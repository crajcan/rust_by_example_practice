#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

struct Nil;

struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}


fn rect_area(rect: Rectangle) -> f32 {
    let Rectangle { p1: Point { x: x1, y: y1 },
                    p2: Point { x: x2, y: y2 } }  = rect;
    println!("x1: {}", x1);
    println!("x2: {}", x2);
    println!("y1: {}", y1);
    println!("y2: {}", y2);
    println!("x1 - x2: {:?}", x1 - x2);
    println!("y1 - y2: {:?}", y1 - y2);
    ((x1 - x2) * (y1 - y2))
}

fn main() {
    // Create struct with field init shorthand
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    // Print debug struct
    println!("{:?}", peter);

    let point: Point = Point { x: 0.3, y: 0.4 };
    println!("point coordinates: ({}, {})", point.x, point.y);
    
    // Make a new point by using struct update syntax to use the fields of our other one
    let new_point = Point { x: 0.2, ..point }; 
    println!("second point: ({}, {})", new_point.x, new_point.y);

    // destructure a point using a let binding
    let Point { x: my_x, y: my_y } = point;
    println!("my_x: {}", my_x);
    println!("my_y: {}", my_y);
    
    let _rectangle = Rectangle {
        p1: Point { x: my_y, y: my_x}, 
        p2: point
    }; 

    println!("_rectangle == {:?}", _rectangle);

    let _nil = Nil;

    let pair = Pair(1, 0.1);

    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;
    println!("pair contains {:?}, and {:?}", integer, decimal);

    println!("_rectangle: {:?}", _rectangle);
    println!("rect_area(&_rectangle) = {:?}", rect_area(_rectangle));
}
