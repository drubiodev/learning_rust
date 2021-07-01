fn main() {
    // s is not valid here, it’s not yet declared
    let s = "hello"; // s is valid from this point forward (string literal)

    string_type();
    variables_move();
} // this scope is now over, and s is no longer valid

// stored on heap
fn string_type() {
    //let s = String::from("Hello");
    let mut s = String::from("Hello");
    s.push_str(", world");

    print!("{}", s);
}

fn variables_move() {
    let x = 5;
    let y = x; // makes a copy of x binds to y

    // String has 3 parts
    //  - pointer to the memory that holds contents of string
    //  - a length (how much memory in bytes)
    //  - a capacity

    let mut s1 = String::from("hello");
    let s2 = s1; // copies pointer, length, and capicty (moves s1 to s2)

    // println!("{}, world!", s1); // s1 is no longer valid and goes out of scope
}

fn variables_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}
