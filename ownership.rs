// Ownership is Rust’s most unique feature and has deep implications for the rest of the language. 
// It enables Rust to make memory safety guarantees without needing a garbage collector, so it’s important to understand how ownership works.

// Ownership is a set of rules that govern how a Rust program manages memory.
// Some languages have garbage collection that regularly looks for no-longer-used memory as the program runs; in other languages, the programmer must explicitly allocate and free the memory.
// Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks. 
// If any of the rules are violated, the program won’t compile. 
// None of the features of ownership will slow down your program while it’s running

// The Stack and the Heap
//Stack
// in a systems programming language like Rust, whether a value is on the stack or the heap affects how the language behaves and why you have to make certain decisions. 
// The stack stores values in the order it gets them and removes the values in the opposite order. This is referred to as last in, first out.
// Adding data is called pushing onto the stack, and removing data is called popping off the stack
// All data stored on the stack must have a known, fixed size. 
// Data with an unknown size at compile time or a size that might change must be stored on the heap instead.
// Heap
// The heap is less organized: when you put data on the heap, you request a certain amount of space. 
// The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, 
// and returns a pointer, which is the address of that location. 
// This process is called allocating on the heap and is sometimes abbreviated as just allocating
// Because the pointer to the heap is a known, fixed size, you can store the pointer on the stack, but when you want the actual data, you must follow the pointer
// Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data; that location is always at the top of the stack. 
// Comparatively, allocating space on the heap requires more work because the allocator must first find a big enough space to hold the data and then perform bookkeeping to prepare for the next allocation.
// Accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer to get there. 

// Keeping track of what parts of code are using what data on the heap, 
//                 minimizing the amount of duplicate data on the heap, 
//                 cleaning up unused data on the heap so you don’t run out of space are all problems that ownership addresses.

// Ownership Rules
// Each value in Rust has an owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.

// String literals are convenient, but they aren’t suitable for every situation in which we may want to use text. 
// One reason is that they’re immutable. 
// Another is that not every string value can be known when we write our code: 
// For these situations, Rust has a second string type, String.
// This type manages data allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time. 
// You can create a String from a string literal using the from function, like so

// let s = String::from("hello");

// fn main() {
//     let mut s = String::from("hello");

//     s.push_str(", world!"); // push_str() appends a literal to a String

//     println!("{}", s); // This will print `hello, world!`
// }

// Memory and Allocation
// In the case of a string literal, we know the contents at compile time, so the text is hardcoded directly into the final executable. 
// This is why string literals are fast and efficient. 
// But these properties only come from the string literal’s immutability.

// With the String type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, 
//    unknown at compile time, to hold the contents. This means:

// The memory must be requested from the memory allocator at runtime.
// We need a way of returning this memory to the allocator when we’re done with our String
// That first part is done by us: when we call String::from, its implementation requests the memory it needs. 
// This is pretty much universal in programming languages

// Unlike languages without a garbage collecter, Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope.
// {
//     let s = String::from("hello"); // s is valid from this point forward

//     // do stuff with s
// }                                  // this scope is now over, and s is no
//                                    // longer valid
// When a variable goes out of scope, Rust calls a special function for us. 
// This function is called drop, and it’s where the author of String can put the code to return the memory. 
// Rust calls drop automatically at the closing curly bracket.
// Note: In C++, this pattern of deallocating resources at the end of an item’s lifetime is sometimes called Resource Acquisition Is Initialization (RAII). 

// Variables and Data Interacting with Move

// let x = 5;
// let y = x;

// let s1 = String::from("hello");
// let s2 = s1;

// String under the covers. A String is made up of three parts, shown on the left: 
//                                 a pointer to the memory that holds the contents of the string, 
//                                 a length, and 
//                                 a capacity. 
// This group of data is stored on the stack. On the right is the memory on the heap that holds the contents.


// The length is how much memory, in bytes, the contents of the String are currently using. 
// The capacity is the total amount of memory, in bytes, that the String has received from the allocator.

// When we assign s1 to s2, the String data is copied, meaning we copy the pointer, the length, and the capacity that are on the stack. 
// We do not copy the data on the heap that the pointer refers to.

// Earlier, we said that when a variable goes out of scope, Rust automatically calls the drop function and cleans up the heap memory for that variable. 
// But Figure 4-2 shows both data pointers pointing to the same location. 
// This is a problem: when s2 and s1 go out of scope, they will both try to free the same memory. 
// This is known as a double free error and is one of the memory safety bugs we mentioned previously. 
// Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities.

