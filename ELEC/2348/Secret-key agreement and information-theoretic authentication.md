# Secret-key agreement and information-theoretic authentication

## Topics and results

- Parties observing correlated randomness can communicate publicly to reconcile
  their observations and distill a shared secret (`CRYPTO`, pp. 21–24).
- Public discussion helps agreement but is also observed by the adversary;
  privacy amplification compresses partially secret data to reduce leakage.
- Interaction can enable key agreement in settings where a one-way protocol is
  insufficient.
- An information-theoretic message-authentication code uses a shared secret key
  so that substitution or impersonation succeeds only with bounded probability
  (`CRYPTO`, pp. 25–28).
- Strongly universal hash families make tag pairs uniformly distributed and
  provide explicit forgery bounds (`CRYPTO`, pp. 29–31).
- Authentication does not itself conceal the message, and repeated-key use must
  be included in the security accounting.

## Internal connections

- [Entropy, conditional entropy, and mutual information](Entropy%2C%20conditional%20entropy%2C%20and%20mutual%20information.md)
- [Perfect secrecy, one-time pads, and wiretap channels](Perfect%20secrecy%2C%20one-time%20pads%2C%20and%20wiretap%20channels.md)
- [Secret sharing and secure function evaluation](Secret%20sharing%20and%20secure%20function%20evaluation.md)
