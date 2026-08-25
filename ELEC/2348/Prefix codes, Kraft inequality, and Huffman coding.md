# Prefix codes, Kraft inequality, and Huffman coding

## Topics and results

- A uniquely decodable code permits unambiguous concatenation; a prefix code
  ensures this instantaneously through its code tree.
- Binary prefix-code lengths satisfy Kraft's inequality
  $$\sum_x2^{-\ell(x)}\leq1,$$
  and every integer length set satisfying it has a prefix code.
- Shannon lengths $\ell(x)=\lceil-\log_2p(x)\rceil$ yield expected length
  $H(X)\leq L<H(X)+1$ (`SOURCE`, pp. 26–29).
- Huffman's greedy merging algorithm constructs a prefix code minimizing the
  expected length for a known memoryless source (`SOURCE`, pp. 29–31).
- Coding blocks reduces per-symbol integer-length overhead, approaching entropy
  at the cost of larger codebooks and delay.
- Adaptive Huffman coding updates a tree when source probabilities are not
  known in advance.
