#[derive(Debug, Clone)]
pub struct Vector {
    values: [f32; 3], // [x, y, z] or [magnitude, azimuth, zenith]
    system: System,
}

impl Vector {
    pub fn new_polar(azimuth: f32, zenith: f32) -> Self {
        let azimuth = azimuth % (2. * std::f32::consts::PI);

        let zenith = zenith % (2. * std::f32::consts::PI);
        let zenith = if zenith >= std::f32::consts::PI {
            zenith - std::f32::consts::PI
        } else {
            zenith
        };

        Self {
            values: [1., azimuth, zenith],
            system: System::Polar,
        }
    }

    pub fn new_cartesian(x: f32, y: f32, z: f32) -> Result<Self, String> {
        let r_sq = x * x + y * y + z * z;
        if r_sq == 0. {
            Err("Cannot have vector with zero magnitude".to_string())
        } else {
            let r = r_sq.sqrt();
            Ok(Self {
                values: [x / r, y / r, z / r],
                system: System::Cartesian,
            })
        }
    }

    pub fn system(&self) -> System {
        self.system
    }

    pub fn to_polar(mut self) -> Self {
        match self.system {
            System::Cartesian => {
                let [x, y, z] = self.values;

                let xy_sq = x * x + y * y;
                let zenith = if xy_sq != 0. {
                    libm::acosf(x / xy_sq.sqrt())
                } else {
                    std::f32::consts::FRAC_PI_2
                };

                let azimuth = libm::acosf(z);

                self.values = [1., azimuth, zenith];
                self.system = System::Cartesian;
                self
            }
            System::Polar => self,
        }
    }

    pub fn to_cartesian(mut self) -> Self {
        match self.system {
            System::Cartesian => self,
            System::Polar => {
                let [_, a, z] = self.values;
                let [sa, sz] = [libm::sinf(a), libm::sinf(z)];
                let [ca, cz] = [libm::cosf(a), libm::cosf(z)];
                
                self.values = [sa * cz, sa * sz, ca];
                self.system = System::Polar;
                self
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum System {
    Cartesian,
    Polar,
}
