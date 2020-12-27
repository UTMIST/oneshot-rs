use rand::seq::SliceRandom;
use std::path::PathBuf;
use std::vec::Vec;
use std::{ffi, fs, io};

macro_rules! choose {
    ($dir:expr, $func:expr) => {
        fs::read_dir($dir)?
            .map($func)
            .collect::<Result<Vec<_>, io::Error>>()?
            .choose(&mut rand::thread_rng())
    };
    ($dir:expr, $func:expr, $num:expr) => {
        fs::read_dir($dir)?
            .map($func)
            .collect::<Result<Vec<_>, io::Error>>()?
            .choose_multiple(&mut rand::thread_rng(), $num)
            .cloned()
            .collect();
    };
}

/// Two random samples from different characters from different scripts.
pub fn sample_negative_diff_scripts(
    script1: &PathBuf,
    script2: &PathBuf,
) -> io::Result<(PathBuf, PathBuf)> {
    if let (Some(c1), Some(c2)) = (
        choose!(script1, |res| res.map(|e| e.file_name())),
        choose!(script2, |res| res.map(|e| e.file_name())),
    ) {
        if let (Some(cs_1), Some(cs_2)) = (c1.to_str(), c2.to_str()) {
            if let (Some(p1), Some(p2)) = (
                choose!(script1.join(cs_1), |res| res.map(|e| e.path())),
                choose!(script2.join(cs_2), |res| res.map(|e| e.path())),
            ) {
                return Ok((p1.to_path_buf(), p2.to_path_buf()));
            }
        }
    }
    panic!()
}

/// Two random samples from different characters from the same script.
pub fn sample_negative_same_script(script: &PathBuf) -> io::Result<(PathBuf, PathBuf)> {
    let chars: Vec<ffi::OsString> = choose!(script, |res| res.map(|e| e.file_name()), 2);
    assert_eq!(chars.len(), 2);
    if let (Some(cs_1), Some(cs_2)) = (&chars[0].to_str(), &chars[1].to_str()) {
        if let (Some(p1), Some(p2)) = (
            choose!(script.join(cs_1), |res| res.map(|e| e.path())),
            choose!(script.join(cs_2), |res| res.map(|e| e.path())),
        ) {
            return Ok((p1.to_path_buf(), p2.to_path_buf()));
        }
    }
    panic!()
}

/// Two random samples from the same character from the same script.
pub fn sample_positive(script: &PathBuf) -> io::Result<(PathBuf, PathBuf)> {
    if let Some(c_os_str) = choose!(script, |res| res.map(|e| e.file_name())) {
        if let Some(c_str) = c_os_str.to_str() {
            let paths: Vec<PathBuf> = choose!(script.join(c_str), |res| res.map(|e| e.path()), 2);
            assert_eq!(paths.len(), 2);
            return Ok((paths[0].to_path_buf(), paths[1].to_path_buf()));
        }
    }
    panic!()
}
