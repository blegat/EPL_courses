# Support-vector machines and kernel classification

## Topics and results

- A hard-margin SVM maximizes the geometric margin between two classes; scaling
  fixes the functional margin and turns the problem into convex quadratic
  optimization (`APPLICATIONS`, pp. 24–39).
- Support vectors are the active training points that determine the separating
  hyperplane. KKT conditions and duality expose the dependence on pairwise
  inner products (`APPLICATIONS`, pp. 40–55).
- Slack variables and the penalty $C$ give a soft-margin formulation for
  nonseparable data, trading margin width against violations (`APPLICATIONS`,
  pp. 56–63).
- Replacing inner products by a kernel produces nonlinear decision boundaries.
  The representer theorem gives a finite expansion even for an
  infinite-dimensional RKHS (`APPLICATIONS`, pp. 64–83).
- Kernel and hyperparameter choice control the induced geometry, expressivity,
  computational cost, and generalization behaviour.

## Connections

- Kernel mechanism: [Feature spaces and the kernel trick](Feature%20spaces%20and%20the%20kernel%20trick.md)
- Scalable approximation: [Bochner's theorem and random Fourier features](Bochner's%20theorem%20and%20random%20Fourier%20features.md)
- Classification treatment: [LINFO2262 — support-vector machines and kernels](../../INFO/2262/Support-vector%20machines%20and%20kernels.md)
