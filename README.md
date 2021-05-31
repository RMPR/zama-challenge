# Zama-challenge
This is a simple library to perform inference on Neural Networks, it's a set 
of linear algebra primitives that can be used to manipulate layers.

# Usage
## To Run the test
Clone this repository then:
```
cargo test
```
To use the library, you will need to instantiate a Matrix struct, for the 
examples make sure you run:
```
use matrix::Matrix;
let m = matrix::Matrix::new(3, 3, 0.0);
```
Beforehand.

## Activations

You can run an activation function on each element by calling it on the Matrix. 
The currently implemented are: `tanh`, `relu`,  `sigmoid(sm)`.

## Linear combination 
You can combine a matrix with a scalar and a vector:
```
use matrix::Scalar;

(matrix::Scalar::new(4.0) * m).data;
let n = matrix::Matrix::new(3, 1, 0.0);
(m * n).data;
```
For now to view the elements you need to access the `data` field in next versions 
we will make it printable.

## Flattening
You can obtain a flat vector from a Matrix:
```
m.flatten();
```


