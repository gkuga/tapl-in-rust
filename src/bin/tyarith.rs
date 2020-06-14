use std::env;
use std::fs;

use tapl::tyarith::*;

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
        let ty = type_of(t.clone()).unwrap();
        println!("{} : {}", big_step_eval(t).unwrap(), ty);
    }
}
