// cargo run --example m5_methods
//
// Demonstrates methods and associated functions using impl blocks.
// Methods are functions attached to a struct (or enum/trait).

// #[derive(Debug)] automatically implements the Debug trait
// allowing us to print the struct with {:?}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// impl block: where we define methods for Rectangle
impl Rectangle {
    // ASSOCIATED FUNCTION (no self parameter)
    // Called with :: syntax: Rectangle::new(30, 50)
    // Often used as constructors
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    // Another associated function - creates a square
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }

    // METHOD with &self - borrows the instance immutably
    // Called with dot syntax: rect.area()
    // Most common type - reads data but doesn't modify
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // METHOD with &mut self - borrows the instance mutably
    // Can modify the struct's fields
    fn double(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }

    // METHOD with self - takes ownership of the instance
    // The original instance is consumed (moved)
    // Useful for transformations that return a new value
    fn into_square(self) -> Rectangle {
        let side = self.width.max(self.height);
        Rectangle { width: side, height: side }
    }
}

fn main() {
    // Associated functions are called with ::
    let rect = Rectangle::new(30, 50);
    let sq = Rectangle::square(10);

    // Methods are called with dot notation
    println!("Rectangle: {:?}", rect);
    println!("Area: {}", rect.area());
    println!("\nSquare: {:?}, Area: {}", sq, sq.area());

    // &mut self method requires mutable variable
    let mut rect2 = Rectangle::new(5, 10);
    println!("\nBefore double: {:?}", rect2);
    rect2.double();  // Modifies rect2 in place
    println!("After double: {:?}", rect2);

    // self method consumes the original
    let rect3 = Rectangle::new(20, 30);
    let square = rect3.into_square();  // rect3 is moved here
    println!("\nConverted to square: {:?}", square);
    // println!("{:?}", rect3);  // ERROR: rect3 was moved
}
