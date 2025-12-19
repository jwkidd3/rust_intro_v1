// cargo run --example m6_closure_traits
//
// Demonstrates the three closure traits: Fn, FnMut, and FnOnce.
// These traits determine how a closure captures and uses its environment.

// Fn: borrows captured values immutably
// Can be called multiple times, doesn't modify captured values
fn call_fn<F: Fn()>(f: F) {
    println!("  Calling Fn closure twice:");
    f();
    f();
}

// FnMut: borrows captured values mutably
// Can be called multiple times, may modify captured values
fn call_fn_mut<F: FnMut()>(mut f: F) {
    println!("  Calling FnMut closure twice:");
    f();
    f();
}

// FnOnce: takes ownership of captured values
// Can only be called once (consumes captured values)
fn call_fn_once<F: FnOnce()>(f: F) {
    println!("  Calling FnOnce closure once:");
    f();
    // f();  // ERROR: can't call again, values were consumed
}

// Generic function accepting any closure that transforms i32
fn apply<F>(f: F, value: i32) -> i32
where
    F: Fn(i32) -> i32,  // F must be callable with i32 and return i32
{
    f(value)
}

fn main() {
    println!("=== Closure Traits ===\n");

    // Fn: closure only reads captured value
    println!("Fn trait (immutable borrow):");
    let message = String::from("Hello");
    let print_msg = || println!("    {}", message);
    call_fn(print_msg);
    println!("  message still valid: {}\n", message);

    // FnMut: closure modifies captured value
    println!("FnMut trait (mutable borrow):");
    let mut counter = 0;
    let increment = || {
        counter += 1;
        println!("    counter = {}", counter);
    };
    call_fn_mut(increment);
    println!("  final counter: {}\n", counter);

    // FnOnce: closure consumes captured value
    println!("FnOnce trait (takes ownership):");
    let data = vec![1, 2, 3];
    let consume_data = || {
        println!("    consuming: {:?}", data);
        drop(data);  // Explicitly drop to show ownership
    };
    call_fn_once(consume_data);
    // println!("{:?}", data);  // ERROR: data was moved into closure

    // Using closures with apply function
    println!("\nUsing generic closure parameter:");
    let double = |x| x * 2;
    let add_ten = |x| x + 10;

    println!("  apply(double, 5) = {}", apply(double, 5));
    println!("  apply(add_ten, 5) = {}", apply(add_ten, 5));

    println!("\nClosure trait hierarchy:");
    println!("  FnOnce - all closures implement this (callable at least once)");
    println!("  FnMut  - closures that don't consume values (extends FnOnce)");
    println!("  Fn     - closures that don't mutate values (extends FnMut)");
    println!("\n  Fn ⊂ FnMut ⊂ FnOnce");
}
