fn main() {
    let name = "mahan"; // to define a variable in Rust we write "let + the variable's name"
    println!("Hi my name is {name} "); // we can use a variable in a sentence by putting it in {}s or do this:
    println!("Hi my name is {}", name);

    let age = 18;

    let sentence = format!("My name is {name} and I am {age} years old");
    println!("{}", sentence);

    println!(r".D\Rust\main"); // = everything in here is all strings or we can simply put two \\ s

    let decimal_number = 123;
    println!(
        "The decimal number is {}\n which in hexa is {:#X}",
        decimal_number, decimal_number
    );
    println!(
        "The decimal number is {}\n which in hexa is {:#x}",
        decimal_number, decimal_number
    );
    println!(
        "The decimal number is {}\n which in hexa is {:x}",
        decimal_number, decimal_number
    );
    println!(
        "The decimal number is {}\n which in binary is {:b}",
        decimal_number, decimal_number
    );
    println!(
        "The decimal number is {}\n which in octa is {:o}",
        decimal_number, decimal_number
    );
}
