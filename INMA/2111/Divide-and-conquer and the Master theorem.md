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

## Internal connections

- [Sorting, heaps, and comparison lower bounds](Sorting%2C%20heaps%2C%20and%20comparison%20lower%20bounds.md)
- [Discrete Fourier transform and fast multiplication](Discrete%20Fourier%20transform%20and%20fast%20multiplication.md)
- [Fast matrix multiplication, reductions, and selection](Fast%20matrix%20multiplication%2C%20reductions%2C%20and%20selection.md)
- [Dynamic programming](Dynamic%20programming.md)
