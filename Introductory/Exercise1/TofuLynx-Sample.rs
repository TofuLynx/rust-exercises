// Author: TofuLynx
// This program prints "Hello World!".
// To compile: $ rustc TofuLynx-Sample.rs
fn main() {
    // First Part: Simple Print.
    println!("Hello World!");

    // Second Part: Formatted Print.

    println!("{} {}{}", "Hello", "World", "!");

    println!("The {} value is: {}", "Pi", 3.14);

    println!("{0} said {1} is beautiful, but {1} said that {0} is ugly.", "Bob", "Chloe");

    println!("{introduction}, {object}{exclamation}",
             object       = "the lazy dog",
             introduction = "Hello",
             exclamation  = "!");

    // We use :b to print the binary representation of a number.
    println!("The binary representation of {0} is {0:b}", 30072017);

    // This prints a string with exactly 6 characters, with the number on the right and spaces in the remaining slots.
    println!("{number:>width$}", number=30072017, width=12);

    // This prints a string with exactly 6 characters, with the number on the right and zeroes in the remaining slots.
    println!("{number:>0width$}", number=30072017, width=12); 

    // This prints to io::stderr
    eprintln!("Error: Something Happened!");
}