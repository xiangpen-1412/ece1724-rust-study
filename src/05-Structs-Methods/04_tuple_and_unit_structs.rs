#[derive(Debug)]
struct Point(i32, i32, i32);
struct Color(i32, i32, i32);
struct Points(String, String);

struct Market;

fn main() {
    // Practice 04: Compare tuple structs, distinct named types, and unit-like structs.
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let Color(r, g, b) = black;

    // destructure black not move
    println!("{}", black.0);

    let copy = black;
    // note: thought all variables are movable, the whole struct is not
    // println!("{}", black.0);

    dbg!(&origin);
    println!("{:?}", origin);

    let points = Points(String::from("first"), String::from("last"));
    let Points(first, last) = points;
    // note: value used after being moved
    // println!("{} {}", points.0, points.1);
}
