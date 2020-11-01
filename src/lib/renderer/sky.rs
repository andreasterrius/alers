// Approximation of sun and sky based on https://github.com/ebruneton/precomputed_atmospheric_scattering
// Original paper by: Eric Bruneton, Fabrice Neyret. Precomputed Atmospheric Scattering. Computer Graphics Forum, Wiley, 2008,
// https://hal.inria.fr/inria-00288758/en

use cgmath::Vector3;

struct AtmosphereParameters {
  // Solar irradiance at the top of the atmosphere
  solar_irradiance: Vector3<f64>,
  // Sun angular radius
  sun_angular_radius: f64,
  // Distance between planet center at bottom of atmosphere
  bottom_radius: f64,
  // Distance between planet center at top of atmosphere
  top_radius: f64,
  // density profile of air molecules
  rayleigh_density: DensityProfile,
  rayleigh_scattering: Vector3<f64>,

  mie_density: DensityProfile,
  mie_scattering: Vector3<f64>,
  mie_extinction: Vector3<f64>,
  mie_phase_function_g: f64,

  absorption_density: DensityProfile,
  absorption_extinction: Vector3<f64>,
  ground_albedo: f64,

  mu_s_min: f64,
}

struct BrunetonSky {}

impl BrunetonSky {
  pub fn new() -> BrunetonSky {
    // Settings ?
    let mut use_half_precision = true;
    let constant_solar_spectrum = true;
    let use_ozone = true;

    // Constants
    let k_pi = 3.1415926;
    let k_sun_angular_radius = 0.00935 / 2.0;
    let k_sun_solid_angle = k_pi * k_sun_angular_radius * k_sun_angular_radius;
    let k_length_unit_in_meters = 1000.0;

    let k_lambda_min = 360;
    let k_lambda_max = 830;
    let k_solar_irradiance = vec![
      1.11776, 1.14259, 1.01249, 1.14716, 1.72765, 1.73054, 1.6887, 1.61253, 1.91198, 2.03474, 2.02042, 2.02212,
      1.93377, 1.95809, 1.91686, 1.8298, 1.8685, 1.8931, 1.85149, 1.8504, 1.8341, 1.8345, 1.8147, 1.78158, 1.7533,
      1.6965, 1.68194, 1.64654, 1.6048, 1.52143, 1.55622, 1.5113, 1.474, 1.4482, 1.41018, 1.36775, 1.34188, 1.31429,
      1.28303, 1.26758, 1.2367, 1.2082, 1.18737, 1.14683, 1.12362, 1.1058, 1.07124, 1.04992,
    ];

    let k_ozone_cross_section = vec![
      1.18e-27, 2.182e-28, 2.818e-28, 6.636e-28, 1.527e-27, 2.763e-27, 5.52e-27, 8.451e-27, 1.582e-26, 2.316e-26,
      3.669e-26, 4.924e-26, 7.752e-26, 9.016e-26, 1.48e-25, 1.602e-25, 2.139e-25, 2.755e-25, 3.091e-25, 3.5e-25,
      4.266e-25, 4.672e-25, 4.398e-25, 4.701e-25, 5.019e-25, 4.305e-25, 3.74e-25, 3.215e-25, 2.662e-25, 2.238e-25,
      1.852e-25, 1.473e-25, 1.209e-25, 9.423e-26, 7.455e-26, 6.566e-26, 5.105e-26, 4.15e-26, 4.228e-26, 3.237e-26,
      2.451e-26, 2.801e-26, 2.534e-26, 1.624e-26, 1.465e-26, 2.078e-26, 1.383e-26, 7.105e-27,
    ];

    let k_dobson_unit = 2.687e20;
    let k_max_ozone_number_density = 300.0 * k_dobson_unit / 15000.0;
    let k_constant_solar_irradiance = 1.5;
    let k_bottom_radius = 6360000.0;
    let k_top_radius = 6420000.0;
    let k_rayleigh = 1.24062e-6;
    let k_rayleigh_scale_height = 1200.0;
    let k_mie_scale_height = 1200.0;
    let k_mie_angstrom_alpha = 0.0;
    let k_mie_angstrom_beta = 5.328e-3;
    let k_mie_single_scattering_albedo = 0.0;
    let k_mie_phase_function_g = 0.8;
    let k_ground_albedo = 0.1;

    let max_sun_zenith_angle = if use_half_precision { 102.0 } else { 180.0 } / 180.0 * k_pi;
    let rayleigh_layer = DensityProfileLayer {
      width: 0.0,
      exp_term: 1.0,
      exp_scale: -1.0 / k_rayleigh_scale_height,
      linear_term: 0.0,
      constant_term: 0.0,
    };

    let mie_layer = DensityProfileLayer {
      width: 0.0,
      exp_term: 1.0,
      exp_scale: -1.0 / k_mie_scale_height,
      linear_term: 0.0,
      constant_term: 0.0,
    };

    let mut ozone_density = vec![];
    ozone_density.push(DensityProfileLayer {
      width: 25000.0,
      exp_term: 0.0,
      exp_scale: 0.0,
      linear_term: 1.0 / 15000.0,
      constant_term: -2.0 / 3.0,
    });

    ozone_density.push(DensityProfileLayer {
      width: 0.0,
      exp_term: 0.0,
      exp_scale: 0.0,
      linear_term: -1.0 / 15000.0,
      constant_term: 8.0 / 3.0,
    });

    let mut wavelengths = vec![];
    let mut solar_irradiance = vec![];
    let mut rayleigh_scattering = vec![];
    let mut mie_scattering = vec![];
    let mut mie_extinction = vec![];
    let mut absorption_extinction = vec![];
    let mut ground_albedo = vec![];

    for l in (k_lambda_min..=k_lambda_max).step_by(10) {
      let lambda = l as f64 * 1e-3;
      let mie = k_mie_angstrom_beta / k_mie_scale_height * f64::powf(lambda, -k_mie_angstrom_alpha);
      wavelengths.push(l);

      if constant_solar_spectrum {
        solar_irradiance.push(k_constant_solar_irradiance);
      } else {
        solar_irradiance.push(k_solar_irradiance[(l - k_lambda_min) / 10]);
      }
      rayleigh_scattering.push(k_rayleigh * f64::powf(lambda, -4.0));
      mie_scattering.push(mie * k_mie_single_scattering_albedo);
      mie_extinction.push(mie);
      absorption_extinction.push(if use_ozone {
        k_max_ozone_number_density * k_ozone_cross_section[(l - k_lambda_min) / 10]
      } else {
        0.0
      });
      ground_albedo.push(k_ground_albedo);
    }

    return BrunetonSky {};
  }

