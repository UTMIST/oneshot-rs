extern crate autograd as ag;
extern crate ndarray;

mod layers;

use super::data::NDArr;
use ag::{ndarray_ext as array, optimizers::adam, tensor::Variable};
use ndarray::s;

type Tensor<'graph> = ag::Tensor<'graph, f32>;

pub fn train(train_x1: NDArr, train_x2: NDArr, train_y: NDArr, batch_size: usize, epochs: usize) {
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
        let b1 = zeroes!([1, 64, 96, 96]);
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
        let sig_1 = layers::sigmoid_layer(x1, params);
        let sig_2 = layers::sigmoid_layer(x2, params);

        // Siamese Distance
        let pre_weighted_l1_dist = g.abs(sig_2 - sig_1);
        let final_prediction = g.matmul(params[5], pre_weighted_l1_dist);
        let loss = g.sigmoid_cross_entropy(final_prediction, y);
        let grads = &g.grad(&[&loss], params);
        let update_ops: &[Tensor] =
            &adam::Adam::default().compute_updates(params, grads, &adam_state, g);

        // Computing the number of batches.
        let mut batch_count = train_y.shape()[0] / batch_size;
        if train_y.shape()[0] % batch_size != 0 {
            batch_count += 1;
        }

        for epoch in 0..epochs {
            for batch_idx in 0..batch_count {
                let start_idx = batch_size * batch_idx;
                let end_idx = if batch_idx + 1 < batch_count {
                    start_idx + batch_size
                } else {
                    train_y.shape()[0]
                };

                let x1_batch = train_x1
                    .slice(s![start_idx..end_idx, .., .., ..])
                    .into_dyn();
                let x2_batch = train_x2
                    .slice(s![start_idx..end_idx, .., .., ..])
                    .into_dyn();
                let y_batch = train_y.slice(s![start_idx..end_idx, ..]).into_dyn();

                println!("\tCompleted batch {}.", batch_idx);
                g.eval(
                    update_ops,
                    &[x1.given(x1_batch), x2.given(x2_batch), y.given(y_batch)],
                );
            }
            println!("Finished epoch {}.", epoch);
        }

        let predictions = g.argmax(final_prediction, -1, true);
        let accuracy = g.reduce_mean(&g.equal(predictions, &y), &[0, 1], false);
        println!(
            "Test Accuracy: {:?}",
            accuracy.eval(&[
                x1.given(train_x1.view()),
                x2.given(train_x2.view()),
                y.given(train_y.view())
            ])
        );
    })
}
