extern crate autograd as ag;
extern crate ndarray;

use ag::ndarray_ext as array;
use ag::optimizers::adam;
use ag::rand::seq::SliceRandom;
use ag::tensor::Variable;
use ag::Graph;
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

type Tensor<'graph> = ag::Tensor<'graph, f32>;

fn inputs(g: &Graph<f32>) -> (Tensor, Tensor) {
    let x = g.placeholder(&[-1, 1, 105, 105]);
    let y = g.placeholder(&[-1, 1]);
    (x, y)
}

fn conv_pool<'g>(x: Tensor<'g>, w: Tensor<'g>, b: Tensor<'g>, num_filters: i32) -> Tensor<'g> {
    let g = x.graph();
    let y1 = g.conv2d(x, w, 0, 1) + b;
    let y2 = g.relu(y1);
    x.graph().max_pool2d(y2, 2, 0, 2)
}

pub fn train(samples: (&PathBuf, &PathBuf)) {
    println!("{:?}\n{:?}", samples.0, samples.1);
    // let opt = Opt::from_args();
    let mut image1 = open_gray_image(samples.0).expect("unable to open input image");
    let mut image2 = open_gray_image(samples.1).expect("unable to open input image");
    let xtrain = image1
        .into_shape(ndarray::IxDyn(&[1, 1, 105, 105]))
        .unwrap();

    ag::with(|g| {
        let rng = ag::ndarray_ext::ArrayRng::<f32>::default();
        let w1 = g.variable(rng.random_normal(&[64, 1, 10, 10], 0., 0.1));
        let b1 = g.variable(array::zeros(&[1, 64, 105, 105]));
        let params = &[w1, b1];
        let param_arrays = params
            .iter()
            .map(|v| v.get_variable_array().unwrap())
            .collect::<Vec<_>>();
        let adam_state = adam::AdamState::new(param_arrays.as_slice());
    }
}
