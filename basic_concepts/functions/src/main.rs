fn main() {
    println!("Hello, world!");
    a_function();
    a_function_with_params(6);

    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
    println!("The value of x is: {}", x);

    let z = a_function_return_value();

    println!("The value of z is: {}", z);
}

fn a_function() {
    println!("A function.");
}

fn a_function_with_params(x: i32) {
    println!("Your number is {}", x)
}

fn a_function_return_value() -> i32 {
    5
}
