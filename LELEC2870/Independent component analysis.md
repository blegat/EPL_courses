# Independent component analysis

## Topics and results

- ICA seeks a linear instantaneous transformation whose outputs are as
  statistically independent as possible; independence is stronger than
  uncorrelatedness, so PCA alone generally does not solve ICA (`ICA`, pp. 2–9).
- The source-separation model observes unknown mixtures $x=As$ and estimates an
  unmixing transform up to unavoidable permutation and scaling ambiguities
  (`ICA`, pp. 10–18).
- Whitening is a preprocessing step that removes second-order dependence and
  reduces the remaining search to rotations (`ICA`, pp. 19–25).
- Independence objectives include mutual information/KL divergence,
  non-Gaussianity, kurtosis, and negentropy approximations (`ICA`, pp. 26–42).
- Identifiability relies on non-Gaussian sources (with at most one Gaussian
  component under the standard model); extensions and examples cover blind
  source separation and signal applications (`ICA`, pp. 43–55).

## Related courses

- Linear precursor: [Principal component analysis](Principal%20component%20analysis.md)
- Probability prerequisite: [LEPL1109 — dependence and multivariate probability](../LEPL1109/Dependence%20and%20multivariate%20probability.md)
- Information-theoretic formulation: [LDACS1110 — KL divergence and mutual information](../LDACS1110/KL%20divergence,%20cross-entropy,%20and%20mutual%20information.md)

