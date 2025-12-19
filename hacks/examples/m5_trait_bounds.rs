// cargo run --example m5_trait_bounds
//
// Demonstrates trait bounds - constraining generic types to those
// that implement specific traits.

use std::fmt::Display;

// Define a simple trait for demonstration
trait Summary {
    fn summarize(&self) -> String;
}

#[derive(Clone)]
struct Article {
    title: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("Article: {}", self.title)
    }
}

impl Display for Article {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "'{}'", self.title)
    }
}

// TRAIT BOUND SYNTAX 1: <T: Trait>
// T must implement Summary
fn notify_bound<T: Summary>(item: &T) {
    println!("[bound syntax] {}", item.summarize());
}

// TRAIT BOUND SYNTAX 2: impl Trait (shorthand)
// Simpler syntax for single use
fn notify_impl(item: &impl Summary) {
    println!("[impl syntax] {}", item.summarize());
}

// MULTIPLE TRAIT BOUNDS: T must implement BOTH traits
fn notify_both<T: Summary + Display>(item: &T) {
    println!("[both traits] Display: {}, Summary: {}", item, item.summarize());
}

// WHERE CLAUSE: cleaner syntax for complex bounds
fn complex_function<T, U>(t: &T, u: &U) -> String
where
    T: Summary + Clone,
    U: Display,
{
    format!("T says: {}, U displays: {}", t.summarize(), u)
}

// RETURNING impl Trait: function returns some type implementing the trait
fn create_summarizable() -> impl Summary {
    Article {
        title: String::from("Dynamic Article"),
    }
}

fn main() {
    println!("=== Trait Bounds ===\n");

    let article = Article {
        title: String::from("Rust Trait Bounds Explained"),
    };

    // Different syntax styles, same result
    println!("Different trait bound syntaxes:");
    notify_bound(&article);
    notify_impl(&article);

    // Multiple bounds
    println!("\nMultiple trait bounds (Summary + Display):");
    notify_both(&article);

    // Where clause
    println!("\nWhere clause syntax:");
    let result = complex_function(&article, &"Hello");
    println!("  {}", result);

    // Returning impl Trait
    println!("\nReturning impl Trait:");
    let dynamic = create_summarizable();
    println!("  {}", dynamic.summarize());

    println!("\nSyntax summary:");
    println!("  fn foo<T: Trait>(x: &T)     - bound syntax");
    println!("  fn foo(x: &impl Trait)      - impl syntax (shorthand)");
    println!("  fn foo<T: A + B>(x: &T)     - multiple bounds");
    println!("  fn foo<T>(x: &T) where T: A - where clause");
    println!("  fn foo() -> impl Trait      - return impl trait");
}
