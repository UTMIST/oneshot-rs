use ndarray_image::open_gray_image;
use std::path::PathBuf;
use std::vec::Vec;

/// ND-Array for bilevel (black/white) image pixels.
type NDArr = ndarray::Array<bool, ndarray::IxDyn>;

macro_rules! push_pixels {
    ($p:expr, $v:expr) => {
        let img = open_gray_image($p).expect("unable to open image");
        for pixel in img.into_iter() {
            $v.push(*pixel > 0);
        }
    };
}

pub fn load_pair_set(pair_paths: Vec<(PathBuf, PathBuf)>) -> (NDArr, NDArr, NDArr) {
    let set_size = pair_paths.len();

    // Create vectors and push pixels/labels.
    let (mut x1, mut x2): (Vec<bool>, Vec<bool>) = (Vec::new(), Vec::new());
    let mut y = Vec::new();
    for (s1, s2) in pair_paths {
        y.push(s1.parent() == s2.parent());
        push_pixels!(s1, x1);
        push_pixels!(s2, x2);
    }

    // Reshape and return vectors.
    let as_arr = NDArr::from_shape_vec;
    let x1 = as_arr(ndarray::IxDyn(&[set_size, 1, 105, 105]), x1).unwrap();
    let x2 = as_arr(ndarray::IxDyn(&[set_size, 1, 105, 105]), x2).unwrap();
    let y = as_arr(ndarray::IxDyn(&[set_size, 1]), y).unwrap();
    (x1, x2, y)
}
