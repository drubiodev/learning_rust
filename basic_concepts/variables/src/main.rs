fn main() {
    // variables are immutable by default can change by using 'mut'
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // constants are always immutable and the type must be annotated.
    const MAX_POINTS: u32 = 100_000;

    // Shadowing You can declare new variable with same name
    // will get error if try to reassign without let keyword
    // with shadowing it basically creating a new variable wit let and gives us the option to change the type
    let y = 5;
    let y = y + 1; // takes original value and adds one (6)
    let y = y * 2; // this muliplies previous (6) by two (12)
    let spaces = "   "; // type &str
    let spaces = spaces.len(); // type i32

    println!("The value of y is: {}", y);
}
