# Rust Temperature Converter

This project is a continuation of my efforts to familiarize myself with writing
idiomatic Rust code. For this exercise, I aim to port an example from Brian
W. Kernighan and Dennis M. Ritchie's 'The ANSI C Programming Language
2nd Edition' into Rust, and compare the differences between Rust and C.

This also presented a chance to learn more about parsing command line options.
While the example in the book used some hard-coded variables to specify the
minimum, maximum, and step sizes, I wanted to see if I could read these values
from the program's arguments.

## Differences from C

I am going to consider some well-documented differences between C and Rust,
such as Rust's ownership/borrowing system, out of the scope of this writeup.
