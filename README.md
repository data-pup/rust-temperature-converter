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

This is not a very idiomatic implementation however, and largely mimics the
original C code. About the only thing here that is distinctly Rust is the
declaration statements of `fahr` and `celcius`, which include the `mut` keyword.
Unless explicitly marked otherwise, variables are immutable.

The code can be rewritten like so:

```rust
#![feature(iterator_step_by)]
fn main() {
    let (lower, upper) = (0, 300);  // Declare the upper/lower limits.
    let step: usize = 20;           // Step size.
    println!("F\tC");               // Print a header and begin the loop.
    for fahr in (lower..upper + step as i32).step_by(step) {
        let celsius = 5 * (fahr - 32) / 9;
        println!("{f}\t{c}", f=fahr, c=celsius);
    }
}
```

Here the `upper` and `lower` limit variables are initialized in a single line.
Rust offers destructuring, which means we can assign the two variables at once.

Rust's `Range` type can also be used, so that we can use a `for` loop instead
of the previous implementation's `while` loop. `(a..b)` specifies a range
including the values `a, a+1, ..., b-1`. We can specify a step size by using
the `step_by(..)` method, which expects a `usize` variable.

Because the end of the range is not inclusive, we must add the step size to the
upper limit when declaring the range. This requires casting the `usize` value
into an `i32` using the `as` keyword.

One other thing worth noting is that this is using a feature only available
currently in the nightly build. This feature is enabled by the first line
of the program.

After making all of these changes, we have a concise implementation that also
involves no mutably declared variables. Neat! Next, let's see if we can read
the limit and step values from the command line arguments.

## Reading Command Line Arguments

The equivalent to C's `argv` and `argc` variables used to read arguments in
Rust is `env::args`. I was interested in implementing some basic option flags,
as well as learning more about the crates ecosystem, so rather than building
my own bird-feeder, I elected to use the `getopts` crate.

The `getopts` crate can be found here: https://crates.io/crates/getopts

Adding this dependency is not difficult at all! First, the following lines
are added to the `Cargo.toml` file:

```
[dependencies]
getopts = "0.2"
```

Then the following line is then added to the `main.rs` file.

```rust
extern crate getopts;
```

After substantial refactoring, I was able to create a program that would
parse the command line options into a configuration struct that stored the
table settings, or print help information if the `-h` flag was included.

```sh
$: ./rust-temp-converter -h
Usage: ./rust-temp-converter

Options:
    -h, --help          Print this help information
    -l, --lower TEMP    Lower temperature limit (Default: 0)
    -u, --upper TEMP    Upper temperature limit (Default: 300)
    -s, --step STEP     Temperature step size
```

One of the other alterations I enjoyed making here was in the for loop used
to print the table. Higher order functions allow us to program functionally
in Rust!

```rust
println!("F\tC");
for curr_row in
    (lower..upper + step as i32)
    .step_by(step)
    .map(|fahr| (fahr, 5 * (fahr - 32) / 9)) {
        let (fahr, celcius) = curr_row;
        println!("{f}\t{c}", f=fahr, c=celcius);
    }
```

Here is an example of what the program printed after rebuilding it to accept
command line arguments.

```sh
$: ./rust-temp-converter -l 20 -u 40 -s 2
F       C
20      -6
22      -5
24      -4
26      -3
28      -2
30      -1
32      0
34      1
36      2
38      3
40      4
```