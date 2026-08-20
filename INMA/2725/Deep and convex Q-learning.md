# Deep and convex Q-learning

## Topics and results

- Approximate Q-learning combines a Bellman optimality target with a
  parameterized Q-function and greedy policy improvement (`CH3B`, pp. 33–34).
- DQN uses a multilayer network and stabilization devices such as delayed/target
  parameters to weaken the moving-target feedback (`CH3B`, pp. 35–38).
- The Bellman inequalities admit an infinite-dimensional linear-program view;
  finite feature classes yield relaxations (`CH3B`, pp. 40–43).
- Convex Q-learning replaces direct fixed-point iteration by an optimization
  formulation with sampled constraints and regularization (`CH3B`, pp. 40–47).
- Batch convex Q-learning and kernel variants fit from blocks of observed
  state–action transitions (`CH3B`, pp. 43–49).

## Related courses

- Neural-network background: [LELEC2870 — deep learning architectures and training](../../ELEC/2870/Deep%20learning%20architectures%20and%20training.md)
- Kernel background: [LELEC2870 — support-vector machines and kernels](../../ELEC/2870/Support-vector%20machines%20and%20kernels.md)
