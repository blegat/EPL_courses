# Constrained and compressed pattern mining

Constraint-based mining asks only for patterns satisfying conditions on length, content, aggregates, or interestingness. Anti-monotone, monotone, succinct, and convertible constraints support different pruning strategies; constraint programming offers a declarative formulation of the same search.

Closed itemsets preserve exact support information, while maximal itemsets retain only the boundary of the frequent region. These condensed representations can be exponentially smaller than the complete frequent-itemset collection, with different information-loss tradeoffs.

Source: `S4`, pp. 2–204.

## Related courses

- [Frequent itemset mining algorithms](Frequent%20itemset%20mining%20algorithms.md) provides the underlying enumeration mechanisms.
- [Interestingness, association rules, and rare patterns](Interestingness,%20association%20rules,%20and%20rare%20patterns.md) motivates constraints beyond support.
- [Sequential pattern mining](Sequential%20pattern%20mining.md) transfers pruning and projection to ordered data.

