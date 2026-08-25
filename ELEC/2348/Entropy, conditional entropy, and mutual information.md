# Entropy, conditional entropy, and mutual information

## Topics and results

- The self-information of an event of probability $p$ is $-\log_2p$; the
  entropy of a discrete source is
  $$H(X)=-\sum_xp(x)\log_2p(x).$$
  It is nonnegative and at most $\log_2|\mathcal X|$, with equality for the
  uniform law (`SOURCE`, pp. 13–17).
- Joint and conditional entropy obey the chain rule
  $H(X,Y)=H(X)+H(Y\mid X)$.
- Conditioning cannot increase entropy: $H(X\mid Y)\leq H(X)$.
- Mutual information
  $$I(X;Y)=H(X)-H(X\mid Y)=D(P_{XY}\|P_XP_Y)$$
  is symmetric, nonnegative and zero exactly for independence (`SOURCE`,
  pp. 18–25).
- Data processing formalizes that post-processing cannot create information
  about an upstream variable.

## Related courses

- Foundations treatment: [LDACS1110 — Shannon entropy and conditional entropy](../../DACS/1110/Shannon%20entropy%20and%20conditional%20entropy.md)
- Divergence viewpoint: [LDACS1110 — KL divergence, cross-entropy, and mutual information](../../DACS/1110/KL%20divergence,%20cross-entropy,%20and%20mutual%20information.md)

## Internal connections

- [Prefix codes, Kraft inequality, and Huffman coding](Prefix%20codes%2C%20Kraft%20inequality%2C%20and%20Huffman%20coding.md)
- [Rate-distortion, transform, and predictive coding](Rate-distortion%2C%20transform%2C%20and%20predictive%20coding.md)
- [Discrete memoryless channels and capacity](Discrete%20memoryless%20channels%20and%20capacity.md)
- [Perfect secrecy, one-time pads, and wiretap channels](Perfect%20secrecy%2C%20one-time%20pads%2C%20and%20wiretap%20channels.md)
- [Secret-key agreement and information-theoretic authentication](Secret-key%20agreement%20and%20information-theoretic%20authentication.md)
