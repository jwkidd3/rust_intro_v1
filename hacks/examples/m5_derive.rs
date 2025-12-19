// cargo run --example m5_derive

#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = p1.clone();

    println!("{:?}", p1);
    println!("{:#?}", p2);

    if p1 == p2 {
        println!("Points are equal!");
    }
}
