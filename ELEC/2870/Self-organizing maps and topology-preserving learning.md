# Self-organizing maps and topology-preserving learning

## Topics and results

- A self-organizing map places prototypes on a fixed low-dimensional grid and
  updates the best-matching unit together with its grid neighbors (`VQ`,
  pp. 57–65).
- The learning rate and neighborhood radius decrease over time, producing an
  ordering phase followed by finer quantization (`VQ`, pp. 61–69).
- Unlike ordinary K-means or competitive learning, the grid induces topological
  organization: nearby grid units are encouraged to represent nearby regions
  of input space (`VQ`, pp. 69–76; `NLDR`, pp. 22–26).
- Component planes, U-matrices, labels, and hit counts support visualization,
  but grid topology can distort the original geometry and must be assessed
  (`VQ`, pp. 70–77).

## Related courses

- Foundation: [Vector quantization and prototype clustering](Vector%20quantization%20and%20prototype%20clustering.md)
- Embedding context: [Nonlinear dimensionality reduction and quality assessment](Nonlinear%20dimensionality%20reduction%20and%20quality%20assessment.md)
- Prerequisite: [LEPL1109 — unsupervised learning](../LEPL1109/Unsupervised%20learning.md)

