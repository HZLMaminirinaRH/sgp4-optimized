use crate::errors::SGP4Error;

pub struct TLE {
    pub name: String,
    pub norad_id: u32,
    pub epoch_year: u32,
    pub epoch_day: f64,
    pub mean_motion: f64,
    pub eccentricity: f64,
    pub inclination: f64,
    pub raan: f64,
    pub argument_perigee: f64,
    pub mean_anomaly: f64,
}

impl TLE {
    pub fn from_lines(name: &str, line1: &str, line2: &str) -> Result<Self, SGP4Error> {
        if line1.len() < 69 || line2.len() < 69 {
            return Err(SGP4Error::InvalidTLE("Format TLE invalide".into()));
        }
        
        let norad_id: u32 = line1[2..7].trim().parse()
            .map_err(|_| SGP4Error::InvalidTLE("NORAD ID invalide".into()))?;
        
        let epoch_year: u32 = line1[18..20].parse()
            .map_err(|_| SGP4Error::InvalidTLE("Année époque invalide".into()))?;
        
        let epoch_day: f64 = line1[20..32].trim().parse()
            .map_err(|_| SGP4Error::InvalidTLE("Jour époque invalide".into()))?;
        
        let mean_motion: f64 = line2[52..63].trim().parse()
            .map_err(|_| SGP4Error::InvalidTLE("Moyen mouvement invalide".into()))?;
        
        let eccentricity: f64 = format!("0.{}", line2[26..33].trim())
            .parse()
            .map_err(|_| SGP4Error::InvalidTLE("Excentricité invalide".into()))?;
        
        let inclination: f64 = line2[8..16].trim().parse()
            .map_err(|_| SGP4Error::InvalidTLE("Inclinaison invalide".into()))?;
        
        let raan: f64 = line2[17..25].trim().parse()
            .map_err(|_| SGP4Error::InvalidTLE("RAAN invalide".into()))?;
        
        let argument_perigee: f64 = line2[34..42].trim().parse()
            .map_err(|_| SGP4Error::InvalidTLE("Argument du périgée invalide".into()))?;
        
        let mean_anomaly: f64 = line2[43..51].trim().parse()
            .map_err(|_| SGP4Error::InvalidTLE("Anomalie moyenne invalide".into()))?;
        
        Ok(TLE {
            name: name.to_string(),
            norad_id,
            epoch_year,
            epoch_day,
            mean_motion,
            eccentricity,
            inclination: inclination.to_radians(),
            raan: raan.to_radians(),
            argument_perigee: argument_perigee.to_radians(),
            mean_anomaly: mean_anomaly.to_radians(),
        })
    }
}
