# OneShot Rust

## Usage

### Setup

```sh
git submodule update --init
```

### Unpacking Dataset

Review the contents of `oneshot-data/data_augmented` and choose a dataset.

```sh
unzip oneshot-data/data_augmented/<dataset>.zip
```

### Random Sampling

From the `<dataset>`, choose a `<script1>` and `<script2>` and use `0` for negative and `1` for positive.

```sh
cargo run -- <dataset> <positive> <script1> [<script2>]
```
