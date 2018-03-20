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
