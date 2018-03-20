# Rust Temperature Converter

This is a Rust port of the example in Chapter 1.2 of the K&R C book.

This project is a continuation of my efforts to familiarize myself with writing
idiomatic Rust code. For this exercise, I aim to port an example from Brian
W. Kernighan and Dennis M. Ritchie's 'The ANSI C Programming Language
2nd Edition' into Rust, and compare the differences between Rust and C.

This also presented a chance to learn more about parsing command line options.
While the example in the book used some hard-coded variables to specify the
minimum, maximum, and step sizes, I wanted to see if I could read these values
from the program's arguments.

## Basic Implementation

I am going to consider some well-documented differences between C and Rust,
such as Rust's ownership/borrowing system, out of the scope of this writeup.
First, I started by implementing a basic port of the temperature conversion
table printing code from the book.

When I was finished with my basic reimplementation, the code looked like this:

```rust
fn main() {
    let mut fahr: i32;
    let mut celsius: i32;

    let lower: i32 = 0;    // Lower temperature limit.
    let upper: i32 = 300;  // Upper temperature limit.
    let step: i32 = 20;    // Step size.

    println!("F\tC");      // Print a table header, and begin the loop.
    fahr = lower;
    while fahr <= upper {
        celsius = 5 * (fahr - 32) / 9;
        println!("{f}\t{c}", f=fahr, c=celsius);
        fahr = fahr + step;
    }
}
```
