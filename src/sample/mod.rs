mod two;

use rand::seq::SliceRandom;
use std::fs;
use std::io::Result;
use std::path::PathBuf;
use std::vec::Vec;

/// Generate <num_pairs> pairs of character images.
pub fn random_pairs(dataset: &str, num_pairs: usize) -> Result<Vec<Vec<PathBuf>>> {
    let scripts = fs::read_dir(dataset)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>>>()?;

    let mut pairs: Vec<Vec<PathBuf>> = Vec::new();
    macro_rules! sample {
        ( $func:expr $( , $more:expr )* ) => (
            if let Ok(ps) = $func($( $more ),*) {
                pairs.push(ps);
                continue;
            }
            panic!();
        );
    }

    for _ in 0..num_pairs {
        // Probabilisticly, half will be pairs of the same character.
        if rand::random::<bool>() {
            if let Some(s) = scripts.choose(&mut rand::thread_rng()) {
                sample!(two::sample_positive, s);
            }
            panic!();
        }
        if let Some(s1) = scripts.choose(&mut rand::thread_rng()) {
            if let Some(s2) = scripts.choose(&mut rand::thread_rng()) {
                if s1 == s2 {
                    sample!(two::sample_negative_same_script, s1);
                }
                sample!(two::sample_negative_diff_scripts, s1, s2);
            }
        }
    }

    Ok(pairs)
}
