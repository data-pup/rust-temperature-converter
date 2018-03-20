#![feature(iterator_step_by)]

extern crate getopts;
use getopts::{Matches, Options};
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
    // Initialize an options parser, collect the arguments, the program name,
    // and then parse the program arguments into a Matches object.
    let opts:    Options     = init_options_parser();
    let args:    Vec<String> = env::args().collect();
    let program: String      = args[0].clone();
    let matches: Matches = match opts.parse(&args[1..]) {
        Ok(m) =>  { m }
        Err(f) => { panic!(f.to_string()) }
    };

    // If the help flag is present, print usage information and exit.
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    // Parse the upper and lower limit options, and the step size.
    let lower: i32 = match matches.opt_str("l") {  // Initialize the lower limit.
        Some(l) => { l.parse::<i32>().unwrap() }
        None    => { 0 }
    };
    let upper = match matches.opt_str("l") {       // Initialize the upper limit.
        Some(u) => { u.parse::<i32>().unwrap() }
        None    => { 300 }
    };
    let step: usize = match matches.opt_str("s") { // Initialize the step size.
        Some(s) => { s.parse::<usize>().unwrap() }
        None    => { 20 }
    };

    println!("F\tC");               // Print a header and begin the loop.
    for fahr in (lower..upper + step as i32).step_by(step) {
        let celsius = 5 * (fahr - 32) / 9;
        println!("{f}\t{c}", f=fahr, c=celsius);
    }
}
