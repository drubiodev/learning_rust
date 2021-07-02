fn main() {
    let s1 = String::from("hello");
    let mut s = String::from("hello");
    let len = calculate_length(&s1); // creates a reference to s1 value but does not own it
    mutable_change(&mut s);
    println!("The length of '{}' is {}.", s1, len);

    let reference_to_nothing = dangle();
}

// - & reference
// - * dereference

fn calculate_length(s: &String) -> usize {
    s.len()
}

// cant modify something borrowed
fn change(some_string: &String) {
    //some_string.push_str(", world");
}

// mutable reference
fn mutable_change(some_string: &mut String) {
    some_string.push_str(", world");
}

// returns a reference to a String
fn dangle() -> &String {
    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
