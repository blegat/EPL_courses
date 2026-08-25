# Rate-distortion, transform, and predictive coding

## Topics and results

- Lossy source coding trades bitrate against an allowed distortion measure.
  The rate-distortion function is the least mutual information compatible with
  expected distortion at most $D$.
- Coding gain compares the distortion of a structured coder with a baseline at
  the same rate (`SOURCE`, pp. 44–45).
- Transform coding decorrelates a vector, allocates bits across coefficients and
  quantizes them separately. Energy compaction makes unequal allocation useful.
- Predictive coding transmits a quantized innovation rather than the raw sample;
  the decoder must reproduce the same predictor state (`SOURCE`, pp. 46–47).
- Hybrid codecs combine prediction, transforms, quantization and entropy coding
  (`SOURCE`, pp. 47–48).
- Operational performance includes finite block size, side information, delay
  and model mismatch in addition to the asymptotic rate-distortion limit.

## Internal connections

- [Entropy, conditional entropy, and mutual information](Entropy%2C%20conditional%20entropy%2C%20and%20mutual%20information.md)
- [Scalar quantization and quantization noise](Scalar%20quantization%20and%20quantization%20noise.md)
