// cargo run --example m5_structs

struct User {
    username: String,
    email: String,
    active: bool,
}

fn main() {
    let user1 = User {
        email: String::from("user@example.com"),
        username: String::from("someuser"),
        active: true,
    };
    println!("User: {} ({})", user1.username, user1.email);

    // Struct update syntax
    let user2 = User {
        email: String::from("other@example.com"),
        ..user1
    };
    println!("User2: {}", user2.email);
}
