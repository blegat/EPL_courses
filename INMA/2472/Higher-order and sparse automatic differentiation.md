# Higher-order and sparse automatic differentiation

## Topics and results

- Nesting forward and reverse modes produces Hessian–vector products, gradients
  of gradients, and full second derivatives with different time and memory
  trade-offs (`AD`, “Second-order”).
- Forward-over-reverse efficiently computes Hessian–vector products for scalar
  objectives without materializing the full Hessian.
- Known Jacobian sparsity permits several structurally orthogonal columns or
  rows to be compressed into a single directional derivative (`SPARSE`,
  “Sparse Jacobian”).
- Recovering entries from compressed derivatives becomes a graph-coloring
  problem. Jacobian column/row coloring and Hessian star or acyclic coloring
  encode which variables can share a seed (`SPARSE`, “Coloring problems”).
- Fewer colors mean fewer AD sweeps, while substitution and recovery complexity
  distinguish direct from more aggressive coloring schemes.

## Connections

- First-order modes: [Forward and reverse automatic differentiation](Forward%20and%20reverse%20automatic%20differentiation.md)
- Repeated linear solves: [Implicit differentiation and optimization sensitivity](Implicit%20differentiation%20and%20optimization%20sensitivity.md)
