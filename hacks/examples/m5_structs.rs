// cargo run --example m5_structs
//
// Demonstrates structs - custom data types that group related values.
// Structs are similar to classes in other languages (but without inheritance).

// Define a struct with named fields
struct User {
    username: String,
    email: String,
    active: bool,
}

fn main() {
    // Create an instance of the struct
    // All fields must be initialized
    let user1 = User {
        email: String::from("user@example.com"),
        username: String::from("someuser"),
        active: true,
    };

    // Access fields with dot notation
    println!("User: {} ({})", user1.username, user1.email);
    println!("Active: {}", user1.active);

    // Struct Update Syntax: create new struct reusing fields from another
    // The ..user1 copies remaining fields from user1
    let user2 = User {
        email: String::from("other@example.com"),  // New email
        ..user1  // Copy username and active from user1
    };
    // Note: user1.username was moved to user2, so user1 is partially invalid now

    println!("\nUser2: {} ({})", user2.username, user2.email);

    // Mutable struct - the entire struct must be marked mut
    let mut user3 = User {
        email: String::from("mutable@example.com"),
        username: String::from("mutableuser"),
        active: false,
    };

    // Now we can modify fields
    user3.active = true;
    user3.email = String::from("updated@example.com");
    println!("\nUser3 (modified): {} - active: {}", user3.email, user3.active);
}
