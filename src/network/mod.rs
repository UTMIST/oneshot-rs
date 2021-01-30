extern crate autograd as ag;
extern crate ndarray;

mod layers;

use super::data::NDArr;
use ag::{ndarray_ext as array, optimizers::adam, tensor::Variable};
use ndarray::s;

type Tensor<'graph> = ag::Tensor<'graph, f32>;

pub fn train(train_x1: NDArr, train_x2: NDArr, train_y: NDArr) {

    
    ag::with(|g| {
        let rng = array::ArrayRng::<f32>::default();
        macro_rules! rand_normal {
            ($arr: expr) => {
                g.variable(rng.random_normal(&$arr, 0., 0.1));
            };
        }
        macro_rules! zeroes {
            ($arr: expr) => {
                g.variable(array::zeros(&$arr));
            };
        }

        // Weights/Biases for convolutional layers.
        let w1 = rand_normal!([64, 1, 10, 10]);
        let w2 = rand_normal!([128, 1, 7, 7]);
        let w3 = rand_normal!([128, 1, 4, 4]);
        let w4 = rand_normal!([256, 1, 4, 4]);
        let b1 = zeroes!([1, 64, 105, 105]);
        let b2 = zeroes!([1, 128, 42, 42]);
        let b3 = zeroes!([1, 128, 18, 18]);
        let b4 = zeroes!([1, 256, 6, 6]);

        // Weights/Biases for dense layers.
        let w5 = rand_normal!([4096, 256 * 6 * 6]);
        let w6 = rand_normal!([4096, 1]);
        let b5 = zeroes!([4096, 1]);

        // Collect parameters and add to adam_state.
        let params = &[w1, w2, w3, w4, w5, w6, b1, b2, b3, b4, b5];
        let adam_state = adam::AdamState::new(
            params
                .iter()
                .map(|v| v.get_variable_array().unwrap())
                .collect::<Vec<_>>()
                .as_slice(),
        );

        // Load inputs and compute sigmoid layers.
        let (x1, x2, y) = layers::inputs(g);
        let (_dense_1, sig_1) = layers::sigmoid_layer(x1, params);
        let (_dense_2, sig_2) = layers::sigmoid_layer(x2, params);

        // Siamese Distance
        let pre_weighted_l1_dist = g.abs(sig_2 - sig_1);
        let weighted_l1_dist = g.matmul(params[5], pre_weighted_l1_dist);
        let final_prediction = weighted_l1_dist;
        let loss = g.sigmoid_cross_entropy(final_prediction, y);
        let grads = &g.grad(&[&loss], params);
        let _update_ops: &[Tensor] =
            &adam::Adam::default().compute_updates(params, grads, &adam_state, g);

        // println!("{:?}", );
        for epoch in 0..1 {
            let x1_batch = train_x1.slice(s![0..500, .., .., ..]).into_dyn();
            let x2_batch = train_x2.slice(s![0..500, .., .., ..]).into_dyn();
            let y_batch = train_y.slice(s![0..500, ..]).into_dyn();

            println!("{:?}", x1_batch.shape());
            println!("{:?}", x2_batch.shape());
            println!("{:?}", y_batch.shape());

            g.eval(
                _update_ops,
                &[x1.given(x1_batch), x2.given(x2_batch), y.given(y_batch)],
            );
            println!("finish epoch {}", epoch);
        }
    })
}
