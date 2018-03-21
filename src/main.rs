#![feature(iterator_step_by)]

extern crate getopts;

mod prog_options {
    use std::env;
    use getopts::{Matches, Options};

    static LOWER_DEFAULT: i32 = 0;
    static UPPER_DEFAULT: i32 = 300;
    static STEP_DEFAULT: usize = 20;

    pub struct TableConfig {
        pub lower: i32,
        pub upper: i32,
        pub step: usize,
    }

    pub fn get_config() -> Option<TableConfig> {
        let args:    Vec<String> = env::args().collect();
        let opts:    Options     = init_options_parser();
        let matches: Matches = match opts.parse(&args[1..]) {
            Ok(m) =>  { m }
            Err(f) => { panic!(f.to_string()) }
        };
        if print_help(&args, &opts, &matches) { return None; }
        Some(TableConfig{
            lower:get_lower(&matches),
            upper:get_upper(&matches),
            step:get_step(&matches),
        })
    }

    fn init_options_parser() -> Options {
        let mut opts = Options::new();
        opts.optflag("h", "help", "Print this help information");
        opts.optopt("l", "lower", "Lower temperature limit (Default: 0)",   "TEMP");
        opts.optopt("u", "upper", "Upper temperature limit (Default: 300)", "TEMP");
        opts.optopt("s", "step",  "Temperature step size",                  "STEP");
        opts
    }

    fn print_help(args: &Vec<String>, opts:&Options, matches: &Matches) -> bool {
        if matches.opt_present("h") {
            let program = &args[0];
            let info = format!("Usage: {}", program);
            print!("{}", opts.usage(&info));
            return true;
        }
        return false;
    }

    fn get_lower(matches: &Matches) -> i32 {
        return match matches.opt_str("l") {
            Some(l) => { l.parse::<i32>().ok().unwrap_or(LOWER_DEFAULT) },
            None    => { LOWER_DEFAULT },
        };
    }

    fn get_upper(matches: &Matches) -> i32 {
        return match matches.opt_str("u") {
            Some(u) => { u.parse::<i32>().ok().unwrap_or(UPPER_DEFAULT) },
            None    => { UPPER_DEFAULT },
        };
    }

    fn get_step(matches: &Matches) -> usize {
        return match matches.opt_str("s") {
            Some(s) => { s.parse::<usize>().ok().unwrap_or(STEP_DEFAULT) },
            None    => { STEP_DEFAULT },
        }
    }
}

fn print_table(config:prog_options::TableConfig) {
    let prog_options::TableConfig {upper, lower, step} = config;
    println!("F\tC");
    for curr_row in
        (lower..upper + step as i32)
        .step_by(step)
        .map(|fahr| (fahr, 5 * (fahr - 32) / 9)) {
            let (fahr, celcius) = curr_row;
            println!("{f}\t{c}", f=fahr, c=celcius);
        }
}

fn main() {
    match prog_options::get_config() {
        Some(config) => { print_table(config); }
        _ => { }
    };
}
