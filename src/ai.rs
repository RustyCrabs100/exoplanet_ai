// This is the location for creating the AI/ML Model

use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};
use std::{error::Error, ops::Range};

/// This is the struct that is outputted after converting the .csv data into Rust data
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TrainingRow {
    /// Candidate disposition
    /// Aliases: "disposition", "tfopwg_disp", "koi_disposition"
    /// Values:
    ///     Confirmed: "CONFIRMED", "CP", "VALIDATED", "KP"
    ///     Candidates: "CANDIDATE", "PC", "CANDIDATE PLANET", "APC"
    ///     False Positives: "FALSE POSITIVE", "FP", "FA"
    #[serde(alias = "disposition")]
    #[serde(alias = "koi_disposition")]
    #[serde(alias = "tfopwg_disp")]
    pub status: String,

    /// Unique star identifier
    /// TESS: "tid", "tic", "tic_id", "TIC", "TIC ID"
    /// K2:   "epic_id", "epic", "EPIC", "EPIC ID"
    /// Kepler/KOI: "kepid", "kepler_id", "Kepler ID"
    #[serde(alias = "tid")]
    #[serde(alias = "tic")]
    #[serde(alias = "tic_id")]
    #[serde(alias = "TIC")]
    #[serde(alias = "TIC ID")]
    #[serde(alias = "epic_id")]
    #[serde(alias = "epic")]
    #[serde(alias = "EPIC")]
    #[serde(alias = "EPIC ID")]
    #[serde(alias = "kepid")]
    #[serde(alias = "kepler_id")]
    #[serde(alias = "Kepler ID")]
    pub star_id: Option<String>,

    /// Unique planet/candidate identifier (mission-native label or name)
    /// TESS: "toi", "TOI", "pl_name", "Planet Name"
    /// K2:   "epic_cand_id", "K2 Name", "pl_name"
    /// KOI:  "koi", "KOI", "kepoi_name", "KOI Name", "pl_name"
    #[serde(alias = "toi")]
    #[serde(alias = "TOI")]
    #[serde(alias = "epic_cand_id")]
    #[serde(alias = "koi")]
    #[serde(alias = "KOI")]
    #[serde(alias = "pl_name")]
    #[serde(alias = "planet_name")]
    #[serde(alias = "Planet Name")]
    #[serde(alias = "kepoi_name")]
    #[serde(alias = "KOI Name")]
    #[serde(alias = "K2 Name")]
    pub planet_id: Option<String>,

    /// Right Ascension (deg)
    #[serde(alias = "ra")]
    pub ra: Option<f64>,

    /// Declination (deg)
    #[serde(alias = "dec")]
    pub dec: Option<f64>,

    /// TESS magnitude (Tmag) or Kepler magnitude (kepmag/Kp)
    #[serde(alias = "Tmag")]
    #[serde(alias = "kepmag")]
    #[serde(alias = "Kp")]
    pub mag: Option<f64>,

    /// Orbital period (days)
    #[serde(alias = "pl_orbper")]
    #[serde(alias = "koi_period")]
    #[serde(alias = "Period")]
    pub orbital_period_days: Option<f64>,

    /// Transit epoch in BJD(TDB)
    #[serde(alias = "pl_tranmid")]
    #[serde(alias = "koi_time0bk")]
    #[serde(alias = "Epoch")]
    pub transit_epoch_bjd: Option<f64>,

    /// Transit duration (hours)
    #[serde(alias = "koi_duration")]
    #[serde(alias = "Duration")]
    pub transit_duration_hr: Option<f64>,

    /// Transit depth (ppm)
    #[serde(alias = "koi_depth")]
    #[serde(alias = "Depth")]
    pub transit_depth_ppm: Option<f64>,

    /// Planet-to-star radius ratio (Rp/Rs)
    #[serde(alias = "koi_ror")]
    #[serde(alias = "Rp/Rs")]
    pub rp_over_rs: Option<f64>,

    /// Planet radius (Earth radii)
    #[serde(alias = "pl_rade")]
    #[serde(alias = "koi_prad")]
    #[serde(alias = "Planet Radius")]
    pub planet_radius_re: Option<f64>,

    /// Planet equilibrium temperature (K)
    #[serde(alias = "pl_eqt")]
    #[serde(alias = "koi_teq")]
    #[serde(alias = "Equilibrium Temp")]
    pub planet_eq_temp_k: Option<f64>,

    /// Stellar effective temperature (K)
    #[serde(alias = "st_teff")]
    #[serde(alias = "koi_steff")]
    #[serde(alias = "Teff")]
    pub stellar_teff_k: Option<f64>,

    /// Stellar radius (Solar radii)
    #[serde(alias = "st_rad")]
    #[serde(alias = "koi_srad")]
    #[serde(alias = "Radius")]
    pub stellar_radius_rs: Option<f64>,

    /// Stellar mass (Solar masses)
    #[serde(alias = "st_mass")]
    #[serde(alias = "koi_smass")]
    #[serde(alias = "Mass")]
    pub stellar_mass_ms: Option<f64>,

    /// Mission tag (Kepler, K2, TESS)
    /// Useful for provenance tracking
    #[serde(alias = "mission")]
    #[serde(alias = "facility")]
    pub mission: Option<String>,

    /// Discovery year
    #[serde(alias = "disc_year")]
    #[serde(alias = "Discovery Year")]
    pub discovery_year: Option<i32>,

    /// Discovery method (Transit, Radial Velocity, Imaging, etc.)
    #[serde(alias = "discoverymethod")]
    #[serde(alias = "Discovery Method")]
    pub discovery_method: Option<String>,

    /// Orbital eccentricity
    #[serde(alias = "pl_orbeccen")]
    #[serde(alias = "Eccentricity")]
    pub eccentricity: Option<f64>,

    /// Orbital inclination (deg)
    #[serde(alias = "pl_orbincl")]
    #[serde(alias = "Inclination")]
    pub inclination_deg: Option<f64>,

    /// Semi-major axis (AU)
    #[serde(alias = "pl_orbsmax")]
    #[serde(alias = "koi_sma")]
    #[serde(alias = "Semi-major Axis")]
    pub semi_major_axis_au: Option<f64>,

    /// Planet mass (Earth masses)
    #[serde(alias = "pl_bmasse")]
    #[serde(alias = "Planet Mass")]
    pub planet_mass_me: Option<f64>,

    /// Planet mass (Jupiter masses)
    #[serde(alias = "pl_bmassj")]
    #[serde(alias = "Planet Mass [Jup]")]
    pub planet_mass_mj: Option<f64>,

    /// Planet density (g/cm^3)
    #[serde(alias = "pl_dens")]
    #[serde(alias = "Density")]
    pub planet_density: Option<f64>,

    /// Insolation flux (Earth flux units)
    #[serde(alias = "pl_insol")]
    #[serde(alias = "Insolation Flux")]
    pub insolation_flux: Option<f64>,

    /// Signal-to-noise ratio of detection
    #[serde(alias = "koi_model_snr")]
    #[serde(alias = "SNR")]
    #[serde(alias = "snr")]
    pub detection_snr: Option<f64>,

    /// Vetting score / probability of being a planet
    #[serde(alias = "koi_score")]
    #[serde(alias = "score")]
    #[serde(alias = "prob")]
    pub vetting_score: Option<f64>,

    /// Stellar metallicity [Fe/H] (dex)
    #[serde(alias = "st_metfe")]
    #[serde(alias = "koi_smet")]
    #[serde(alias = "Metallicity")]
    pub stellar_metallicity: Option<f64>,

    /// Stellar surface gravity (log g, cgs)
    #[serde(alias = "st_logg")]
    #[serde(alias = "koi_slogg")]
    #[serde(alias = "logg")]
    pub stellar_logg: Option<f64>,
}

