fn main() {
    println!("Hello, world!");

    another_function();
    another_another_function(9332043489);
    function_with_two_arguments(2, 'm');

    let x = 5;
    
}

fn another_function() -> char {
    println!("Another function.");
    let x = 'i';
    x
}

fn another_another_function(x: i128) {
    println!("{}", x)
}

fn function_with_two_arguments(value: i32, unit: char) {
    println!("this thing is {} {}", value, unit)
}

// by the rust book, for statements and arguments

// Statements are instructions that perform some action and do not return a value. 
// Expressions evaluate to a resulting value. 

// x <- this is an expression
// x; <- this is a statement, notice the lack of a semicolon

// also we use an arrow '->' to signify the return type of a func
// eg
// fn lol() -> char {
//     let x = 'i'
//     x 
//}