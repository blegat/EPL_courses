# Matching pursuits and adaptive dictionaries

## Topics and results

- A redundant time-frequency dictionary contains localized atoms at multiple
  positions, scales, and frequencies, avoiding the rigidity of any single
  Fourier or wavelet basis (`MP`, pp. 2–7).
- Matching pursuit greedily selects the normalized atom with largest residual
  correlation, subtracts its contribution, and repeats. The residual energy
  decreases monotonically and the ordered expansion is progressive (`MP`,
  pp. 8–11).
- Coherent atoms can extract structured patterns from noise; applications include
  denoising and face authentication (`MP`, pp. 12–21).
- Image coding transmits selected atom parameters and coefficients. Rate–distortion
  optimization balances reconstruction error against the bit rate needed to
  describe and quantize those atoms (`MP`, pp. 22–35).
- Exhaustive correlation over a large dictionary is expensive. Fourier-domain
  convolution, subband organization, hierarchical approximation, and dictionary
  restrictions reduce the search cost (`MP`, pp. 36–43).
- Dictionaries can incorporate geometric transformations and domain structure;
  this flexibility improves description but increases selection complexity
  (`MP`, pp. 44–51).

## Related courses

- Recovery framework: [Sparse representations and inverse problems](Sparse%20representations%20and%20inverse%20problems.md)
- Fixed multiscale bases: [Multiscale pyramids and wavelets](Multiscale%20pyramids%20and%20wavelets.md)
