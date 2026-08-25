# Sorting, heaps, and comparison lower bounds

## Topics and results

- Selection and insertion sort take $\Theta(n^2)$ time in the worst case;
  insertion sort benefits from nearly sorted inputs (`NOTES`, pp. 6–8).
- Merge sort and randomized quicksort illustrate divide-and-conquer. Merge sort
  is worst-case $\Theta(n\log n)$; quicksort's balance controls its recurrence
  (`S1`, pp. 13–15; `NOTES`, pp. 8–11).
- A binary heap implements priority-queue insertion and extremum extraction in
  $O(\log n)$, yielding heapsort in $O(n\log n)$ (`S2`, pp. 10–12;
  `NOTES`, pp. 12–15).
- A comparison sorting algorithm induces a decision tree with at least $n!$
  leaves, hence worst-case depth $\Omega(\log(n!))=\Omega(n\log n)$ (`S2`,
  pp. 2–5).
- Counting sort escapes the lower bound by using more than comparisons and runs
  in $O(n+k)$ for keys in a range of size $k$ (`S2`, pp. 13–16).
