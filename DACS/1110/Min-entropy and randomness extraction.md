# 19. Min-entropy and randomness extraction

**Status:** Core, Bridge.

### Concepts

- Weak random source versus uniform distribution.
- Average uncertainty (Shannon entropy) versus maximum guessing probability
  (min-entropy).
- Seeded extractor and strong extractor.
- Public independent seed.
- Statistical closeness to uniform.

### Candidate results

**Min-entropy.**

$$
H_\infty(X)=-\log\max_xP(X=x).
$$

Thus the optimal one-shot guessing probability is `2^{-H_infinity(X)}`.

**Strong seeded extraction.** An extractor should make `(S,Ext(X,S))` close to
`(S,U_l)`, so the seed may be revealed.

**Leftover Hash Lemma.** Let `H` be chosen uniformly from a suitable
2-universal family independently of `X`. One standard convention gives

$$
\Delta\bigl((H,H(X)),(H,U_\ell)\bigr)
\leq \frac12\,2^{(\ell-H_\infty(X))/2}.
$$

Consequently, approximately

$$
\ell\leq H_\infty(X)-2\log(1/\varepsilon)
$$

bits can be extracted within statistical distance `epsilon`, up to additive
constants determined by the theorem convention.

**Classical side-information form.** If an observer holds correlated classical
side information `E` and the public hash seed is independent of the joint pair
`(X,E)`, the relevant source quality is average conditional min-entropy. A
corresponding form controls

$$
\Delta\bigl((E,H,H(X)),(E,H,U_\ell)\bigr)
$$

by the same type of expression with conditional min-entropy
`\widetilde H_\infty(X|E)`. This is the form needed for privacy amplification;
the unconditional statement alone only supports extraction when no correlated
observer information is present. Quantum side information requires a stronger
version outside the proposed scope.

**Public seed principle.** Security does not require the independently chosen
hash-function seed to remain secret because the joint output includes it.

### LEPL1109 dependency

- Discrete/joint distributions and independence:
  [probability foundations](../../EPL/1109/Probability%20foundations%20and%20random%20variables.md)
  and [dependence](../../EPL/1109/Dependence%20and%20multivariate%20probability.md).
- Shannon entropy, statistical distance, and universal hashing are new FoL
  prerequisites and should precede the lemma.

### What is new beyond LEPL1109

- Min-entropy and worst-case predictability.
- Statistical extraction from weak randomness.
- A theorem joining hashing, entropy, and indistinguishability.

### FoC reuse

- Privacy amplification and key derivation from imperfect randomness.
- Information-theoretic secrecy and public discussion.
- Distinction between high Shannon entropy, high min-entropy, statistical
  uniformity, and computational pseudorandomness.

### Bibliography

[B11, Ch. 6](README.md#b11), [B17](README.md#b17).
