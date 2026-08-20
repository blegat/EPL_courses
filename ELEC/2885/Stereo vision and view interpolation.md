# Stereo vision and view interpolation

## Topics and results

- Calibrated stereo recovers depth by triangulating corresponding image points
  observed from two viewpoints (`GEO`, pp. 36–43).
- Epipolar geometry constrains a point's match to an epipolar line, reducing a
  two-dimensional search to one dimension; rectification makes those lines
  horizontal (`GEO`, pp. 44–49).
- Dense correspondence needs a discriminative matching cost and must handle
  textureless regions, repetitive patterns, radiometric changes, and occlusions
  (`GEO`, pp. 48–57).
- Ordering, disparity smoothness, and piecewise-planar scene models regularize
  ambiguous local evidence while preserving meaningful discontinuities (`GEO`,
  pp. 58–65).
- View interpolation combines disparity/depth with image warping to synthesize a
  virtual viewpoint; visibility and occlusion determine which source samples can
  contribute (`GEO`, pp. 66–81).

## Related courses

- Calibration and robust correspondences: [Camera calibration, homographies, and image stitching](Camera%20calibration,%20homographies,%20and%20image%20stitching.md)
- Motion correspondences: [Recursive appearance-based tracking](Recursive%20appearance-based%20tracking.md)
