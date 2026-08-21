# Feature spaces and the kernel trick

## Topics and results

- Linear regression, PCA, and linear classification depend on linear
  combinations and inner products (`KERNELS`, pp. 2–17).
- An inner product equips a finite-dimensional vector space with norms,
  distances, and angles; every such product can be represented by a symmetric
  positive-definite matrix (`KERNELS`, pp. 18–27).
- A feature map embeds structured or nonlinearly arranged data into a Euclidean
  or Hilbert feature space where a linear method can operate (`KERNELS`,
  pp. 28–38).
- The Gram matrix records all pairwise feature-space inner products. Algorithms
  expressible solely through these products can use the kernel matrix without
  explicitly constructing a possibly very high-dimensional feature map
  (`KERNELS`, pp. 39–46).
- Kernels can act on nonvectorial objects such as strings; the spectrum kernel
  compares strings through counts of fixed-length subsequences.

## Connections

- Mathematical foundation: [Reproducing-kernel Hilbert spaces and kernel construction](Reproducing-kernel%20Hilbert%20spaces%20and%20kernel%20construction.md)
- Classification application: [Support-vector machines and kernel classification](Support-vector%20machines%20and%20kernel%20classification.md)
- Nonlinear spectral method: [Kernel PCA and nonlinear structure discovery](Kernel%20PCA%20and%20nonlinear%20structure%20discovery.md)
- Related survey: [LELEC2870 — support-vector machines and kernels](../../ELEC/2870/Support-vector%20machines%20and%20kernels.md)
