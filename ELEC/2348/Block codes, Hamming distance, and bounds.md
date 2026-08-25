# Block codes, Hamming distance, and bounds

## Topics and results

- A binary $(n,M,d)$ block code maps one of $M$ messages to an $n$-bit word;
  its rate is $(\log_2M)/n$ and minimum Hamming distance is $d$ (`BLOCK`,
  pp. 4–10).
- Maximum-likelihood decoding chooses the codeword most likely to have produced
  the observation; on a binary symmetric channel this is nearest-neighbor
  decoding (`BLOCK`, pp. 11–18).
- Minimum distance $d$ detects up to $d-1$ errors and corrects
  $t=\lfloor(d-1)/2\rfloor$ errors.
- Disjoint Hamming balls of radius $t$ yield the sphere-packing bound
  $$M\sum_{i=0}^t\binom ni\leq2^n.$$
  Equality defines a perfect code (`BLOCK`, pp. 19–27).
- Bounds reveal incompatible goals among rate, distance and block length, but
  do not alone provide an efficient encoder or decoder.
