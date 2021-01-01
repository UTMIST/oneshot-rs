# Rust for Machine Learning: Benchmarking Performance in One-shot

A Rust implementation of [Siamese Neural Networks for One-shot Image Recognition](https://www.cs.cmu.edu/~rsalakhu/papers/oneshot1.pdf) for benchmarking performance and results.

## Abstract

Python is a very flexible scripting language, suitable for machine learning experiments. However, it's widely known that the semantic properties of Python and of interpreted languages in general do not lend themselves well in performance, frugality, or scaling. While various Python libraries have been implemented in performant languages such as C++, there may remain sizeable gains in performance, safety, and efficiency in using modern performant languages directly. Rust is a language that in 2020 seems mature enough to begin to be used seriously. We aim to implement [Siamese Neural Networks for One-shot Image Recognition](https://www.cs.cmu.edu/~rsalakhu/papers/oneshot1.pdf) using both Rust and Python (with popular libraries) and to compare their performance, efficiency, and anomalies (errors and discrepancies) to hopefully provide insight into whether or not it's worth using a more difficult programming language.

## Setup

```sh
git submodule update --init
```

#### Submodules

- [UTMIST/oneshot-data: Augmented Omniglot data set for one-shot learning](https://github.com/utmist/oneshot-data).

### Unpacking Dataset

Review the contents of `oneshot-data/data_augmented` and choose a `<background-dataset>` from the `*background*` datasets.

```sh
unzip oneshot-data/data_augmented/<background-dataset>.zip
```

## Running the Code

To select `<num-background-pairs>` pairs of images from `<background-dataset>` for a background set, run the following script.

```sh
cargo run -- <dataset-directory> <num-pairs>
```

### Library Modules

- `sampling`: Sample `<num-background-pairs>` image pairs in a 50%/50% positive/negative split.
  - Negative pairs using the same script strictly avoid using the same character.
  - Pairs are represented by filenames relative to the repository root.
- `data`: Given an image pair set, load the pairs into two parallel arrays of images (each image is an 105x105 matrix of `bool` values) along another parallel array of labels (`bool` values for positive/negative).
  - `bool` values were chosen because images in the Omniglot dataset are bilevel (black/white) and therefore each pixel can be represented by just one bit.
- `network`: (Work in Progress)

## Repositories & Resources

- [oneshot-data](https://github.com/utmist/oneshot-data)
- [oneshot-py](https://github.com/utmist/oneshot-py)
- [oneshot-rs](https://github.com/utmist/oneshot-rs)
- [Planning Notes](https://hackmd.io/@utmist/ByKkTiSzw)

## References

[Lake, B. M., Salakhutdinov, R., and Tenenbaum, J. B. (2015). Human-level concept learning through probabilistic program induction.](http://www.sciencemag.org/content/350/6266/1332.short) _Science_, 350(6266), 1332-1338.

- Update: [Lake, B. M., Salakhutdinov, R., and Tenenbaum, J. B. (2019). The Omniglot Challenge: A 3-Year Progress Report.](https://arxiv.org/abs/1902.03477) Preprint available on arXiv:1902.03477.
