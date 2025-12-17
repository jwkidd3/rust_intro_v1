fn main() {
    // Array: fixed length, same type
    let a = [1, 2, 3, 4, 5];

    // With type annotation
    let b: [i32; 5] = [1, 2, 3, 4, 5];

    // Initialize with same value
    let c = [3; 5];  // [3, 3, 3, 3, 3]

    // Access elements
    let first = a[0];
    let second = a[1];

    println!("Array a: {:?}", a);
    println!("Array b: {:?}", b);
    println!("Array c: {:?}", c);
    println!("First: {}, Second: {}", first, second);

    // Array length
    println!("Length of a: {}", a.len());

    // Access all elements (iteration covered in Lab 3)
    println!("All elements: {}, {}, {}, {}, {}", a[0], a[1], a[2], a[3], a[4]);
}