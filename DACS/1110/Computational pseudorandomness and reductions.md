# 20. Computational pseudorandomness and reductions

**Status:** Bridge overview in FoL; cryptographic formalization belongs to FoC.

### FoL handoff concepts

- Statistical indistinguishability quantifies all tests, including unbounded
  ones.
- Computational indistinguishability quantifies efficient tests and has no
  simple distribution metric known to the test.
- Efficient probabilistic algorithm and finite distinguisher advantage.
- Hybrid/telescoping argument from triangle inequality.
- Reduction: convert an adversary against a construction into an algorithm
  against an assumed primitive while tracking probability and running time.

### Candidate results

**Computational indistinguishability, finite-security preview.** Relative to a
specified efficient distinguisher class, two distributions are
`epsilon`-indistinguishable when every distinguisher has advantage at most
`epsilon`:

$$
|P(D(X)=1)-P(D(Y)=1)|\leq\varepsilon.
$$

**Hybrid lemma.** If a sequence contains `m` adjacent game hops and the total
endpoint advantage is `epsilon`, at least one adjacent hop has advantage at
least `epsilon/m`. Conversely, bounding each hop by `epsilon_i` bounds the
endpoint difference by `sum_i epsilon_i`.

### Boundary with FoC

FoL owns the statistical/computational contrast, finite efficient-distinguisher
advantage, and the generic hybrid/telescoping lemma. FoC owns asymptotic
security parameters and negligible functions, cryptographic PRGs/PRFs/PRPs,
random oracles, formal security games, concrete constructions, and reductions
to cryptographic assumptions.

### LEPL1109 dependency

- LEPL1109 only treats numerical PRNGs for simulation:
  [simulation](../../EPL/1109/Simulation%20and%20bootstrap.md)
  [STAT, pp. 107-108].
- This must not be mistaken for cryptographic pseudorandomness.

### FoC reuse

Directly foundational for every computational-security topic in part B.

### Bibliography

[B11, Chs. 2-7](README.md#b11), [B17, introductory chapters](README.md#b17), [B18](README.md#b18).
