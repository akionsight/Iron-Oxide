fn main () {

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("{}", THREE_HOURS_IN_SECONDS);

    // a const cant be changed at runtime
    // can only be calculated at compile time
    // mut keyword doesnt work


    // rust has 4 (four) scaler data types that are
    // integer, floating point numbers, booleans and
    // charecters

    // floating points

    let _x = 2.0; // defaults to f64
    let _x: f32 = 3.0; // f32 has similar speed to f64 but with less precision

    // f64 has double precision float
    // while f32 only has single precision float

        // addition
        let sum = 5 + 10;

        // subtraction
        let difference = 95.5 - 4.3;
    
        // multiplication
        let product = 4 * 30;
    
        // division
        let quotient = 56.7 / 32.2;
        let floored = 2 / 3; // Results in 0
    
        // remainder
        let remainder = 43 % 5;

        println!("{},{},{},{},{},{}", product, quotient, floored, remainder, sum, difference);

        // booleans
        // basically true and false
        // for eg

        let am_i_wearing_a_shirt = true;

        // with explicit type annotation

        let do_i_smell: bool = false;

        println!("{},{}", am_i_wearing_a_shirt, do_i_smell);

        // char
        // they are enclosed in single quotes
        // they contain more than ascii, they can contain Unicode literals

        let _char_c = 'c';
        let _char_weird_z = 'ｚ';
        let _char_z = 'z';

        println!("{},{},{}", _char_c, _char_weird_z, _char_z);

        // compound types
        // are those types which store more than one type of value
        // eg: Tuple

        // Tuples
        // By the rust book "A tuple is a general way of grouping together a number of values with a variety of types into one compound type"
        // tuples have a fixed length and once declared, cannot be changed

        let tup: (i32, f64, u8) = (500, 6.4, 1);

        // destructuring a tuple 

        let (x, y, z) = tup;
        println!("Value of x = {}", x);

        // to access specific values of the tuple use the '.' seperator
        // unline ['index'] we use in python

        println!("Value of the 0th Index = {}", tup.0);
        println!("Value of 1st Index = {}", tup.1);
}
