//! Typed SI quantities and elementary mechanics for a future computational
//! engineering model. These functions are not rocket sizing or safety models.

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MassKg(f64);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AccelerationMPerS2(f64);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ForceN(f64);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AreaM2(f64);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StressPa(f64);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DensityKgPerM3(f64);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VelocityMPerS(f64);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MassFlowKgPerS(f64);

fn positive(value: f64, name: &str) -> Result<f64, String> {
    if value.is_finite() && value > 0.0 {
        Ok(value)
    } else {
        Err(format!("{name} must be finite and positive"))
    }
}
fn finite(value: f64, name: &str) -> Result<f64, String> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(format!("{name} must be finite"))
    }
}

impl MassKg {
    pub fn new(value: f64) -> Result<Self, String> {
        Ok(Self(positive(value, "mass")?))
    }
    pub fn value(self) -> f64 {
        self.0
    }
}
impl AccelerationMPerS2 {
    pub fn new(value: f64) -> Result<Self, String> {
        Ok(Self(finite(value, "acceleration")?))
    }
    pub fn value(self) -> f64 {
        self.0
    }
}
impl ForceN {
    pub fn new(value: f64) -> Result<Self, String> {
        Ok(Self(finite(value, "force")?))
    }
    pub fn value(self) -> f64 {
        self.0
    }
}
impl AreaM2 {
    pub fn new(value: f64) -> Result<Self, String> {
        Ok(Self(positive(value, "area")?))
    }
    pub fn value(self) -> f64 {
        self.0
    }
}
impl StressPa {
    pub fn value(self) -> f64 {
        self.0
    }
}
impl DensityKgPerM3 {
    pub fn new(value: f64) -> Result<Self, String> {
        Ok(Self(positive(value, "density")?))
    }
    pub fn value(self) -> f64 {
        self.0
    }
}
impl VelocityMPerS {
    pub fn new(value: f64) -> Result<Self, String> {
        Ok(Self(finite(value, "velocity")?))
    }
    pub fn value(self) -> f64 {
        self.0
    }
}
impl MassFlowKgPerS {
    pub fn value(self) -> f64 {
        self.0
    }
}

/// Sum of force for a constant-mass body in an inertial frame: F = ma.
pub fn net_force(mass: MassKg, acceleration: AccelerationMPerS2) -> ForceN {
    ForceN(mass.0 * acceleration.0)
}

/// Average normal stress over a stated area: σ = Fn/A.
pub fn average_normal_stress(normal_force: ForceN, area: AreaM2) -> StressPa {
    StressPa(normal_force.0 / area.0)
}

/// Steady one-dimensional mass flow: ṁ = ρ A vn, for uniform section values.
pub fn steady_mass_flow(
    density: DensityKgPerM3,
    area: AreaM2,
    normal_velocity: VelocityMPerS,
) -> MassFlowKgPerS {
    MassFlowKgPerS(density.0 * area.0 * normal_velocity.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn si_equations_match_worked_values() {
        let force = net_force(
            MassKg::new(2.0).unwrap(),
            AccelerationMPerS2::new(3.0).unwrap(),
        );
        assert_eq!(force.value(), 6.0);
        assert_eq!(
            average_normal_stress(force, AreaM2::new(0.003).unwrap()).value(),
            2000.0
        );
        let flow = steady_mass_flow(
            DensityKgPerM3::new(1.2).unwrap(),
            AreaM2::new(0.01).unwrap(),
            VelocityMPerS::new(3.0).unwrap(),
        );
        assert!((flow.value() - 0.036).abs() < 1e-12);
    }

    #[test]
    fn invalid_inputs_are_rejected() {
        assert!(MassKg::new(0.0).is_err());
        assert!(AreaM2::new(-1.0).is_err());
        assert!(DensityKgPerM3::new(f64::NAN).is_err());
    }
}
