use super::*;

use std::path::Path;
use std::process::Command;

#[test]
fn omniglot_images_background_10_pairs_test() {
    if !Path::new("oneshot-data/data_augmented").exists() {
        Command::new("git")
            .arg("submodule")
            .arg("update")
            .arg("--init")
            .output()
            .expect("Failed to initialize data submodule.");
    }
    if !Path::new("data_augmented").exists() {
        Command::new("unzip")
            .arg("oneshot-data/data_augmented/images_background_augmented.zip")
            .output()
            .expect("Failed to unzip data zip.");
    }
    let num_pairs: usize = 76;
    let directory: &str = "data_augmented/images_background_augmented";
    let pairs = match sample::random_pairs(directory, num_pairs) {
        Err(e) => panic!(e),
        Ok(ps) => ps,
    };

    let (train_x1, train_x2, train_y) = data::load_pair_set(pairs);

    println!("{:?}", train_x1.shape());
    network::train(train_x1, train_x2, train_y, 25, 2)
}
