# Variable and model selection

Model selection balances fit and complexity. Adjusted $R^2$, Mallows' $C_p$, AIC, BIC, likelihood criteria, and cross-validation penalize or estimate generalization error differently. Forward, backward, and stepwise searches explore subsets without enumerating every model.

Post-selection inference is delicate because the selected model is random. Prediction, explanation, and causal identification may favor different variables, and hierarchical principles often require retaining lower-order terms beneath interactions.

Source: `LM`, pp. 222–258.

## Related courses

- [Multicollinearity and ridge regression](Multicollinearity%20and%20ridge%20regression.md) gives a continuous shrinkage alternative to subset selection.
- [Dummy variables, interactions, and ANOVA](Dummy%20variables,%20interactions,%20and%20ANOVA.md) supplies structured groups of candidate terms.
- [Classifier performance and model comparison](../../INFO/2262/Classifier%20performance%20and%20model%20comparison.md) discusses held-out and resampling-based comparison.
