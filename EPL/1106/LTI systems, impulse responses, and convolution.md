# LTI systems, impulse responses, and convolution

## Topics and results

- Any discrete signal decomposes as $x[n]=\sum_kx[k]\delta[n-k]$; the Dirac
  impulse gives the analogous continuous representation (`TIME-1`, pp. 13–23).
- Linearity and time invariance imply that an LTI system is characterized by
  its impulse response $h$.
- Its zero-state response is convolution:
  $$y[n]=\sum_{k=-\infty}^{\infty}x[k]h[n-k],\qquad
  y(t)=\int_{-\infty}^{\infty}x(\tau)h(t-\tau)\,d\tau.$$
- Convolution is commutative, associative and distributive, so cascades and
  parallel interconnections can be rearranged (`TIME-2`, pp. 44–60).
- A causal LTI impulse response vanishes for negative time. BIBO stability is
  equivalent to absolute summability or integrability of $h$.
- The impulse response describes zero-state behavior; nonzero initial
  conditions require a state or differential-equation model.

## Internal connections

- [Signals, transformations, and system properties](Signals%2C%20transformations%2C%20and%20system%20properties.md)
- [Differential equations, block diagrams, and state space](Differential%20equations%2C%20block%20diagrams%2C%20and%20state%20space.md)
- [Continuous and discrete Fourier transforms](Continuous%20and%20discrete%20Fourier%20transforms.md)
