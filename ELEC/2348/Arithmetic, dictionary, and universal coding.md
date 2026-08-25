# Arithmetic, dictionary, and universal coding

## Topics and results

- Arithmetic coding represents an entire sequence by a nested interval whose
  width is its model probability, approaching ideal fractional code lengths
  (`SOURCE`, pp. 33–35).
- Finite-precision implementations must renormalize intervals and handle
  underflow without allowing encoder and decoder states to diverge.
- Dictionary methods replace repeated phrases by references rather than
  requiring an explicit probabilistic model.
- Lempel–Ziv parsing grows a dictionary from the observed sequence and is
  universal for broad stationary-source classes (`LEMPEL`, pp. 3–13).
- Universal variable-length integer codes represent unbounded positive
  integers with short descriptions for small values (`SOURCE`, pp. 35–36).
- Compression performance reflects model mismatch, adaptation cost, finite
  block length and implementation constraints, not entropy alone.
