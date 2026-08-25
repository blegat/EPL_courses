# Las Vegas algorithms, hashing, and derandomization

## Topics and results

- A Las Vegas algorithm always returns a valid result but has random running
  time; a Monte Carlo algorithm has controlled output error (`S9`, pp. 2–4).
- Coping with a poor random choice or restarting after detectable failure are
  standard Las Vegas patterns (`S9`, p. 5).
- Hash-table collision strategies include chaining, probing and cuckoo hashing;
  a randomized universal family prevents a fixed adversarial key set from
  forcing collisions (`S9`, pp. 6–8).
- Pseudorandom generators expand a short random seed into a deterministic
  sequence intended to be indistinguishable for the application; Blum–Blum–Shub
  and practical generators illustrate different guarantees (`S9`, pp. 9–13).
- Derandomization fixes random choices by exhaustive seed search or the method
  of conditional expectations while preserving a proven expected objective
  (`S9`, pp. 14–16).

## Related courses

- Hashing continuation: [LDACS1110 — universal hashing and randomized hash maps](../../DACS/1110/Universal%20hashing%20and%20randomized%20hash%20maps.md)
- Cryptographic continuation: [LDACS1110 — computational pseudorandomness and reductions](../../DACS/1110/Computational%20pseudorandomness%20and%20reductions.md)
