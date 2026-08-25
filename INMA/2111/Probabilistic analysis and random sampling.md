# Probabilistic analysis and random sampling

## Topics and results

- Average-case analysis assumes a distribution on inputs; a randomized
  algorithm creates a distribution over executions for every fixed input
  (`S7`, pp. 3–4).
- Indicator variables and linearity of expectation count events without
  requiring independence (`S7`, pp. 5–7).
- Randomly permuting candidates makes the expected number of hires harmonic,
  $H_n=\Theta(\log n)$ (`S7`, pp. 6–8).
- In the secretary problem, observing roughly the first $n/e$ candidates and
  then taking the next record succeeds with probability approaching $1/e$
  (`S7`, pp. 9–11).
- Birthday collisions occur after about the square root of the range size;
  coupon collection takes expected time $nH_n=\Theta(n\log n)$ (`S7`,
  pp. 12–16).

## Related courses

- Further probabilistic techniques: [LDACS1110 — randomized algorithms and probabilistic analysis](../../DACS/1110/Randomized%20algorithms%20and%20probabilistic%20analysis.md)

## Internal connections

- [Monte Carlo algorithms and error amplification](Monte%20Carlo%20algorithms%20and%20error%20amplification.md)
- [Las Vegas algorithms, hashing, and derandomization](Las%20Vegas%20algorithms%2C%20hashing%2C%20and%20derandomization.md)
