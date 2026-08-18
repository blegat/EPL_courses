# Vector quantization and prototype clustering

## Topics and results

- Vector quantization maps a continuous or large input space to finitely many
  prototypes; nearest-prototype assignment induces Voronoi cells and
  quantization error (`VQ`, pp. 3–15).
- Distance choice determines the geometry and must reflect scaling and the data
  representation (`VQ`, pp. 16–22).
- Lloyd's alternating principle assigns samples to nearest prototypes and then
  replaces each prototype by its cell centroid. Each step cannot increase the
  squared distortion, but convergence is only to a local optimum (`VQ`,
  pp. 23–35).
- Initialization and empty/poorly populated cells matter; repeated or informed
  starts reduce, but do not remove, local-minimum sensitivity (`VQ`, pp. 36–42).
- Online competitive learning gives stochastic prototype updates. Frequency
  sensitivity, soft competition, stochastic relaxation, and neural gas modify
  winner-take-all behavior to improve prototype use or topology (`VQ`,
  pp. 43–56).

## Related courses

- Prerequisite: [LEPL1109 — unsupervised learning](../LEPL1109/Unsupervised%20learning.md)
- Topological extension: [Self-organizing maps and topology-preserving learning](Self-organizing%20maps%20and%20topology-preserving%20learning.md)

