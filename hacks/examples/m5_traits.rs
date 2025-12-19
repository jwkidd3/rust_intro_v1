// cargo run --example m5_traits
//
// Demonstrates traits - Rust's way of defining shared behavior.
// Similar to interfaces in Java or protocols in Swift.

// Define a trait: a collection of methods that types can implement
trait Summary {
    // Required method - implementors must provide this
    fn summarize(&self) -> String;
}

// A struct that will implement Summary
struct Article {
    title: String,
    author: String,
    _content: String,
}

// Another struct that will implement Summary differently
struct Tweet {
    username: String,
    _content: String,
}

// Implement the Summary trait for Article
impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.title, self.author)
    }
}

// Implement the Summary trait for Tweet
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self._content)
    }
}

// Function that accepts any type implementing Summary
fn print_summary(item: &impl Summary) {
    println!("Summary: {}", item.summarize());
}

fn main() {
    let article = Article {
        title: String::from("Breaking News"),
        author: String::from("John"),
        _content: String::from("Something happened today..."),
    };

    let tweet = Tweet {
        username: String::from("rustacean"),
        _content: String::from("Hello Rust world!"),
    };

    // Both types can use the same method name
    println!("Article: {}", article.summarize());
    println!("Tweet: {}", tweet.summarize());

    // Both can be passed to functions expecting Summary
    println!();
    print_summary(&article);
    print_summary(&tweet);
}
