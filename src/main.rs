fn main() {
    let mut fahr: i32;
    let mut celsius: i32;

    let lower: i32 = 0;    // Lower temperature limit.
    let upper: i32 = 300;  // Upper temperature limit.
    let step: i32 = 20;    // Step size.

    fahr = lower;
    println!("F\tC");
    while fahr <= upper {
        celsius = 5 * (fahr - 32) / 9;
        println!("{f}\t{c}", f=fahr, c=celsius);
        fahr = fahr + step;
    }
}