  pub fn create_texture() {}
}

struct DensityProfileLayer {
  pub width: f64,
  pub exp_term: f64,
  pub exp_scale: f64,
  pub linear_term: f64,
  pub constant_term: f64,
}

struct DensityProfile {
  layers: [DensityProfileLayer; 2],
}

struct Model {
  wavelengths: Vec<f64>,
  solar_irradiance: Vec<f64>,
  sun_angular_radius: f64,
  bottom_radius: f64,
  top_radius: f64,

  rayleigh_density: Vec<DensityProfileLayer>,
  rayleigh_scattering: Vec<f64>,

  mie_density: Vec<DensityProfileLayer>,
  mie_scattering: Vec<f64>,
  mie_extinction: Vec<f64>,
  mie_phase_function_g: f64,

  absorption_density: DensityProfileLayer,
  absorption_extinction: Vec<f64>,
  ground_albedo: f64,

  max_sun_zenith_angle: f64,
  length_unit_in_meters: f64,

  num_precomputed_wavelengths: u32,
  combine_scattering_textures: bool,
  half_precision: bool,
}

fn clamp(val: f64, min: f64, max: f64) -> f64 {
  if val < min {
    min
  } else if val > max {
    max
  } else {
    val
  }
}

fn clamp_cosine(mu: f64) -> f64 {
  clamp(mu, -1.0, 1.0)
}

fn clamp_distance(d: f64) -> f64 {
  return f64::max(d, 0.0);
}

fn clamp_radius(atmosphere: &AtmosphereParameters, r: f64) -> f64 {
  clamp(r, atmosphere.bottom_radius, atmosphere.top_radius)
}

fn safe_sqrt(a: f64) -> f64 {
  f64::sqrt(f64::max(a, 0.0))
}

fn exp(a: Vector3<f64>) -> Vector3<f64> {
  return Vector3::new(f64::exp(a.x), f64::exp(a.y), f64::exp(a.z));
}

fn distance_to_top_atmosphere_boundary(atmosphere: &AtmosphereParameters, r: f64, mu: f64) -> f64 {
  assert!(r <= atmosphere.top_radius);
  assert!(mu >= -1.0 && mu <= 1.0);
  let discriminant = r * r * (mu * mu - 1.0) + atmosphere.top_radius * atmosphere.top_radius;
  return clamp_distance(-r * mu + safe_sqrt(discriminant));
}

