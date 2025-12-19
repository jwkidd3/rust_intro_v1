// cargo run --example m6_map_filter

fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled);

    let evens: Vec<&i32> = v.iter().filter(|x| *x % 2 == 0).collect();
    println!("Evens: {:?}", evens);

    // Chained
    let result: i32 = v.iter()
        .filter(|x| *x % 2 == 0)
        .map(|x| x * x)
        .sum();
    println!("Sum of squared evens: {}", result);
}
