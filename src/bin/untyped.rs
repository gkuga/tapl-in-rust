use std::env;
use std::fs;

use tapl::untyped::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: $ cargo run --bin untyped test/untyped/test.f");
        std::process::exit(1);
    }
    let content = fs::read_to_string(&args[1]).unwrap();
    let res = parse(args[1].clone(), content).unwrap();
    for term in res {
        match term {
            Command::Bind(_, x, _) => println!("{}", x),
            Command::Eval(_, t) => {
                println!("{}", eval(Vec::<(String, Binding)>::new(), t).unwrap());
            }
        }
    }
}
