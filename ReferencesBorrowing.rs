// A reference is like a pointer in that it’s an address we can follow to access the data stored at that address; that data is owned by some other variable. 
// Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.
// Here is how you would define and use a calculate_length function that has a reference to an object as a parameter instead of taking ownership of the value.

fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize { //// s is a reference to a String
    s.len()

    // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.
}
// First, notice that all the tuple code in the variable declaration and the function return value is gone. 
// Second, note that we pass &s1 into calculate_length and, in its definition, we take &String rather than String. 
// These ampersands represent references, and they allow you to refer to some value without taking ownership of it.
// . Because it does not own it, the value it points to will not be dropped when the reference stops being used
// The opposite of referencing by using & is dereferencing, which is accomplished with the dereference operator, *. 














