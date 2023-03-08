fn main() {
    scope_variable();
    string_heap();
    stack_vs_heap();
}

// String literal
fn scope_variable() {
    // s is not valid here
    let s = "hello"; // s is valid from here
                     // do stuff with s
    println!("String is {s}");
    // scope over, s is no longer valid
}
fn string_heap() {
    let mut s = String::from("Hello");
    s.push_str(", world");
    println!("{}", s);
}
fn stack_vs_heap() {
    // Handled on the stack
    let x = 5;
    let y = x;

    // Handled on the heap. Only pointer, length and capacity is stored on stack
    let s1 = String::from("hello");
    // This is a reference copy
    let s2 = s1;

    // s1 was moved to s2. (Almost shallow copy, but because Rust invalidated the first variable, it's known as a move)
    // println!("{}, world!", s1); // Invalid
    // This means we only need to free memory for s2. Problem solved..
    // Rust will never automatically deep copy data

    // However, both x and y are valid. stack-only data is copied.

    // Deep copy
    let s3 = s2.clone();
    println!("s2 = {s2}, s3 = {s3}");
}

fn ownership() {
    let s = String::from("hello"); // s comes into scope
    takes_ownership(s); // s's value moves into the function
                        // println!("{}", s);                   // so it is no longer valid here

    let x = 5; // x comes into scope
    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still use x afterwards.
} // Here, x goes out of scope, then s. But because s's value was moved, nothing special happens.

fn ownership_2() {
    let s1 = gives_ownership();

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing happens. s1 goes out of scope and is dropped

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
fn borrow() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    // Mutate borrowed value
    let mut s1 = String::from("hello");
    change(&mut s1);

    // But can only do one
    let mut s = String::from("hello");

    // Exception
    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);

    // But this is OK, as it is scoped
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    }
    let r2 = &mut s;

    let mut s = String::from("hello");
    let r1 = &s; // No problem
    let r2 = &s; // No problem
                 //let r3 = &mut s; // BIG PROBLEM
                 //println!("{}, {}, and {}", r1, r2, r3);
}
fn calculate_length(s: &String) -> usize {
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what it refers to, it is not dropped.

// fn change(some_string: &String) {
//     some_string.push_str(", world"); // Can't do this, you don't own it and it is not mutable.
// }

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// // Error, dangling is not OK
// fn dangle() -> &String { // Dangle returns a reference to a string
//     let s = String::from("hello"); // S is a new string
//     &s // We return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away. Danger!
// Solution, just return s. Ownership is moved out, and nothing is deallocated.
