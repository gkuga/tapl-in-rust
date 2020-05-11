use std::env;
use std::fs;

use tapl::arith::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: $ cargo run --bin arith test/arith/test.f");
        std::process::exit(1);
    }
    let content = fs::read_to_string(&args[1]).unwrap();
    let res = parse(args[1].clone(), content).unwrap();
    for term in res {
        let Command::Eval(t) = term;
        println!("{}", eval(t).unwrap());
    }
}
