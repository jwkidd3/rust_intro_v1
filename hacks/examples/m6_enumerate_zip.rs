// cargo run --example m6_enumerate_zip

fn main() {
    let v = vec!["a", "b", "c"];

    // enumerate
    for (i, val) in v.iter().enumerate() {
        println!("{}: {}", i, val);
    }

    // zip
    let names = vec!["Alice", "Bob"];
    let ages = vec![30, 25];
    for (name, age) in names.iter().zip(ages.iter()) {
        println!("{} is {}", name, age);
    }
}
