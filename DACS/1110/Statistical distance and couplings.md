# Statistical distance and couplings

## Concepts

- Statistical/total-variation distance between discrete distributions.
- Event and test characterizations.
- Statistical versus computational indistinguishability.
- Coupling as a joint construction with prescribed marginals.
- Data processing under randomized mappings.

## Principal results

**Total-variation distance.**

$$
\Delta(P,Q)=\frac12\sum_x|P(x)-Q(x)|.
$$

**Event characterization.**

$$
\Delta(P,Q)=\max_A|P(A)-Q(A)|.
$$

**Distinguisher characterization.** For any test `T` with output in `{0,1}`,

$$
|P(T(X)=1)-P(T(Y)=1)|\leq\Delta(P,Q),
$$

and an optimal unbounded test attains equality.

**Data processing.** For any randomized mapping `K`,

$$
\Delta(KP,KQ)\leq\Delta(P,Q).
$$

**Triangle inequality.** This supports hybrid/game-hopping arguments.

**Coupling lemma.** For every coupling `(X,Y)`,
`Delta(P_X,P_Y) <= P(X != Y)`, and an optimal coupling attains equality.

## Prerequisites from LEPL1109

- Discrete distributions, events, and conditional probability:
  [probability foundations](../../EPL/1109/Probability%20foundations%20and%20random%20variables.md)
  [STAT, pp. 7-15].
- Classification tests provide useful intuition but total variation is new.

## Developments beyond LEPL1109

- A metric on distributions with operational test meaning.
- Statistical indistinguishability and data processing.
- Coupling and hybrids as proof tools.

## Connection to foundations of cryptography

- Real-versus-ideal definitions of information-theoretic security.
- Statistical secrecy, extractors, and privacy amplification.
- Game hops and distinguishing advantage.

## References

[B11, Chs. 3 and 6](README.md#b11), [B17, probability appendix](README.md#b17).
