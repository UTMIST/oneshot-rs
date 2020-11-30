use oneshot;
use std::{env, io};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        4..=5 => oneshot::sample::random_sample(
            &args[1],
            args[2].as_str() == "1",
            &args[3],
            match args.len() {
                5 => &args[4],
                _ => "",
            },
        ),
        _ => Ok(()),
    }
}
