fn main() {
    // Booelans = true or false
    let bool1: bool = true; // to specify the type
    let bool2 = false;

    println!("Boolean1: {}\nBoolean2: {}", bool1, bool2);

    // Characters = only one charecter
    let char: char = 'm'; // don't forget to use single quots ''

    println!("Charecter: {}", char);

    // integers = whole numbers with no decimal points and the ability to be used un math
    let int1: i8 = 127; // maximum
    let int2: i16 = 32767;
    let int3: i32 = 2147483647; // i32 = 2^32
    let int4: i64 = 9_223_372_036_854_775_807;
    let int5: i128 = 170_141_183_460_469_231_731_687_303_715_884_105_727;
    //different i(numbers) = different capacities
    println!(
        "Int1: {}\nInt2: {}\nInt3: {}\nInt4: {}\nInt5: {}",
        int1, int2, int3, int4, int5
    );

    // unasigned integers = only positive numbers includiong 0
    let uint1: u8 = 255; // maximum
    let uint2: u16 = 255;
    let uint3: u32 = 255;
    let uint4: u64 = 255;
    let uint5: u128 = 255;

    println!(
        "UInt1: {}\nUInt2: {}\nUInt3: {}\nUInt4: {}\nUInt5: {}",
        uint1, uint2, uint3, uint4, uint5
    );

    // floats = decimal points
    let float1: f32 = 32.64;
    let float2: f64 = 32.64;

    println!("Float32: {}\nFloat64: {}", float1, float2);

    // size = 32 or 64 bit (automatically)
    let var_num: isize = 6444;
    let var_num2: usize = 6444; // unasigned

    // strings = a collection of characters

    let var_str: &str = "Hello, World!"; // &str = string (slice)

    println!("String: {}", var_str)
}
