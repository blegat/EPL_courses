# Discrete memoryless channels and capacity

## Topics and results

- A discrete memoryless channel is specified by transition probabilities
  $P_{Y\mid X}(y\mid x)$; repeated uses factor conditionally (`CAP`, pp. 4–8).
- For an input law $P_X$, mutual information $I(X;Y)$ is the average reduction
  in uncertainty about the input after observing the output.
- Channel capacity is
  $$C=\max_{P_X}I(X;Y),$$
  measured in bits per channel use (`CAP`, pp. 9–14).
- Mutual information is concave in the input distribution. Optimality
  conditions and convex optimization characterize a capacity-achieving law
  (`CAP`, pp. 24–31).
- Symmetry often makes the uniform input capacity-achieving; binary symmetric
  and erasure channels yield closed-form capacities (`CAP`, pp. 32–39).
- Capacity is an operational threshold, not merely the entropy of the output.

## Internal connections

- [Entropy, conditional entropy, and mutual information](Entropy%2C%20conditional%20entropy%2C%20and%20mutual%20information.md)
- [Typical sequences and the noisy-channel coding theorem](Typical%20sequences%20and%20the%20noisy-channel%20coding%20theorem.md)
- [Block codes, Hamming distance, and bounds](Block%20codes%2C%20Hamming%20distance%2C%20and%20bounds.md)
- [Perfect secrecy, one-time pads, and wiretap channels](Perfect%20secrecy%2C%20one-time%20pads%2C%20and%20wiretap%20channels.md)
