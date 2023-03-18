// fn main() {
//     let x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }
// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }
// Constants can be declared in any scope, including the global scope, which makes them useful for values that many parts of code need to know about.
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
// Rust’s naming convention for constants is to use all uppercase with underscores between words
// Naming hardcoded values used throughout your program as constants is useful in conveying the meaning of that value to future maintainers of the code. 
// It also helps to have only one place in your code you would need to change if the hardcoded value needed to be updated in the future.

// Shadowing
// fn main() {
//     let x = 5;

//     let x = x + 1;

//     {
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {x}");
//     }

//     println!("The value of x is: {x}");
// }














