use std::io; //importing input/output library

fn main() {
    print!("Enter your weight on earth (in kg): \n");
    let mut weight_str: String = String::new(); // creating a mutable string variable to store user input
    io::stdin().read_line(&mut weight_str).unwrap(); // error handling;

    //typecasting

    let weight_trimmed = weight_str.trim(); /* removing the spaces */
    let weight: f32 = weight_trimmed.parse().unwrap(); // parsing the string to float
                                                       // changes the weight_trimmed variable's type to the weight variable's type

    println!(
        "Your weight on mars will be: {}kg",
        calculating_weight_on_mars(weight)
    );
}

fn calculating_weight_on_mars(weight: f32) -> f32 {
    (weight * 3.73) / 9.81
}
