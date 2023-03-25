// main function, which is the entry point of many programs.

// fn main() {
//     println!("Hello, world!");

//     another_function(5 , 'h');
// }

// fn another_function(x: i32, unit_label: char) {
//     println!("Another function.value is: {x}{unit_label}");
// }

// Rust doesn’t care where you define your functions, only that they’re defined somewhere in a scope that can be seen by the caller
// In function signatures, you must declare the type of each parameter. 
// This is a deliberate decision in Rust’s design: 
// requiring type annotations in function definitions means the compiler almost never needs you to use them elsewhere in the code to figure out what type you mean. 
// The compiler is also able to give more helpful error messages if it knows what types the function expects.

// Statements are instructions that perform some action and do not return a value.
// Expressions evaluate to a resultant value. Let’s look at some examples.

// Expressions can be part of statements
// Calling a function is an expression. Calling a macro is an expression. A new scope block created with curly brackets is an expression
// fn main() {
//     let y = {
//         let x = 3;
//         x + 1
//     };

//     println!("The value of y is: {y}");
// }
// Expressions do not include ending semicolons. 
// If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value. 
// Functions with Return Values
// Functions can return values to the code that calls them. We don’t name return values, but we must declare their type after an arrow (->). 
// In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function.
// You can return early from a function by using the return keyword and specifying a value, but most functions return the last expression implicitly.

fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}
// There are no function calls, macros, or even let statements in the five function—just the number 5 by itself.That’s a perfectly valid function in Rust
// Second, the five function has no parameters and defines the type of the return value, but the body of the function is a lonely 5 with no semicolon because it’s an expression whose value we want to retur

fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}







