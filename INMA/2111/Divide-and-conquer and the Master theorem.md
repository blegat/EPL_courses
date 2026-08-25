# Divide-and-conquer and the Master theorem

## Topics and results

- Divide-and-conquer splits an instance, recursively solves subinstances and
  combines their answers; its cost is described by a recurrence.
- The Master theorem classifies $T(n)=aT(n/b)+f(n)$ by comparing $f(n)$ with
  $n^{\log_ba}$, subject to its regularity assumptions (`S3`, p. 5).
- Schoolbook multiplication is quadratic. Karatsuba replaces four half-size
  products by three and obtains $T(n)=3T(n/2)+O(n)=\Theta(n^{\log_2 3})$
  (`S3`, pp. 2–6).
- Toom–Cook generalizes evaluation and interpolation at more points, trading
  asymptotic exponent against overhead (`S3`, p. 6).
- Recursion trees help derive bounds and reveal whether leaves, internal work,
  or all levels dominate.
