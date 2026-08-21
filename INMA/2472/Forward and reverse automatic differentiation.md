# Forward and reverse automatic differentiation

## Topics and results

- Finite differences approximate derivatives but suffer truncation and rounding
  errors; symbolic differentiation can create large expressions. Automatic
  differentiation evaluates exact chain-rule derivatives of a program (`AD`,
  “Differentiation approaches”).
- A computation graph decomposes a program into elementary operations with
  known local derivatives.
- Forward mode propagates Jacobian–vector products (JVPs) with tangents; reverse
  mode records the primal computation and propagates vector–Jacobian products
  (VJPs) with adjoints (`AD`, “Two different takes on the multivariate chain rule”).
- Forward mode is favourable for few inputs and reverse mode for few outputs;
  gradients of scalar losses therefore motivate reverse-mode backpropagation.
- Matrix multiplication, broadcasting, and other vectorized operations require
  pullback rules that respect shapes and accumulated dependencies (`AD`,
  “Neural network”).
- Differentiability is a property of the executed numerical program; branches
  and discontinuities require care even when AD returns a local derivative.

## Connections

- Beyond first derivatives: [Higher-order and sparse automatic differentiation](Higher-order%20and%20sparse%20automatic%20differentiation.md)
- Differentiating solvers: [Implicit differentiation and optimization sensitivity](Implicit%20differentiation%20and%20optimization%20sensitivity.md)
- Generative score gradients: [Score matching and diffusion models](Score%20matching%20and%20diffusion%20models.md)
- Neural-network use: [LELEC2870 — deep learning architectures and training](../../ELEC/2870/Deep%20learning%20architectures%20and%20training.md)
