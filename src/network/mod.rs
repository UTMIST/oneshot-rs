extern crate autograd as ag;
extern crate ndarray;

use ag::ndarray_ext as array;
use ag::{optimizers::adam, tensor::Variable, Graph};
use ndarray_image::open_gray_image;
use std::path::PathBuf;

type Tensor<'graph> = ag::Tensor<'graph, f32>;

fn inputs(g: &Graph<f32>) -> (Tensor, Tensor, Tensor) {
    let x1 = g.placeholder(&[-1, 1, 105, 105]);
    let x2 = g.placeholder(&[-1, 1, 105, 105]);
    let y = g.placeholder(&[-1, 1]);
    (x1, x2, y)
}

fn logits<'g>(w: Tensor<'g>, x: Tensor<'g>, b: Tensor<'g>) -> Tensor<'g> {
    x.graph().matmul(w, x) + b
}

fn conv_pool<'g>(x: Tensor<'g>, w: Tensor<'g>, b: Tensor<'g>) -> Tensor<'g> {
    let g = x.graph();
    let y1 = g.conv2d(x, w, 0, 1) + b;
    let y2 = g.relu(y1);
    x.graph().max_pool2d(y2, 2, 0, 2)
}

fn conv_final<'g>(x: Tensor<'g>, w: Tensor<'g>, b: Tensor<'g>) -> Tensor<'g> {
    let g = x.graph();
    let y1 = g.conv2d(x, w, 0, 1) + b;
    x.graph().relu(y1)
}

pub fn train(samples: (&PathBuf, &PathBuf)) {
    let mut _image1 = open_gray_image(samples.0).expect("unable to open input image");
    let mut _image2 = open_gray_image(samples.1).expect("unable to open input image");

    ag::with(|g| {
        let rng = ag::ndarray_ext::ArrayRng::<f32>::default();

        // Weights/Biases for convolutional layers.
        let w1 = g.variable(rng.random_normal(&[64, 1, 10, 10], 0., 0.1));
        let w2 = g.variable(rng.random_normal(&[128, 1, 7, 7], 0., 0.1));
        let w3 = g.variable(rng.random_normal(&[128, 1, 4, 4], 0., 0.1));
        let w4 = g.variable(rng.random_normal(&[256, 1, 4, 4], 0., 0.1));
        let b1 = g.variable(array::zeros(&[1, 64, 105, 105]));
        let b2 = g.variable(array::zeros(&[1, 128, 42, 42]));
        let b3 = g.variable(array::zeros(&[1, 128, 18, 18]));
        let b4 = g.variable(array::zeros(&[1, 256, 6, 6]));

        // Weights/Biases for dense layers.
        let w5 = g.variable(rng.random_normal(&[4096, 256 * 6 * 6], 0., 0.1));
        let w6 = g.variable(rng.random_normal(&[4096, 1], 0., 0.1));
        let b5 = g.variable(array::zeros(&[4096, 1]));

        let params = &[w1, w2, w3, w4, w5, w6, b1, b2, b3, b4, b5];
        let adam_state = adam::AdamState::new(
            params
                .iter()
                .map(|v| v.get_variable_array().unwrap())
                .collect::<Vec<_>>()
                .as_slice(),
        );

        let (x1, x2, y) = inputs(g);
        // Twin #1
        let z1 = conv_pool(x1, w1, b1);
        let z2 = conv_pool(z1, w2, b2);
        let z3 = conv_pool(z2, w3, b3);
        let twin_1_z4 = conv_final(z3, w4, b4);
        let flatten_twin_1 = g.reshape(twin_1_z4, &[-1, 256 * 6 * 6]); // flatten
        let _dense_twin_1 = logits(w5, flatten_twin_1, b5);
        let sigmoid_twin_1 = g.sigmoid(flatten_twin_1);

        // Twin #2
        let z1 = conv_pool(x2, w1, b1);
        let z2 = conv_pool(z1, w2, b2);
        let z3 = conv_pool(z2, w3, b3);
        let twin_2_z4 = conv_final(z3, w4, b4);
        let flatten_twin_2 = g.reshape(twin_2_z4, &[-1, 256 * 6 * 6]); // flatten
        let _dense_twin_2 = logits(w5, flatten_twin_2, b5);
        let sigmoid_twin_2 = g.sigmoid(flatten_twin_2);

        // Siamese Distance
        let pre_weighted_l1_dist = g.abs(sigmoid_twin_2 - sigmoid_twin_1);
        let weighted_l1_dist = g.matmul(w6, pre_weighted_l1_dist);
        let final_prediction = weighted_l1_dist;
        let loss = g.sigmoid_cross_entropy(final_prediction, y);
        let grads = &g.grad(&[&loss], params);
        let _update_ops: &[Tensor] =
            &adam::Adam::default().compute_updates(params, grads, &adam_state, g);
    })
}
