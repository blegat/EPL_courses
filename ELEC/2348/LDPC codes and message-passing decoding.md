# LDPC codes and message-passing decoding

## Topics and results

- An LDPC code has a sparse parity-check matrix. Its Tanner graph is bipartite,
  with variable nodes for bits and check nodes for parity equations (`LDPC`,
  pp. 7–12).
- Sparse neighborhoods permit iterative local decoding with complexity roughly
  linear in block length per iteration.
- Hard-decision bit flipping exchanges parity-consistency information; soft
  belief propagation exchanges likelihood or log-likelihood messages
  (`LDPC`, pp. 15–27).
- Variable-to-check messages combine channel evidence with other checks;
  check-to-variable messages impose parity using the other neighboring bits.
- On a cycle-free graph, sum-product gives exact marginals. LDPC graphs contain
  cycles, so iterations are approximate and short cycles degrade independence
  assumptions.
- Degree distributions, girth, block length and stopping criteria jointly
  determine threshold, error floor and implementation cost.
