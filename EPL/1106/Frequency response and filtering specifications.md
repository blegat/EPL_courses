# Frequency response and filtering specifications

## Topics and results

- For an LTI system, a complex exponential input produces the same exponential
  multiplied by the frequency response $H(j\omega)$ or $H(e^{j\omega})$.
- Magnitude controls attenuation/amplification and phase controls delay or
  waveform alignment. Bode plots use logarithmic magnitude and frequency.
- Ideal low-pass, high-pass and band-pass filters specify brick-wall frequency
  responses, but ideal selectivity requires noncausal or infinitely long
  impulse responses (`FILTER`, pp. 4–12).
- Practical specifications use passband/stopband edges, allowed ripple and
  minimum attenuation, leaving a nonzero transition band.
- Filtering is multiplication $Y=HX$ in frequency and convolution $y=h*x$ in
  time; transient behavior and implementation cost remain relevant even when
  the frequency response looks satisfactory.

## Internal connections

- [Continuous and discrete Fourier transforms](Continuous%20and%20discrete%20Fourier%20transforms.md)
- [Analog Butterworth and Chebyshev filter design](Analog%20Butterworth%20and%20Chebyshev%20filter%20design.md)
- [Digital FIR and IIR filter design](Digital%20FIR%20and%20IIR%20filter%20design.md)
