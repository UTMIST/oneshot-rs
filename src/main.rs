use oneshot::{network, sample};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        process::exit(1);
    }

    let (directory, positive, script1) = (&args[1], args[2].as_str() == "1", &args[3]);
    let script2 = match args.len() {
        5 => &args[4],
        _ => "",
    };
    let samples = match sample::random_sample(directory, positive, script1, script2) {
        Ok(s) => s,
        Err(_) => Vec::new(),
    };

    println!("{:?}", samples);
    match (samples.first(), samples.last()) {
        (Some(s1), Some(s2)) => network::train((s1, s2)),
        _ => println!("{:?}", samples),
    }
}
