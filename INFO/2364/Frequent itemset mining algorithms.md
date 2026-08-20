# Frequent itemset mining algorithms

For a transaction database, an itemset is frequent when its support exceeds a threshold. The anti-monotonicity of support—the Apriori principle—allows every superset of an infrequent itemset to be pruned.

Apriori alternates candidate generation and database scans. FP-growth compresses transactions into an FP-tree and mines conditional trees without explicit candidate generation. Eclat instead intersects vertical transaction-ID sets. Their relative performance depends on density, support, and the cost of candidate storage versus projection.

Source: `S2`, pp. 2–169.

## Related courses

- [Data mining foundations and preprocessing](Data%20mining%20foundations%20and%20preprocessing.md) supplies the transaction representation and evaluation setting.
- [Interestingness, association rules, and rare patterns](Interestingness,%20association%20rules,%20and%20rare%20patterns.md) turns frequent itemsets into assessed relationships.
- [Constrained and compressed pattern mining](Constrained%20and%20compressed%20pattern%20mining.md) reduces or restricts the pattern collection.