// To ensure memory safety, after the line let s2 = s1;, Rust considers s1 as no longer valid. 
// Therefore, Rust doesn’t need to free anything when s1 goes out of scope.

// If you’ve heard the terms shallow copy and deep copy while working with other languages, the concept of copying 
//                                                                             the pointer, 
//                                                                             length, and 
//                                                                             capacity without copying the data probably sounds like making a shallow copy. 
// But because Rust also invalidates the first variable, instead of being called a shallow copy, it’s known as a move.
// we would say that s1 was moved into s2
// Rust will never automatically create “deep” copies of your data. 
// Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance.

// Variables and Data Interacting with Clone
// If we do want to deeply copy the heap data of the String, not just the stack data, we can use a common method called clone
// let s1 = String::from("hello");
//     let s2 = s1.clone();

//     println!("s1 = {}, s2 = {}", s1, s2);

// // Stack-Only Data: Copy
// let x = 5;
// let y = x;

// println!("x = {}, y = {}", x, y);
// we don’t have a call to clone, but x is still valid and wasn’t moved into y
// The reason is that types such as integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make.
// Rust has a special annotation called the Copy trait that we can place on types that are stored on the stack, as integers are
// Rust won’t let us annotate a type with Copy if the type, or any of its parts, has implemented the Drop trait
// , any group of simple scalar values can implement Copy,

// Ownership and Functions
// The mechanics of passing a value to a function are similar to those when assigning a value to a variable.
// Passing a variable to a function will move or copy, just as assignment does. 
// Listing 4-3 has an example with some annotations showing where variables go into and out of scope.

// fn main() {
//     let s = String::from("hello");  // s comes into scope

//     takes_ownership(s);             // s's value moves into the function...
//                                     // ... and so is no longer valid here

//     let x = 5;                      // x comes into scope

//     makes_copy(x);                  // x would move into the function,
//                                     // but i32 is Copy, so it's okay to still
//                                     // use x afterward

// } // Here, x goes out of scope, then s. But because s's value was moved, nothing
//   // special happens.

// fn takes_ownership(some_string: String) { // some_string comes into scope
//     println!("{}", some_string);
// } // Here, some_string goes out of scope and `drop` is called. The backing
//   // memory is freed.

// fn makes_copy(some_integer: i32) { // some_integer comes into scope
//     println!("{}", some_integer);
// } // Here, some_integer goes out of scope. Nothing special happens.

// If we tried to use s after the call to takes_ownership, Rust would throw a compile-time error. These static checks protect us from mistakes

// Return Values and Scope
// Returning values can also transfer ownership. 
// Listing 4-4 shows an example of a function that returns some value, with similar annotations as those in Listing 4-3.

// fn main() {
//     let s1 = gives_ownership();         // gives_ownership moves its return
//                                         // value into s1

//     let s2 = String::from("hello");     // s2 comes into scope

//     let s3 = takes_and_gives_back(s2);  // s2 is moved into
//     println!("S1: {s1}");
//     println!("S3: {s3}");                               // takes_and_gives_back, which also
//                                         // moves its return value into s3
// } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
//   // happens. s1 goes out of scope and is dropped.

// fn gives_ownership() -> String {             // gives_ownership will move its
//                                              // return value into the function
//                                              // that calls it

//     let some_string = String::from("yours"); // some_string comes into scope

//     some_string                              // some_string is returned and
//                                              // moves out to the calling
//                                              // function
// }

// // This function takes a String and returns one
// fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
//                                                       // scope

//     a_string  // a_string is returned and moves out to the calling function
// }

// When a variable that includes data on the heap goes out of scope, 
// the value will be cleaned up by drop unless ownership of the data has been moved to another variable.

// While this works, taking ownership and then returning ownership with every function is a bit tedious. 
// What if we want to let a function use a value but not take ownership? 
// It’s quite annoying that anything we pass in also needs to be passed back if we want to use it again,

// Rust does let us return multiple values using a tuple, as shown in Listing 4-5.

// fn main() {
//     let s1 = String::from("hello");

//     let (s2, len) = calculate_length(s1);

//     println!("The length of '{}' is {}.", s2, len);
// }

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len(); // len() returns the length of a String

// }
