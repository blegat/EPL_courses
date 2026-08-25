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

- Classification survey: [LINFO2262 — support-vector machines and kernels](../../INFO/2262/Support-vector%20machines%20and%20kernels.md)
- Linear precursor: [Linear regression, optimization, and the perceptron](Linear%20regression,%20optimization,%20and%20the%20perceptron.md)
- Learning-problem prerequisite: [LEPL1109 — supervised-learning formulation](../../EPL/1109/Supervised-learning%20formulation.md)
- Kernel connection: [LEPL1109 — Gaussian-process regression](../../EPL/1109/Gaussian-process%20regression.md)
- Sequential approximation: [LINMA2725 — value-function approximation architectures](../../INMA/2725/Value-function%20approximation%20architectures.md)
- Sequential approximation: [LINMA2725 — deep and convex Q-learning](../../INMA/2725/Deep%20and%20convex%20Q-learning.md)
- Kernel geometry and the kernel trick: [LINMA2472 — feature spaces and the kernel trick](../../INMA/2472/Feature%20spaces%20and%20the%20kernel%20trick.md)
- Visual-feature application: [LELEC2885 — hand-crafted features and classical vision learning](../2885/Hand-crafted%20features%20and%20classical%20vision%20learning.md)
