// cargo run --example m5_generics
//
// Demonstrates generics - writing code that works with multiple types.
// Generics let you write flexible, reusable code without sacrificing type safety.

// Generic function: <T> declares a type parameter
// T: PartialOrd means T must implement comparison operators
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Generic struct: works with any type T
struct Point<T> {
    x: T,
    y: T,  // Note: both fields must be same type T
}

// Generic struct with multiple type parameters
struct Pair<T, U> {
    first: T,
    second: U,  // Can be different types
}

// Implement methods on generic struct
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Implement methods only for specific types
impl Point<f64> {
    // This method only exists for Point<f64>
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    // Generic function works with different types
    let numbers = vec![34, 50, 25, 100];
    println!("Largest number: {}", largest(&numbers));

    let chars = vec!['a', 'z', 'm'];
    println!("Largest char: {}", largest(&chars));

    // Generic struct with integers
    let int_point = Point { x: 5, y: 10 };
    println!("\nInteger point: ({}, {})", int_point.x, int_point.y);
    println!("x value: {}", int_point.x());

    // Generic struct with floats
    let float_point = Point { x: 1.5, y: 4.2 };
    println!("Float point: ({}, {})", float_point.x, float_point.y);
    // This method only exists for Point<f64>
    println!("Distance from origin: {:.2}", float_point.distance_from_origin());

    // Multiple type parameters
    let pair = Pair { first: 5, second: "hello" };
    println!("\nPair: ({}, {})", pair.first, pair.second);
}
