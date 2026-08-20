# Support-vector machines and kernels

## Topics and results

- A hard-margin SVM selects, among separating hyperplanes, the one maximizing
  geometric margin. Support vectors are the active boundary examples (`SVM`,
  pp. 4–12).
- Soft-margin SVMs introduce slack variables and a penalty controlling the
  tradeoff between margin width and violations; unlike the hard-margin problem,
  they remain feasible for nonseparable data (`SVM`, pp. 13–18).
- The dual formulation depends on inner products. Replacing them by a kernel
  evaluates an implicit feature map without explicitly constructing the
  high-dimensional coordinates (`SVM`, pp. 19–26).
- Polynomial and Gaussian kernels are developed as examples. A valid kernel is
  symmetric and positive semidefinite (Mercer condition in the slides)
  (`SVM`, pp. 27–30).
- Extensions include multiclass one-versus-all schemes and recursive feature
  elimination; the concluding comparison contrasts convex SVM training with
  multilayer perceptrons (`SVM`, pp. 31–38).

## Related courses

- Linear precursor: [Linear regression, optimization, and the perceptron](Linear%20regression,%20optimization,%20and%20the%20perceptron.md)
- Prerequisite boundary: [LEPL1109 — explicitly extra or not covered](../LEPL1109/Explicitly%20extra,%20deferred,%20or%20not%20covered.md)
- Kernel connection: [LEPL1109 — Gaussian-process regression](../LEPL1109/Gaussian-process%20regression.md)
- Sequential approximation: [LINMA2725 — value-function approximation architectures](../LINMA2725/Value-function%20approximation%20architectures.md)
- Sequential approximation: [LINMA2725 — deep and convex Q-learning](../LINMA2725/Deep%20and%20convex%20Q-learning.md)
