// cargo run --example m6_find_any_all

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let first_even = numbers.iter().find(|x| *x % 2 == 0);
    println!("First even: {:?}", first_even);

    let has_even = numbers.iter().any(|x| x % 2 == 0);
    println!("Has even: {}", has_even);

    let all_positive = numbers.iter().all(|x| *x > 0);
    println!("All positive: {}", all_positive);
}
