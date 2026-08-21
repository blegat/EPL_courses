# Kernel PCA and nonlinear structure discovery

## Topics and results

- Classical PCA finds directions of maximal variance through an eigendecomposition
  of a centered covariance or Gram matrix (`APPLICATIONS`, pp. 84–91).
- Kernel PCA performs PCA on centered feature vectors without constructing them.
  Double-centering the kernel matrix is essential because centering in input
  space does not generally center nonlinear features (`APPLICATIONS`,
  pp. 92–99).
- Eigenvectors of the centered kernel matrix provide feature-space principal
  directions and coordinates for both training and new points
  (`APPLICATIONS`, pp. 100–104).
- Nonlinear kernels can uncover curved manifolds and support reconstruction-error
  or feature-space-distance scores for novelty and outlier detection
  (`APPLICATIONS`, pp. 105–107).

## Connections

- Underlying computation: [Feature spaces and the kernel trick](Feature%20spaces%20and%20the%20kernel%20trick.md)
- Related dimensionality reduction: [LELEC2870 — nonlinear dimensionality reduction and quality assessment](../../ELEC/2870/Nonlinear%20dimensionality%20reduction%20and%20quality%20assessment.md)
