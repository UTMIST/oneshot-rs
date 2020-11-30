use rand::seq::SliceRandom;
use std::path::PathBuf;
use std::{fs, io, process, vec::Vec};

pub fn random_sample(
    data_path: &str,
    positive: bool,
    script1: &str,
    script2: &str,
) -> io::Result<()> {
    let random_choices = if positive {
        match random_sample_positive(data_path, script1) {
            Ok(rc) => rc,
            Err(_) => process::exit(0),
        }
    } else if script2.len() > 0 {
        match random_sample_negative_diff_scripts(data_path, script1, script2) {
            Ok(rc) => rc,
            Err(_) => process::exit(0),
        }
    } else {
        match random_sample_negative_same_script(data_path, script1) {
            Ok(rc) => rc,
            Err(_) => process::exit(0),
        }
    };

    println!("{:?}", random_choices);
    Ok(())
}

pub fn random_sample_negative_diff_scripts(
    _data_path: &str,
    _script1: &str,
    _script2: &str,
) -> io::Result<Vec<PathBuf>> {
    unimplemented!();
}

pub fn random_sample_negative_same_script(
    _data_path: &str,
    _script: &str,
) -> io::Result<Vec<PathBuf>> {
    unimplemented!();
}

pub fn random_sample_positive(data_path: &str, script: &str) -> io::Result<Vec<PathBuf>> {
    let character = fs::read_dir(format!("{}/{}", data_path, script))?
        .map(|res| res.map(|e| e.file_name()))
        .collect::<Result<Vec<_>, io::Error>>()?
        .choose(&mut rand::thread_rng())
        .cloned();

    let samples: Vec<PathBuf> = match character {
        None => Vec::new(),
        Some(c) => match c.to_str() {
            None => Vec::new(),
            Some(c) => fs::read_dir(format!("{}/{}/{}", data_path, script, c))?
                .map(|res| res.map(|e| e.path()))
                .collect::<Result<Vec<_>, io::Error>>()?
                .choose_multiple(&mut rand::thread_rng(), 2)
                .cloned()
                .collect(),
        },
    };

    Ok(samples)
}
