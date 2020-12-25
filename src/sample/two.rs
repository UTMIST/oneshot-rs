use rand::seq::SliceRandom;
use std::path::PathBuf;
use std::{ffi, fs, io, vec::Vec};

/// Two random samples from different characters from different scripts.
pub fn sample_negative_diff_scripts(
    script1: &PathBuf,
    script2: &PathBuf,
) -> io::Result<Vec<PathBuf>> {
    let mut samples: Vec<PathBuf> = Vec::new();
    for script in vec![script1, script2] {
        let character = fs::read_dir(script)?
            .map(|res| res.map(|e| e.file_name()))
            .collect::<Result<Vec<_>, io::Error>>()?
            .choose(&mut rand::thread_rng())
            .cloned();

        if let Some(c_os_str) = character {
            if let Some(c_str) = c_os_str.to_str() {
                if let Some(s) = fs::read_dir(script.join(c_str))?
                    .map(|res| res.map(|e| e.path()))
                    .collect::<Result<Vec<_>, io::Error>>()?
                    .choose(&mut rand::thread_rng())
                {
                    samples.push(s.to_path_buf())
                }
            }
        }
    }
    Ok(samples)
}

/// Two random samples from different characters from the same script.
pub fn sample_negative_same_script(data_path: &PathBuf) -> io::Result<Vec<PathBuf>> {
    let characters: Vec<ffi::OsString> = fs::read_dir(data_path)?
        .map(|res| res.map(|e| e.file_name()))
        .collect::<Result<Vec<_>, io::Error>>()?
        .choose_multiple(&mut rand::thread_rng(), 2)
        .cloned()
        .collect();

    let mut samples: Vec<PathBuf> = Vec::new();
    for c_os_str in characters {
        if let Some(c_str) = c_os_str.to_str() {
            if let Some(s) = fs::read_dir(data_path.join(c_str))?
                .map(|res| res.map(|e| e.path()))
                .collect::<Result<Vec<_>, io::Error>>()?
                .choose(&mut rand::thread_rng())
            {
                samples.push(s.to_path_buf())
            }
        };
    }

    Ok(samples)
}

/// Two random samples from the same character from the same script.
pub fn sample_positive(data_path: &PathBuf) -> io::Result<Vec<PathBuf>> {
    let character = fs::read_dir(data_path)?
        .map(|res| res.map(|e| e.file_name()))
        .collect::<Result<Vec<_>, io::Error>>()?
        .choose(&mut rand::thread_rng())
        .cloned();

    let mut samples: Vec<PathBuf> = Vec::new();
    if let Some(c_os_str) = character {
        if let Some(c_str) = c_os_str.to_str() {
            for path in fs::read_dir(data_path.join(c_str))?
                .map(|res| res.map(|e| e.path()))
                .collect::<Result<Vec<_>, io::Error>>()?
                .choose_multiple(&mut rand::thread_rng(), 2)
            {
                samples.push(path.to_path_buf());
            }
        }
    };

    Ok(samples)
}
