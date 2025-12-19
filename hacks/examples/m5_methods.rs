// cargo run --example m5_methods

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn double(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }
}

fn main() {
    let mut rect = Rectangle::new(30, 50);
    println!("{:?}, Area: {}", rect, rect.area());

    rect.double();
    println!("After double: {:?}", rect);
}
