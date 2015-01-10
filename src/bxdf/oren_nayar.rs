//! Defines an Oren-Nayar BRDF that describes diffuse reflection from rough surfaces
//! See [Oren-Nayar reflectance model](https://en.wikipedia.org/wiki/Oren%E2%80%93Nayar_reflectance_model)

use std::f32;
use std::num::Float;
use collect::enum_set::EnumSet;

use linalg::Vector;
use film::Colorf;
use bxdf;
use bxdf::{BxDF, BxDFType};

/// Oren-Nayar BRDf that implements the Oren-Nayar reflectance model
#[derive(Copy, Show)]
pub struct OrenNayar {
    /// Color of the diffuse material
    reflectance: Colorf,
    /// Precomputed and stored value of the A constant
    a: f32,
    /// Precomputed and stored value of the B constant
    b: f32,
}

impl OrenNayar {
    /// Create a new Oren-Nayar BRDF with the desired color and roughness
    /// `roughness` should be the variance of the Gaussian describing the
    /// microfacet distribution
    pub fn new(c: &Colorf, roughness: f32) -> OrenNayar {
        OrenNayar { reflectance: *c,
                    a: 1.0 - roughness / (2.0 * (roughness + 0.33)),
                    b: 0.45 * roughness / (roughness + 0.09),
        }
    }
}

impl BxDF for OrenNayar {
    fn bxdf_type(&self) -> EnumSet<BxDFType> {
        let mut e = EnumSet::new();
        e.insert(BxDFType::Diffuse);
        e.insert(BxDFType::Reflection);
        e
    }
    fn eval(&self, w_o: &Vector, w_i: &Vector) -> Colorf {
        let sin_theta_o = bxdf::sin_theta(w_o);
        let sin_theta_i = bxdf::sin_theta(w_i);
        let max_cos =
            if sin_theta_i > 1e-4 && sin_theta_o > 1e-4 {
                Float::max(0.0, bxdf::cos_phi(w_i) * bxdf::cos_phi(w_o)
                           + bxdf::sin_phi(w_i) * bxdf::sin_phi(w_o))
            } else {
                0.0
            };
        let (sin_alpha, tan_beta) =
            if Float::abs(bxdf::cos_theta(w_i)) > Float::abs(bxdf::cos_theta(w_o)) {
                (sin_theta_o, sin_theta_i / Float::abs(bxdf::cos_theta(w_i)))
            } else {
                (sin_theta_i, sin_theta_o / Float::abs(bxdf::cos_theta(w_o)))
            };
        self.reflectance * f32::consts::FRAC_1_PI * (self.a + self.b * max_cos * sin_alpha * tan_beta)
    }
}
