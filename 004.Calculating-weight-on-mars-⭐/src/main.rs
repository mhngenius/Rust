fn main() {
    println!("Your weight on mars: {}", calculating_weight_on_mars(90.0)); // hardcoded = can't change
    println!("Your weight on mars: {}", calculating_weight_on_mars2());

    let mut a = 1337; // variables are immutable in Rust by default so we can make them mutable by addongthe "mut" keyword behind the variables name
    println!("{}", a);
    a = 42;
    println!("{}", a);
}

// declaring a function
fn calculating_weight_on_mars(weight: f32) -> f32 {
    // calculating weight on mars
    (weight * 3.73) / 9.81
}

fn calculating_weight_on_mars2() -> f32 /* declaring the return type (the type of the variable the gets returned) */
{
    return (90.0 * 3.73) / 9.81;
}

// an example of functions usage:
fn pi() -> f32 {
    3.14159 // if you remove the "return" keyword and the semicolon at the end it does the exact same thing as returning it
}
