# Code selection and parameter design

## Topics and results

- Code design begins from payload size, channel model, target error rate,
  latency and computational budget rather than from a code family alone
  (`DESIGN`, pp. 2–3).
- Rate, minimum distance and block length allow preliminary comparison of block
  codes; channel capacity rules out infeasible asymptotic operating points.
- Reed–Solomon parameters translate a required number of correctable symbol
  errors or erasures into redundancy (`DESIGN`, pp. 4–6).
- Concatenation and interleaving address mixed random and burst errors.
- Video streaming illustrates unequal error protection and application-level
  consequences of residual losses (`DESIGN`, p. 7).
- Adaptive modulation and coding changes rate and robustness with channel
  quality, requiring estimation and feedback (`DESIGN`, p. 8).
- Finite-length performance and decoder complexity remain essential even below
  Shannon capacity (`DESIGN`, p. 9).

## Internal connections

- [Finite fields and Reed-Solomon codes](Finite%20fields%20and%20Reed-Solomon%20codes.md)
- [Convolutional codes and trellis decoding](Convolutional%20codes%20and%20trellis%20decoding.md)
- [LDPC codes and message-passing decoding](LDPC%20codes%20and%20message-passing%20decoding.md)
