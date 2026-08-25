# Convolutional codes and trellis decoding

## Topics and results

- A convolutional encoder maps a stream to coded outputs using finite memory;
  generator polynomials specify its shift-register realization (`CONV`,
  pp. 3–6).
- Equivalent state diagrams and trellises expose transitions and output labels.
- Code rate, constraint length and free distance play roles analogous to block
  code rate, length and minimum distance (`CONV`, pp. 7–9).
- A catastrophic encoder can turn a finite-weight coded error into an
  infinite-weight information error and must be avoided.
- Maximum-likelihood sequence decoding becomes a shortest-path problem on the
  trellis; the Viterbi algorithm keeps the best survivor for every state
  (`CONV`, pp. 10–11).
- Termination, tail biting and truncation trade rate, delay and boundary effects.

## Internal connections

- [Block codes, Hamming distance, and bounds](Block%20codes%2C%20Hamming%20distance%2C%20and%20bounds.md)
- [Code selection and parameter design](Code%20selection%20and%20parameter%20design.md)
