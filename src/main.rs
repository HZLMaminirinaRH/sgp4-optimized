use std::fs;
use std::time::Instant;

mod sgp4;
mod tle;
mod errors;

use sgp4::{SGP4Propagator, PropagationState};
use tle::TLE;
use errors::SGP4Error;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tle_content = fs::read_to_string("satellites.tle")?;
    let tles = parse_tle_file(&tle_content)?;
    
    println!("Chargement de {} satellites", tles.len());
    
    let propagators: Vec<_> = tles
        .iter()
        .filter_map(|tle| SGP4Propagator::new(tle).ok())
        .collect();
    
    println!("Propagateurs initialisés: {}", propagators.len());
    
    let start = Instant::now();
    let results = propagate_batch(&propagators, 0.0, 1440.0, 60.0)?;
    let elapsed = start.elapsed();
    
    println!("⏱️  Propagation de {} positions en {:.3}s", 
             results.len(), elapsed.as_secs_f64());
    
    save_results(&results)?;
    
    Ok(())
}

fn parse_tle_file(content: &str) -> Result<Vec<TLE>, SGP4Error> {
    let mut tles = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    
    let mut i = 0;
    while i + 2 < lines.len() {
        let name = lines[i].trim();
        let line1 = lines[i + 1];
        let line2 = lines[i + 2];
        
        tles.push(TLE::from_lines(name, line1, line2)?);
        i += 3;
    }
    
    Ok(tles)
}

fn propagate_batch(
    propagators: &[SGP4Propagator],
    start_minutes: f64,
    end_minutes: f64,
    step_minutes: f64,
) -> Result<Vec<PropagationState>, SGP4Error> {
    use rayon::prelude::*;
    
    let mut all_states = Vec::new();
    let times: Vec<f64> = (0..((end_minutes - start_minutes) / step_minutes) as i32)
        .map(|i| start_minutes + i as f64 * step_minutes)
        .collect();
    
    for propagator in propagators {
        let states: Vec<_> = times
            .par_iter()
            .filter_map(|&t| propagator.propagate(t).ok())
            .collect();
        all_states.extend(states);
    }
    
    Ok(all_states)
}

fn save_results(states: &[PropagationState]) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(states)?;
    fs::write("output.json", json)?;
    println!("Résultats sauvegardés dans output.json");
    Ok(())
}
