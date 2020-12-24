use rand::seq::SliceRandom;
use std::path::PathBuf;
use std::{ffi, fs, io, vec::Vec};

/// Two random samples drawn from the functions given arguments.
pub fn random_sample(
    data_path: &str,
    positive: bool,
    script1: &str,
    script2: &str,
) -> io::Result<Vec<PathBuf>> {
    let samples = match (positive, script2.len()) {
        (true, _) => random_sample_positive(data_path, script1),
        (_, 0) => random_sample_negative_same_script(data_path, script1),
        _ => random_sample_negative_diff_scripts(data_path, script1, script2),
    };

    Ok(match samples {
        Ok(s) => s,
        Err(_) => Vec::new(),
    })
}

/// Two random samples from different characters from different scripts.
pub fn random_sample_negative_diff_scripts(
    data_path: &str,
    script1: &str,
    script2: &str,
) -> io::Result<Vec<PathBuf>> {
    let mut samples: Vec<PathBuf> = Vec::new();
    for script in vec![script1, script2] {
        let character = fs::read_dir(format!("{}/{}", data_path, script))?
            .map(|res| res.map(|e| e.file_name()))
            .collect::<Result<Vec<_>, io::Error>>()?
            .choose(&mut rand::thread_rng())
            .cloned();

        if let Some(c_os_str) = character {
            if let Some(c_str) = c_os_str.to_str() {
                if let Some(s) = fs::read_dir(format!("{}/{}/{}", data_path, script, c_str))?
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
pub fn random_sample_negative_same_script(
    data_path: &str,
    script: &str,
) -> io::Result<Vec<PathBuf>> {
    let characters: Vec<ffi::OsString> = fs::read_dir(format!("{}/{}", data_path, script))?
        .map(|res| res.map(|e| e.file_name()))
        .collect::<Result<Vec<_>, io::Error>>()?
        .choose_multiple(&mut rand::thread_rng(), 2)
        .cloned()
        .collect();

    let mut samples: Vec<PathBuf> = Vec::new();
    for c_os_str in characters {
        if let Some(c_str) = c_os_str.to_str() {
            if let Some(s) = fs::read_dir(format!("{}/{}/{}", data_path, script, c_str))?
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
pub fn random_sample_positive(data_path: &str, script: &str) -> io::Result<Vec<PathBuf>> {
    let character = fs::read_dir(format!("{}/{}", data_path, script))?
        .map(|res| res.map(|e| e.file_name()))
        .collect::<Result<Vec<_>, io::Error>>()?
        .choose(&mut rand::thread_rng())
        .cloned();

    let mut samples: Vec<PathBuf> = Vec::new();
    if let Some(c_os_str) = character {
        if let Some(c_str) = c_os_str.to_str() {
            for path in fs::read_dir(format!("{}/{}/{}", data_path, script, c_str))?
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
