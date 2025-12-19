// cargo run --example m5_trait_default
//
// Demonstrates default trait implementations.
// Traits can provide default method bodies that implementors can use or override.

// Trait with a required method and a default implementation
trait Summary {
    // Required: implementors MUST provide this
    fn summarize_author(&self) -> String;

    // Default implementation: implementors CAN override this
    // The default calls summarize_author(), so it works automatically
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

struct Article {
    title: String,
    author: String,
}

struct Tweet {
    username: String,
    _content: String,
}

// Article: provides summarize_author, OVERRIDES default summarize
impl Summary for Article {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }

    // Override the default implementation
    fn summarize(&self) -> String {
        format!("{}, by {}", self.title, self.author)
    }
}

// Tweet: provides summarize_author, USES default summarize
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    // No summarize() - uses the default implementation
}

fn main() {
    println!("=== Default Trait Implementations ===\n");

    let article = Article {
        title: String::from("Rust 2024 Edition Released"),
        author: String::from("Jane Developer"),
    };

    let tweet = Tweet {
        username: String::from("rustlang"),
        _content: String::from("Rust is awesome!"),
    };

    // Article uses its custom summarize
    println!("Article (custom summarize):");
    println!("  Author: {}", article.summarize_author());
    println!("  Summary: {}", article.summarize());

    // Tweet uses the default summarize
    println!("\nTweet (default summarize):");
    println!("  Author: {}", tweet.summarize_author());
    println!("  Summary: {}", tweet.summarize());

    println!("\nKey points:");
    println!("  - Default methods can call other trait methods");
    println!("  - Implementors can override defaults or use them as-is");
    println!("  - Only required methods must be implemented");
}