impl std::fmt::Display for TrainingRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = serde_json::to_value(self).unwrap();
        if let serde_json::Value::Object(map) = value {
            for (k, v) in map {
                writeln!(f, "{}: {}", k, v)?;
            }
        }
        Ok(())
    }
}

impl TrainingRow {
    /// Get rid of rows that are candidates
    pub fn row_ok(&self) -> bool {
        self.status != "candidate" && self.status != "pc" && self.status != "apc"
    }
}

pub fn call_ai() {
    // This holds all the training data for exoplanets
    let mut all_confirmed: Vec<TrainingRow> = Vec::new();
    // This holds all the training data for non-exoplanets
    let mut all_false: Vec<TrainingRow> = Vec::new();
    // Collect all the training data and split them
    let (k2_p_t, k2_p_f) = read_csv("data\\k2_planets.csv").expect("uh oh");
    let (koi_p_t, koi_p_f) = read_csv("data\\koi_planets.csv").expect("un oh 2");
    let (toi_p_t, toi_p_f) = read_csv("data\\toi_planets.csv").expect("uh oh 3");
    // MERGE MERGE MERGE
    all_confirmed.extend(k2_p_t);
    all_confirmed.extend(koi_p_t);
    all_confirmed.extend(toi_p_t);
    all_false.extend(k2_p_f);
    all_false.extend(koi_p_f);
    all_false.extend(toi_p_f);
    let _ = iter_compare_planet_rows(all_confirmed, all_false);
}

/// Reads the data from the .csv file inputted into this function.
/// Returns a Result, of either any type of error or 2 vectors, exoplanets and not a exoplanet,
/// or a tuple of 2 Vec<TrainingRow>'s
/// IMPORTANT NOTICE:
///     All inputted .csv files must have a header.
///     If the header is missing, this function will output broken/weird/unexpected information.
fn read_csv(csv_file: &str) -> Result<(Vec<TrainingRow>, Vec<TrainingRow>), Box<dyn Error>> {
    // Create a refrence counted, dynamically checked for borrow rules, Reader.
    // .csv comments are defined by starting the message with #
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        // 0b00100011 = # in Binary
        .comment(Some(0b00100011))
        .from_path(csv_file)?;
    // Mutably borrow rdr
    let rdrmb = &mut rdr;
    // Create a Vec<> of rows indexed by headers
    let mut confirmed: Vec<TrainingRow> = Vec::new();
    let mut not_a_planet: Vec<TrainingRow> = Vec::new();
    // Iterate
    for result in rdrmb.deserialize::<TrainingRow>() {
        let row: TrainingRow = match result {
            Ok(r) => r,
            Err(err) => {
                eprintln!("{:#?}", err);
                continue;
            }
        };
        if !row.row_ok() {
            continue;
        }
        match row.status.trim().to_lowercase().as_str() {
            "confirmed" => confirmed.push(row),
            "kp" => confirmed.push(row),
            "cp" => confirmed.push(row),
            "validated" => confirmed.push(row),
            "false positive" => not_a_planet.push(row),
            "fp" => not_a_planet.push(row),
            "fa" => not_a_planet.push(row),
            other => eprintln!("Unexpected input, {}", other),
        }
    }
    println!("{:#?}", confirmed);
    println!("{:#?}", not_a_planet);
    Ok((confirmed, not_a_planet))
}

fn iter_compare_planet_rows(
    mut true_rows: Vec<TrainingRow>,
    mut false_rows: Vec<TrainingRow>,
) -> Range<f64> {
    todo!()
}
