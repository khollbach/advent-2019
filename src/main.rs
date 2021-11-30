use std::env;
use std::process;

fn main() {
    let mut args = env::args();
    let prog_name = args.next().unwrap();
    let args: Vec<_> = args.collect();

    let usage = || {
        eprintln!();
        eprint_usage(&prog_name);
        process::exit(1)
    };

    if args.len() != 1 {
        eprintln!("Expected 1 argument, got {}.", args.len());
        usage();
    }

    let day: u32 = match args[0].parse() {
        Ok(n) => if 1 <= n && n <= 25 {
            n
        } else {
            eprintln!("Not a number from 1 through 25: {}", n);
            usage()
        }
        Err(_) => {
            eprintln!("Not a u32: {}", &args[0]);
            usage()
        }
    };

    aoc_2019::solve(day);
}

fn eprint_usage(prog_name: &str) {
    eprintln!("Usage: `{} <num>`\nwhere <num> is a number from 1 through 25.", prog_name);
}
