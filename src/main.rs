use ndarray::s;
use ndarray_image::open_gray_image;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "image", about = "Loads image and puts red dots on it")]
struct Opt {
    /// File to put red dots on
    #[structopt(parse(from_os_str))]
    file: PathBuf,

    /// Output file with red dots
    #[structopt(parse(from_os_str))]
    output: PathBuf,
}

use walkdir::{DirEntry, WalkDir};

fn is_not_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| entry.depth() == 0 || !s.starts_with("."))
        .unwrap_or(false)
}

fn main() {
    WalkDir::new("./sandbox")
        .into_iter()
        .filter_entry(|e| is_not_hidden(e))
        .filter_map(|v| v.ok())
        .filter(|e| !e.file_type().is_dir())
        .for_each(|x| process(&x));
}

fn process(entry: &DirEntry) {
    let path = entry.path();
    if let Some(_) = path.to_str() {
        let mut image = open_gray_image(path).expect("unable to open input image");
        for n in image.slice_mut(s![..;10, ..;2]) {
            *n = 255;
        }
        println!("{:?}", image);
    }
}
