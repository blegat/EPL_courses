# Fast matrix multiplication, reductions, and selection

## Topics and results

- Block matrix multiplication gives the usual cubic recurrence. Strassen uses
  seven rather than eight recursive products and runs in
  $O(n^{\log_2 7})$ (`S4`, pp. 10–14).
- Reductions compare problems: fast multiplication yields fast inversion, and
  multiplication can in turn be encoded using inversion, up to polynomial
  overhead (`S4`, pp. 15–20).
- Quickselect partitions around a random pivot and has linear expected time,
  although an unlucky pivot sequence is quadratic (`NOTES`, pp. 28–29).
- Median-of-medians chooses a pivot guaranteeing constant-fraction progress and
  obtains deterministic worst-case $O(n)$ selection (`NOTES`, pp. 29–31).
- Binary search and the Towers of Hanoi provide contrasting divide-and-conquer
  recurrences (`S4`, pp. 22–25).

## Internal connections

- [Sorting, heaps, and comparison lower bounds](Sorting%2C%20heaps%2C%20and%20comparison%20lower%20bounds.md)
- [Divide-and-conquer and the Master theorem](Divide-and-conquer%20and%20the%20Master%20theorem.md)
