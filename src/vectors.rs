use std::f32::consts::{FRAC_PI_2, PI};

use libm::{acosf, cosf, sinf};

/// PI * 2.
const PI_2: f32 = PI + PI;

/// A 3d vector either in polar or cartesian coordinates.
///
/// Vectors are always normalized when created and inputs with zero magnitude
/// are not allowed (for now). Vectors can be converted between the two
/// coordinate systems and they know which coordinate system they are in.
#[derive(Debug, Clone)]
pub struct Vector {
    values: [f32; 3], // [x, y, z] or [_, azimuth, zenith]
    system: System,
}

impl std::fmt::Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.system {
            System::Cartesian => {
                let [x, y, z] = self.values;
                write!(f, "Cartesian: ({x:.6}, {y:.6}, {z:.6})")
            }
            System::Polar => {
                let [_, azimuth, zenith] = self.values;
                write!(f, "Polar: ({azimuth:.6}, {zenith:.6})")
            }
        }
    }
}

impl Vector {
    /// Create a new `Vector` in polar coordinates.
    pub fn new_polar(azimuth: f32, zenith: f32) -> Self {
        let azimuth = azimuth % PI_2;

        let zenith = zenith % PI_2;
        let zenith = if zenith < PI { zenith } else { PI_2 - zenith };

        Self::new_polar_unchecked(azimuth, zenith)
    }

    fn new_polar_unchecked(azimuth: f32, zenith: f32) -> Self {
        Self {
            values: [1., azimuth, zenith],
            system: System::Polar,
        }
    }

    /// Create a new `Vector` in cartesian coordinates.
    ///
    /// # Errors
    ///
    /// * If the magnitude of the given `Vector` would be zero.
    pub fn new_cartesian(x: f32, y: f32, z: f32) -> Result<Self, String> {
        let r_sq = x * x + y * y + z * z;
        if r_sq == 0. {
            // TODO: Deal with zero vectors and remove the Result return type
            Err("Cannot have vector with zero magnitude".to_string())
        } else if r_sq == 1. {
            Ok(Self::new_cartesian_unchecked(x, y, z))
        } else {
            let r = r_sq.sqrt();
            Ok(Self::new_cartesian_unchecked(x / r, y / r, z / r))
        }
    }

    fn new_cartesian_unchecked(x: f32, y: f32, z: f32) -> Self {
        Self {
            values: [x, y, z],
            system: System::Cartesian,
        }
    }

    /// Returns a new `Vector` in polar coordinates.
    pub fn as_polar(&self) -> Self {
        self.clone().to_polar()
    }

    /// Convert the `Vector` to polar coordinates.
    pub fn to_polar(self) -> Self {
        match self.system {
            System::Cartesian => {
                let [x, y, z] = self.values;

                let xy_sq = x * x + y * y;
                let zenith = if xy_sq != 0. {
                    acosf(x / xy_sq.sqrt())
                } else {
                    FRAC_PI_2
                };

                let azimuth = if y < 0. { PI_2 - acosf(z) } else { acosf(z) };

                Self::new_polar_unchecked(azimuth, zenith)
            }
            System::Polar => self,
        }
    }

    /// Returns a new `Vector` in cartesian coordinates.
    pub fn as_cartesian(&self) -> Self {
        self.clone().to_cartesian()
    }

    /// Convert the `Vector` to cartesian coordinates.
    pub fn to_cartesian(self) -> Self {
        match self.system {
            System::Cartesian => self,
            System::Polar => {
                let [_, azimuth, zenith] = self.values;
                let [sin_azimuth, sin_zenith] = [sinf(azimuth), sinf(zenith)];
                let [cos_azimuth, cos_zenith] = [cosf(azimuth), cosf(zenith)];

                Self::new_cartesian_unchecked(
                    sin_azimuth * cos_zenith,
                    sin_azimuth * sin_zenith,
                    cos_azimuth,
                )
            }
        }
    }

    /// Computes the angular distance between two `Vector`s.
    ///
    /// Converts the two vectors to cartesian unit vectors and computes their
    /// dot-product, which is equal to the cosine of the angle between them.
    /// Returns the arccos of the dot-product.
    pub fn angular_distance(&self, other: &Self) -> f32 {
        let [x1, y1, z1] = self.as_cartesian().values;
        let [x2, y2, z2] = other.as_cartesian().values;
        let p = crate::utils::clip(x1 * x2 + y1 * y2 + z1 * z2);
        cosf(p).abs()
    }

    pub fn mean(vectors: &[Self]) -> Result<Self, String> {
        let [x, y, z] = vectors
            .iter()
            .map(|v| v.as_cartesian().values)
            .fold([0., 0., 0.], |[x, y, z], [x_, y_, z_]| {
                [x + x_, y + y_, z + z_]
            });
        let n = vectors.len() as f32;
        Self::new_cartesian(x / n, y / n, z / n)
    }
}

#[derive(Debug, Clone)]
enum System {
    Cartesian,
    Polar,
}
