# Scalar quantization and quantization noise

## Topics and results

- Scalar quantization partitions the real line into decision cells and maps
  each sample to a reconstruction level (`SOURCE`, pp. 37–39).
- Mean-square distortion is
  $D=E[(X-Q(X))^2]$; overload and granular errors have different origins.
- A uniform high-resolution quantizer of step $\Delta$ has approximate error
  variance $\Delta^2/12$ when the error is locally uniform and weakly dependent
  on the input (`SOURCE`, pp. 39–42).
- More levels reduce distortion but require more bits; nonuniform cells devote
  resolution to high-probability regions.
- Lloyd–Max stationarity conditions set reconstruction levels to conditional
  centroids and decision thresholds midway between adjacent levels.
- Quantization noise is a useful approximation, not universally independent
  white noise (`SOURCE`, pp. 42–43).

## Related courses

- Multidimensional continuation: [LELEC2870 — vector quantization and prototype clustering](../2870/Vector%20quantization%20and%20prototype%20clustering.md)
