# Universal hashing and randomized hash maps

## Concepts

- Family of hash functions with a uniformly random public choice of function.
- Universal and 2-universal collision guarantees.
- Pairwise independence versus full independence.
- Chaining and load factor in hash maps.
- Explicit separation among deterministic hashing, universal hashing,
  cryptographic collision resistance, random oracles, and PRFs.

## Principal results

**Universal hashing.** A family `H` mapping `U` to `[m]` is universal if for
distinct `x,x'`,

$$
P_{h\leftarrow\mathcal H}[h(x)=h(x')]
\leq\frac1m.
$$

**Expected collisions.** For a fixed stored set and a fresh/randomly selected
universal hash function, linearity of expectation controls the expected number
of keys colliding with a query key.

**Expected lookup with chaining.** Under simple uniform/universal-hashing
assumptions, expected lookup cost is `O(1+alpha)` at load factor `alpha=n/m`.

**Pairwise-independent construction.** Over a finite field, affine
maps `h_{a,b}(x)=ax+b` with appropriate random parameters provide a simple
pairwise-independent family.

## Prerequisites from LEPL1109

- Uniform discrete variables, indicators, independence, and expectation:
  [probability foundations](../../EPL/1109/Probability%20foundations%20and%20random%20variables.md).
- Algorithmic data structures come from LEPL1402.

## Developments beyond LEPL1109

- Random function families and limited independence.
- Collision guarantees over the random function choice.
- Algorithmic use of weak randomness.

## Connection to foundations of cryptography

- Prepares keyed function families, universal-hash MACs, extraction, and the
  distinction from cryptographic hashes.
- The cryptography part develops collision resistance, preimage resistance,
  random-oracle modeling, PRFs, and cryptographic hash constructions.

## References

[B1, Ch. 11](README.md#b1), [B11, Ch. 6](README.md#b11), [B17, universal hashing](README.md#b17).

## Related courses

- Introductory collision analysis: [LEPL1108 — hash functions, collisions, and birthday bounds](../../EPL/1108/Hash%20functions,%20collisions,%20and%20birthday%20bounds.md)
