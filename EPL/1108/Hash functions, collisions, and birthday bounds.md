# Hash functions, collisions, and birthday bounds

## Topics and results

- A hash maps a large key universe to a smaller output space. Collisions are
  unavoidable; the question is how likely or difficult they are to find
  (`HASH`, pp. 3–7).
- For independent uniform $n$-bit outputs, a fixed-target hit takes about
  $2^n$ trials, while an arbitrary collision appears after about $2^{n/2}$
  trials (`HASH`, pp. 8–14).
- The birthday estimate follows from multiplying no-collision probabilities
  or approximating them exponentially.
- Hash tables need distributional collision control; cryptographic hashing
  asks for adversarial preimage and collision resistance (`HASH`, pp. 15–22).

## Related courses

- Algorithmic continuation: [LDACS1110 — universal hashing and randomized hash maps](../../DACS/1110/Universal%20hashing%20and%20randomized%20hash%20maps.md)
