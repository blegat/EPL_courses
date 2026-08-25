# Multilayer perceptrons and backpropagation

## Topics and results

- Nonlinear activations extend the linear model; multilayer perceptrons compose
  affine maps and nonlinear units (`MLP`, pp. 3–17).
- A network with one sufficiently wide hidden layer has the universal
  approximation property for continuous functions under the stated conditions,
  but this is an existence result rather than a training guarantee (`MLP`,
  pp. 18–20).
- Backpropagation is the chain-rule computation of derivatives from output to
  earlier layers; the resulting gradients drive iterative weight adjustment
  (`MLP`, pp. 21–28).
- Training methods include batch/stochastic first-order updates, momentum and
  adaptive step ideas, and second-order/Newton-style methods; line search and
  curvature trade computation for convergence behavior (`MLP`, pp. 29–38).
- Applications include nonlinear function approximation, lossy and lossless
  image-compression constructions, and one-step or recursive time-series
  forecasting (`MLP`, pp. 39–53).

## Related courses

- Foundation: [Linear regression, optimization, and the perceptron](Linear%20regression,%20optimization,%20and%20the%20perceptron.md)
- Learning-problem prerequisite: [LEPL1109 — supervised-learning formulation](../../EPL/1109/Supervised-learning%20formulation.md)
- Follow-on: [Deep learning architectures and training](Deep%20learning%20architectures%20and%20training.md)
- Sequential approximation: [LINMA2725 — value-function approximation architectures](../../INMA/2725/Value-function%20approximation%20architectures.md)
