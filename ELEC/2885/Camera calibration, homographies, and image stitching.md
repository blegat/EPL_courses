# Camera calibration, homographies, and image stitching

## Topics and results

- Camera calibration estimates the mapping from three-dimensional world points
  to two-dimensional image coordinates; the course focuses on linear projective
  models as useful initializations for richer distortion models (`GEO`, pp. 2–7).
- The pinhole model combines intrinsic parameters with camera pose. Homogeneous
  coordinates turn perspective projection into a matrix relation estimable from
  3D–2D correspondences (`GEO`, pp. 5–14).
- Multiple views improve scene coverage and robustness. Image stitching aligns
  overlapping views through a planar projective transformation, or homography
  (`GEO`, pp. 15–21).
- Salient points must be repeatably detected and distinctively described across
  viewpoint and illumination changes before candidate correspondences can be
  formed (`GEO`, pp. 22–29).
- RANSAC repeatedly fits a model from minimal samples, counts consensus inliers,
  and retains the strongest hypothesis, making homography estimation robust to
  mismatches (`GEO`, pp. 30–35).

## Related courses

- Robust fitting prerequisite: [LEPL1109 — linear least squares and k-nearest neighbors](../../EPL/1109/Linear%20least%20squares%20and%20k-nearest%20neighbors.md)
- Multi-view continuation: [Stereo vision and view interpolation](Stereo%20vision%20and%20view%20interpolation.md)
- Learned local and global features: [Convolutional neural networks for vision](Convolutional%20neural%20networks%20for%20vision.md)
