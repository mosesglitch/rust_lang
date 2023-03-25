// if Expressions

// fn main() {
//     let number = 3;

//     if number < 5 {
//         println!("condition was true");
//     } else {
//         println!("condition was false");
//     }
// }

// Blocks of code associated with the conditions in if expressions are sometimes called arm
// fn main() {
//     let number = 3;

//     if number { //Rust will not automatically try to convert non-Boolean types to a Boolean. You must be explicit and always provide if with a Boolean as its condition.
//         println!("number was three");
//     }
// }

// fn main() {
//     let number = 6;

//     if number % 4 == 0 {
//         println!("number is divisible by 4");
//     } else if number % 3 == 0 {
//         println!("number is divisible by 3");
//     } else if number % 2 == 0 {
//         println!("number is divisible by 2");
//     } else {
//         println!("number is not divisible by 4, 3, or 2");
//     }
// }

// Using too many else if expressions can clutter your code, so if you have more than one, you might want to refactor your code.
// describes a powerful Rust branching construct called match for these cases.

// Using if in a let Statement
// Because if is an expression, we can use it on the right side of a let statement to assign the outcome to a variable
// fn main() {
//     let condition = true;
//     let number = if condition { 5 } else { 6 };

//     println!("The value of number is: {number}");
// }
// the value of the whole if expression depends on which block of code executes. 
// This means the values that have the potential to be results from each arm of the if must be the same type

// Repeating Code with loop
// The loop keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop.

// fn main() {
//     loop {
//         println!("again!");
//     }
// } 
// You can place the break keyword within the loop to tell the program when to stop executing the loop. 

// We also used continue in the guessing game, which in a loop tells the program to skip over any remaining code in this iteration of the loop and go to the next iteration.

// Returning Values from Loops

// fn main() {
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;

//         if counter == 20 {
//             break counter * 2;
//         }
//     };

//     println!("The result is {result}");
// }

// Loop Labels to Disambiguate Between Multiple Loops
// If you have loops within loops, break and continue apply to the innermost loop at that point. 
// You can optionally specify a loop label on a loop that you can then use with break or continue to specify that those keywords apply to the labeled loop instead of the innermost loop. 
// Loop labels must begin with a single quote
// fn main() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 10;

//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }

//         count += 1;
//     }
//     println!("End count = {count}");
// }

// Conditional Loops with while
// fn main() {
//     let mut number = 3;

//     while number != 0 {
//         println!("{number}!");

//         number -= 1;
//     }

//     println!("LIFTOFF!!!");
// }

// Looping Through a Collection with for

// fn main() {
//     let a = [10, 20, 30, 40, 50];
//     let mut index = 0;

//     while index < 5 {
//         println!("the value is: {}", a[index]);

//         index += 1;
//     }
// }
// this approach is error prone; we could cause the program to panic if the index value or test condition is incorrect. 
// It’s also slow, because the compiler adds runtime code to perform the conditional check of whether the index is within the bounds of the array on every iteration through the loop
// fn main() {
//     let a = [10, 20, 30, 40, 50];

//     for element in a {
//         println!("the value is: {element}");
//     }
// }
// we’ve now increased the safety of the code and eliminated the chance of bugs that might result from going beyond the end of the array or not going far enough and missing some items.
fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}