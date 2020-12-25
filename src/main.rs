use oneshot::sample;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        process::exit(1);
    }

    let num_pairs = args[2].to_string().parse::<usize>().unwrap();
    let pairs = match sample::random_pairs(&args[1], num_pairs) {
        Err(e) => panic!(e),
        Ok(ps) => ps,
    };

    for p in pairs {
        println!("{:?}", p);
    }
}