fn distance_to_bottom_atmosphere_boundary(atmosphere: &AtmosphereParameters, r: f64, mu: f64) -> f64 {
  assert!(r >= atmosphere.bottom_radius);
  assert!(mu >= -1.0 && mu <= 1.0);
  let discriminant = r * r * (mu * mu - 1.0) + atmosphere.bottom_radius * atmosphere.bottom_radius;
  return clamp_distance(-r * mu - safe_sqrt(discriminant));
}

fn ray_intersect_ground(atmosphere: &AtmosphereParameters, r: f64, mu: f64) -> bool {
  assert!(r >= atmosphere.bottom_radius);
  assert!(mu >= -1.0 && mu <= 1.0);
  return mu < 0.0 && r * r * (mu * mu - 1.0) + atmosphere.bottom_radius * atmosphere.bottom_radius >= 0.0;
}

fn get_layer_density(layer: &DensityProfileLayer, altitude: f64) -> f64 {
  let density =
    layer.exp_term * f64::exp(layer.exp_scale * altitude) + layer.linear_term * altitude * layer.constant_term;
  clamp(density, 0.0, 1.0)
}

fn get_profile_density(profile: &DensityProfile, altitude: f64) -> f64 {
  if altitude < profile.layers[0].width {
    get_layer_density(&profile.layers[0], altitude)
  } else {
    get_layer_density(&profile.layers[1], altitude)
  }
}

fn compute_optical_length_to_top_atmosphere_boundary(
  atmosphere: &AtmosphereParameters,
  profile: &DensityProfile,
  r: f64,
  mu: f64,
) -> f64 {
  assert!(r >= atmosphere.bottom_radius && r <= atmosphere.top_radius);
  assert!(mu >= -1.0 && mu <= 1.0);

  const SAMPLE_COUNT: i32 = 500;
  let dx = distance_to_top_atmosphere_boundary(&atmosphere, r, mu) / SAMPLE_COUNT as f64;

  let mut result = 0.0;
  for i in 0..SAMPLE_COUNT {
    let d_i = (i as f64 * dx);

    // distance between current sample point and planet center
    let r_i = f64::sqrt(d_i * d_i + 2.0 * r * mu * d_i + r * r);

    // number of density at current sample point
    // divided by density at bot of atmosphere
    let y_i = get_profile_density(profile, r_i - atmosphere.bottom_radius);

    // sample weight (from trapezoidal rule)
    let weight_i = if i == 0 || i == SAMPLE_COUNT { 0.5 } else { 1.0 };

    result += y_i * weight_i * dx;
  }
  return result;
}

fn compute_transmittance_to_top_atmosphere_boundary(
  atmosphere: &AtmosphereParameters,
  r: f64,
  mu: f64,
) -> Vector3<f64> {
  assert!(r >= atmosphere.bottom_radius && r <= atmosphere.top_radius);
  assert!(mu >= -1.0 && mu <= 1.0);
  return exp(
    -(atmosphere.rayleigh_scattering
      * compute_optical_length_to_top_atmosphere_boundary(atmosphere, &atmosphere.rayleigh_density, r, mu)
      + atmosphere.mie_extinction
        * compute_optical_length_to_top_atmosphere_boundary(atmosphere, &atmosphere.mie_density, r, mu)
      + atmosphere.absorption_extinction
        * compute_optical_length_to_top_atmosphere_boundary(atmosphere, &atmosphere.absorption_density, r, mu)),
  );
}

//x [0,1] to u [0.5/n, 1-0.5/n]
fn get_texture_coord_from_unit_range(x: f64, texture_size: i64) -> f64 {
  return 0.5 / texture_size as f64 + x * (1.0 - 1.0 / texture_size as f64);
}

fn get_unit_range_from_texture_coord(u: f64, texture_size: i64) -> f64 {
  return (u - 0.5 / texture_size as f64) / (1.0 - 1.0 / texture_size as f64);
}

#[test]
fn test_get_texture_coord_from_unit_range() {
  let texture_size = 256;
  let r = 10.0;
  let mu = 0.5;
  println!("{}", get_texture_coord_from_unit_range(mu, texture_size));
  println!("{}", get_texture_coord_from_unit_range(0.1, texture_size));
  println!("{}", get_texture_coord_from_unit_range(0.2, texture_size));
  println!("{}", get_texture_coord_from_unit_range(0.3, texture_size));
}
