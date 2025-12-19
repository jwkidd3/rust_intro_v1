// cargo run --example m5_generics

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let numbers = vec![34, 50, 25, 100];
    println!("Largest: {}", largest(&numbers));

    let int_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };

    println!("Int: ({}, {})", int_point.x, int_point.y);
    println!("Float: ({}, {})", float_point.x, float_point.y);
}
