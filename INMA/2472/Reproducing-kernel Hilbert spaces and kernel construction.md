# Reproducing-kernel Hilbert spaces and kernel construction

## Topics and results

- A Hilbert space extends Euclidean geometry to complete, possibly
  infinite-dimensional spaces of functions (`KERNELS`, pp. 47–57).
- In a reproducing-kernel Hilbert space (RKHS), evaluation is continuous and is
  represented by an inner product: $f(x)=\langle f,k(x,\cdot)\rangle_\mathcal H$.
  The canonical feature map is $x\mapsto k(x,\cdot)$ (`KERNELS`, pp. 58–66).
- A continuous function is a valid kernel when every finite Gram matrix it
  induces is symmetric positive semidefinite (`KERNELS`, pp. 67–70).
- Polynomial and Gaussian kernels correspond to implicit nonlinear feature
  maps. Nonnegative scaling, sums, products, positive-coefficient polynomials,
  normalization, and exponentiation provide closure rules for constructing new
  kernels (`KERNELS`, pp. 71–81).

## Connections

- Finite-dimensional motivation: [Feature spaces and the kernel trick](Feature%20spaces%20and%20the%20kernel%20trick.md)
- Finite representers: [Kernel ridge regression and the representer theorem](Kernel%20ridge%20regression%20and%20the%20representer%20theorem.md)
- Spectral view: [Bochner's theorem and random Fourier features](Bochner's%20theorem%20and%20random%20Fourier%20features.md)
