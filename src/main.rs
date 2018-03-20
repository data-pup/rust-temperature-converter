#![feature(iterator_step_by)]

extern crate getopts;
use getopts::Options;
use std::env;

fn print_usage(program: &str, opts: Options) {
    let info = format!("Usage: {}", program);
    print!("{}", opts.usage(&info));
}

fn init_options_parser() -> Options {
    let mut opts = Options::new();
    opts.optflag("h", "help", "Print this help information");
    opts.optopt("l", "lower", "Lower temperature limit (Default: 0)",   "TEMP");
    opts.optopt("u", "upper", "Upper temperature limit (Default: 300)", "TEMP");
    opts.optopt("s", "step",  "Temperature step size",                  "STEP");
    opts
}

fn main() {
    let args: Vec<String> = env::args().collect(); // Collect the arguments
    let program = args[0].clone();                 // and program name.
    let opts = init_options_parser();

    let matches = match opts.parse(&args[1..]) {
        Ok(m) =>  { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let (lower, upper) = (0, 300);  // Declare the upper/lower limits.
    let step: usize = 20;           // Step size.
    println!("F\tC");               // Print a header and begin the loop.
    for fahr in (lower..upper + step as i32).step_by(step) {
        let celsius = 5 * (fahr - 32) / 9;
        println!("{f}\t{c}", f=fahr, c=celsius);
    }
}
