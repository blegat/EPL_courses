# Outliers, leverage, and influential observations

An observation may have an unusual response, unusual predictors (high leverage), or a large effect on the fitted model (influence). Studentized residuals detect response anomalies; diagonal hat values measure leverage; Cook's distance and DFBETAs quantify changes under deletion.

Diagnostics should trigger investigation rather than automatic deletion. Data errors, model misspecification, genuinely rare cases, and heavy-tailed noise call for different responses, including transformations or robust estimation.

Source: `LM`, pp. 303–330.

## Related courses

- [OLS estimation and projection geometry](OLS%20estimation%20and%20projection%20geometry.md) defines the hat matrix behind leverage.
- [Outlier and anomaly detection](../../INFO/2364/Outlier%20and%20anomaly%20detection.md) surveys broader statistical, proximity, and learning-based approaches.
