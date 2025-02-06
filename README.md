# Noisette

[![Crates.io](https://img.shields.io/crates/v/noisette)](https://crates.io/crates/noisette)
[![Docs.rs](https://docs.rs/noisette/badge.svg)](https://docs.rs/noisette)
![License](https://img.shields.io/crates/l/noisette)

A library for generating 2D noise functions with analytical gradients.

## Usage

This crate provides a simple API for creating `Noise` objects which can be sampled within the unit square for both their value and analytical gradient.

```rust
let mut rng = thread_rng();

let noise = Perlin::new(SHAPE, &mut rng);

let value = noise.sample([0.25, 0.75]);
let gradient = noise.gradient([0.25, 0.75]);
```

### Stacks

You can stack multiple weighted noise functions together into a single noise function using the `Stack` struct:

```rust
let noise = Stack::new(
    GradientFunction::Noop,
    vec![
        (Box::new(Perlin::new((5, 5), &mut rng)), 1.0),
        (Box::new(Perlin::new((7, 7), &mut rng)), 0.5),
        (Box::new(Perlin::new((11, 11), &mut rng)), 0.25),
        (Box::new(Perlin::new((23, 23), &mut rng)), 0.125),
        (Box::new(Perlin::new((43, 43), &mut rng)), 0.0625),
    ]
);
```

The `GradientFunction` enum can be used to apply an additional weighting to the sampled value of each noise function influenced by the magnitude of the current gradient.
This can be can be useful for creating effects such as ridges or terraces.
The `Noop` variant does not apply any additional weighting.

#### Gradient Functions

- `Noop`: No change in weighting.
- `Inverse`: 1/(1 + k \* x).
- `Exp`: exp(-(scale \* x)^2).
- `Sigmoid`: 1/(1 + exp(-k \* x)).
- `Tanh`: (tanh(k \* x) + 1) / 2.
- `Cosine`: 0.5 \* (cos(frequency \* x) + 1).
- `Quadratic`: 1/(1 + k \* x^2).
- `Arctan`: (atan(k \* x) / pi + 0.5).

## Features

Left: Each of the images below show the sampled noise function over the unit square, tiled 2 times in each direction to show the periodicity of the noise.

Right: The magnitude of the gradient of the noise function scaled to the range [0, 1].

Top: A single noise function.

Bottom: A stack of multiple noise functions with different weights.

### Perlin Noise

![Perlin Noise](./assets/images/perlin-combined.png)

![Perlin Noise](./assets/images/perlin_stack-combined.png)

### Simplex Noise

![Simplex Noise](./assets/images/simplex-combined.png)

![Simplex Noise](./assets/images/simplex_stack-combined.png)

### OpenSimplex Noise

![OpenSimplex Noise](./assets/images/open_simplex-combined.png)

![OpenSimplex Noise](./assets/images/open_simplex_stack-combined.png)

> Note: Simplex noise is inherently not tileable, however the OpenSimplex implementation is tileable (by using 4D space).

### Worley Noise

![Worley Noise](./assets/images/worley-combined.png)

![Worley Noise](./assets/images/worley_stack-combined.png)

> Note: The gradient of the Worley noise can not be determined analytically, so the gradient is approximated using central differences.

## Examples

You can run the examples to generate images of the noise functions.

First, clone the repository and set the current directory to the root of the repository:

```sh
git clone https://github.com/FreddyWordingham/noise.git
cd noise
```

Make sure to create the `./output` directory for the images to be saved to:

```sh
mkdir ./output
```

Then, build the examples:

```sh
cargo build --release --examples
```

And run the example you want to generate an image.
For example, to generate an image of stacked Simplex noise:

```sh
cargo run --release --example open_simplex_stack
```
