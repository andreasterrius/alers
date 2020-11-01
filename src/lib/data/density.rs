struct DensityProfileLayer {
  width: f64,
  exp_term: f64,
  exp_scale: f64,
  linear_term: f64,
  constant_term: f64,
}

struct DensityProfile {
  layers: [DensityProfileLayer; 2],
}
