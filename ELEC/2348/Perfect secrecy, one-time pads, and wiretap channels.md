# Perfect secrecy, one-time pads, and wiretap channels

## Topics and results

- Perfect secrecy requires the ciphertext to reveal no information about the
  message: $I(M;C)=0$, equivalently $P_{M\mid C}=P_M$ (`CRYPTO`, pp. 3–8).
- With a uniform independent key as long as the message, the one-time pad
  $C=M\oplus K$ achieves perfect secrecy and exact decryption (`CRYPTO`,
  pp. 9–12).
- Shannon's bound shows that a perfectly secret, error-free encryption scheme
  needs key entropy at least the message entropy; a one-time key cannot safely
  be reused (`CRYPTO`, pp. 13–15).
- A wiretap channel gives the legitimate receiver a statistical advantage over
  the eavesdropper. Channel coding and privacy amplification can convert that
  advantage into secrecy (`CRYPTO`, pp. 16–20).
- Secrecy capacity measures the maximum reliable rate with vanishing leakage;
  its positivity depends on the relation between the two channels.

## Related courses

- Entropy foundation: [LDACS1110 — Shannon entropy and conditional entropy](../../DACS/1110/Shannon%20entropy%20and%20conditional%20entropy.md)

## Internal connections

- [Entropy, conditional entropy, and mutual information](Entropy%2C%20conditional%20entropy%2C%20and%20mutual%20information.md)
- [Discrete memoryless channels and capacity](Discrete%20memoryless%20channels%20and%20capacity.md)
- [Secret-key agreement and information-theoretic authentication](Secret-key%20agreement%20and%20information-theoretic%20authentication.md)
