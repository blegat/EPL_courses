# Typical sequences and the noisy-channel coding theorem

## Topics and results

- The asymptotic equipartition property says that long IID sequences lie with
  high probability in a typical set of about $2^{nH(X)}$ elements, each having
  probability about $2^{-nH(X)}$ (`NOISY`, pp. 8–13).
- Joint typicality characterizes empirical compatibility with a joint law;
  independently generated sequences are jointly typical with exponentially
  small probability controlled by $I(X;Y)$ (`NOISY`, pp. 14–18).
- Random coding draws a codebook from a chosen input distribution and decodes
  by joint typicality.
- The noisy-channel coding theorem states that every rate $R<C$ admits codes
  whose error probability tends to zero as block length grows (`NOISY`,
  pp. 19–24).
- Conversely, reliable communication above capacity is impossible.
- The theorem is asymptotic and existential; finite-length code design must
  trade rate, error probability, delay and decoding complexity.

## Internal connections

- [Discrete memoryless channels and capacity](Discrete%20memoryless%20channels%20and%20capacity.md)
