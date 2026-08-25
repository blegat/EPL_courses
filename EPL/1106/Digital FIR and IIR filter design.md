# Digital FIR and IIR filter design

## Topics and results

- A finite-impulse-response filter has a polynomial transfer function
  $H(z)=\sum_{k=0}^Mb_kz^{-k}$ and is BIBO stable; symmetry can provide linear
  phase (`FILTER`, pp. 39–45).
- Truncating the ideal impulse response with a window yields a realizable FIR
  approximation. Filter order controls transition width, while the window
  controls ripple and sidelobes.
- Infinite-impulse-response filters use feedback and rational transfer
  functions, often attaining sharper selectivity with lower order
  (`FILTER`, pp. 46–49).
- IIR stability requires poles strictly inside the unit disk. Quantization and
  realization structure can move poles and affect numerical robustness.
- Analog-prototype transformations provide one route to IIR design; frequency
  mapping and possible warping must be accounted for.
- FIR and IIR design trade phase, order, delay, stability robustness and
  computational cost rather than admitting one universally superior choice.
