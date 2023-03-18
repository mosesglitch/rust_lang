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
// const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
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

// Scalar Types

// Data Types
// Each signed variant can store numbers from -(2n - 1) to 2n - 1 - 1 inclusive, where n is the number of bits that variant uses.
// So an i8 can store numbers from -(27) to 27 - 1, 
// which equals -128 to 127. Unsigned variants can store numbers from 0 to 2n - 1, 
// so a u8 can store numbers from 0 to 28 - 1, which equals 0 to 255.
// Additionally, the isize and usize types depend on the architecture of the computer your program is running on, 
// which is denoted in the table as “arch”: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.
// Length	Signed	Unsigned
// 8-bit	i8	u8
// 16-bit	i16	u16
// 32-bit	i32	u32
// 64-bit	i64	u64
// 128-bit	i128	u128
// arch	isize	usize

// You can write integer literals in any of the forms shown in Table  
// Note that number literals that can be multiple numeric types allow a type suffix, such as 57u8, to designate the type.
// Number literals	Example
// Decimal	98_222
// Hex	0xff
// Octal	0o77
// Binary	0b1111_0000
// Byte (u8 only)	b'A'

// Let’s say you have a variable of type u8 that can hold values between 0 and 255. If you try to change the variable to a value outside that range, 
// such as 256, integer overflow will occur, which can result in one of two behaviors. 
// When you’re compiling in debug mode, Rust includes checks for integer overflow that cause your program to panic at runtime if this behavior occurs.

// To explicitly handle the possibility of overflow, you can use these families of methods provided by the standard library for primitive numeric types:

// Wrap in all modes with the wrapping_* methods, such as wrapping_add.
// Return the None value if there is overflow with the checked_* methods.
// Return the value and a boolean indicating whether there was overflow with the overflowing_* methods.
// Saturate at the value’s minimum or maximum values with the saturating_* methods.

// Floating-Point Types

// fn main() {
//     // addition
//     let sum = 5 + 10;

//     // subtraction
//     let difference = 95.5 - 4.3;

//     // multiplication
//     let product = 4 * 30;

//     // division
//     let quotient = 56.7 / 32.2;
//     let truncated = -5 / 3; // Results in -1

//     // remainder
//     let remainder = 43 % 5;
// }

// boolean

// fn main() {
//     let t = true;

//     let f: bool = false; // with explicit type annotation
// }

// The Character Type

// fn main() {
//     let c = 'z';
//     let z: char = 'ℤ'; // with explicit type annotation
//     let heart_eyed_cat = '😻';
// }
// Note that we specify char literals with single quotes, as opposed to string literals, which use double quotes. 
// Rust’s char type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII

// Compound Types

// The Tuple Type
// A tuple is a general way of grouping together a number of values with a variety of types into one compound type. 
// Tuples have a fixed length: once declared, they cannot grow or shrink in size.

// fn main() {
//     let tup: (i32, f64, u8) = (500, 6.4, 1);
//     tuple()
// }
// The variable tup binds to the entire tuple because a tuple is considered a single compound element. 
// To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value, like this:

// destructuring 
// fn tuple() {
//     let tup = (500, 6.4, 1);

//     let (x, y, z) = tup;

//     println!("The value of y is: {y}");
// }

// period (.)
// fn main() {
//     let x: (i32, f64, u8) = (500, 6.4, 1);

//     let five_hundred = x.0;

//     let six_point_four = x.1;

//     let one = x.2;
// }

// The Array Type

// Unlike a tuple, every element of an array must have the same type. 
// Unlike arrays in some other languages, arrays in Rust have a fixed length.
// fn main() {
//     let a = [1, 2, 3, 4, 5];
// }
// Arrays are useful when you want your data allocated on the stack rather than the heap

// An array isn’t as flexible as the vector type, though. 
// A vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size. 
// You write an array’s type using square brackets with the type of each element, a semicolon, and then the number of elements in the array
// let a: [i32; 5] = [1, 2, 3, 4, 5];

// You can also initialize an array to contain the same value for each element by specifying the initial value, followed by a semicolon, 
// let a = [3; 5];

// Accessing Array Elements
// An array is a single chunk of memory of a known, fixed size that can be allocated on the stack.

fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}

// memory safety principles in action. In many low-level languages, this kind of check is not done, and when you provide an incorrect index, 
// invalid memory can be accessed. 
// Rust protects you against this kind of error by immediately exiting instead of allowing the memory access and continuing







