// cargo run --example m6_iterators

fn main() {
    let v = vec![1, 2, 3];

    // iter() - borrows
    for val in v.iter() {
        println!("{}", val);
    }

    // iter_mut() - mutable borrow
    let mut v2 = vec![1, 2, 3];
    for val in v2.iter_mut() {
        *val *= 2;
    }
    println!("Doubled: {:?}", v2);

    // into_iter() - takes ownership
    for val in v.into_iter() {
        println!("{}", val);
    }
}
