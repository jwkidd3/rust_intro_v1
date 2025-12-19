// cargo run --example m6_fold

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let sum = numbers.iter().fold(0, |acc, x| acc + x);
    println!("Sum: {}", sum);

    let product = numbers.iter().fold(1, |acc, x| acc * x);
    println!("Product: {}", product);

    let words = vec!["hello", "world"];
    let sentence = words.iter().fold(String::new(), |acc, w| acc + w + " ");
    println!("Sentence: {}", sentence.trim());
}
