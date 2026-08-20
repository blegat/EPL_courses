# Detection-based multi-object tracking

## Topics and results

- Detect-to-track starts from per-frame object detections, making recovery after
  occlusion easier than purely recursive appearance tracking at the cost of
  detector latency and association ambiguity (`D2T`, pp. 2–7).
- A tracking graph represents detections as vertices and feasible temporal
  associations as weighted edges; costs combine appearance, motion, and prior
  compatibility (`D2T`, pp. 8–14).
- Shortest-path inference extracts a likely trajectory efficiently, while
  K-shortest paths retain alternative hypotheses (`D2T`, pp. 8–18).
- Iterative hypothesis testing validates a proposed track, removes or contracts
  accepted evidence, and searches again, but requires careful treatment of noisy
  or sporadic detections (`D2T`, pp. 15–19).
- Label propagation spreads identity evidence through a graph and can combine
  positive/negative constraints, multiple feature types, and exclusivity in
  multi-object settings (`D2T`, pp. 19–24).

## Related courses

- Detector foundation: [Convolutional neural networks for vision](Convolutional%20neural%20networks%20for%20vision.md)
- Low-latency alternative: [Recursive appearance-based tracking](Recursive%20appearance-based%20tracking.md)
