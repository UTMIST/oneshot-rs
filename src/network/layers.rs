extern crate autograd as ag;
use ag::Graph;

type Tensor<'graph> = ag::Tensor<'graph, f32>;

/// Compute convolutional layer with max-pooling.
pub fn conv_pool<'g>(x: Tensor<'g>, w: Tensor<'g>, b: Tensor<'g>) -> Tensor<'g> {
    let g = x.graph();
    let y1 = g.conv2d(x, w, 0, 1) + b;
    let y2 = g.relu(y1);
    g.max_pool2d(y2, 2, 0, 2)
}

/// Compute the final convolutional layer.
pub fn conv_final<'g>(x: Tensor<'g>, w: Tensor<'g>, b: Tensor<'g>) -> Tensor<'g> {
    let g = x.graph();
    let y1 = g.conv2d(x, w, 0, 1) + b;
    g.relu(y1)
}

// Load inputs.
pub fn inputs(g: &Graph<f32>) -> (Tensor, Tensor, Tensor) {
    let x1 = g.placeholder(&[-1, 1, 105, 105]);
    let x2 = g.placeholder(&[-1, 1, 105, 105]);
    let y = g.placeholder(&[-1, 1]);
    (x1, x2, y)
}

/// Compute the final sigmoid layer.
pub fn sigmoid_layer<'g>(x: Tensor<'g>, params: &[Tensor<'g>]) -> Tensor<'g> {
    let g = x.graph();
    let z1 = conv_pool(x, params[0], params[6]);
    let z2 = conv_pool(z1, params[1], params[7]);
    let z3 = conv_pool(z2, params[2], params[8]);
    let twin_2_z4 = conv_final(z3, params[3], params[9]);
    let flattened = g.reshape(twin_2_z4, &[-1, 256 * 6 * 6]); // flatten
    let dense = flattened.graph().matmul(params[4], flattened) + params[10];
    g.sigmoid(dense)
}
