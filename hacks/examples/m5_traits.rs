// cargo run --example m5_traits

trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    title: String,
    author: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.title, self.author)
    }
}

fn main() {
    let article = Article {
        title: String::from("Breaking News"),
        author: String::from("John"),
    };
    println!("{}", article.summarize());
}
