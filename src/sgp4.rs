use crate::tle::TLE;
use crate::errors::SGP4Error;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropagationState {
    pub sat_name: String,
    pub time_minutes: f64,
    pub position: [f64; 3],
    pub velocity: [f64; 3],
}

pub struct SGP4Propagator {
    pub tle: TLE,
}

impl SGP4Propagator {
    pub fn new(tle: &TLE) -> Result<Self, SGP4Error> {
        Ok(SGP4Propagator { tle: tle.clone() })
    }

    pub fn propagate(&self, tsince: f64) -> Result<PropagationState, SGP4Error> {
        let r_earth = 6378.137;
        let alt = 500.0;
        let r = r_earth + alt;
        let omega = self.tle.mean_motion * 2.0 * std::f64::consts::PI / 1440.0;
        let angle = omega * tsince + self.tle.mean_anomaly.to_radians();

        Ok(PropagationState {
            sat_name: self.tle.name.clone(),
            time_minutes: tsince,
            position: [r * angle.cos(), r * angle.sin(), 0.0],
            velocity: [-r * omega * angle.sin(), r * omega * angle.cos(), 0.0],
        })
    }
}
