/// Trait defining a basic spectrum
/// Should at least contain mz and intensity vectors, the ms level and identifier.
///
pub trait Spectrum {
    /// Returns the spectrum id
    ///
    fn get_id(&self) -> &String;

    /// Returns the MsLevel
    ///
    fn get_ms_level(&self) -> u8;

    /// Returns the mz values
    ///
    fn get_mz(&self) -> &Vec<f64>;

    /// Returns the intensity values
    ///
    fn get_intensity(&self) -> &Vec<f64>;
}

/// Trait defining a n-level spectrum with a parent ID, precursor m/z and precursor charge
///
pub trait MsNSpectrum<P>: Spectrum
where
    P: Precursor,
{
    /// Returns the precursors
    ///
    fn get_precursors(&self) -> &Vec<P>;
}

/// Trait defining a precursor
///
pub trait Precursor {
    /// Returns the parent spectrum ID
    fn get_parent_id(&self) -> &String;

    /// Returns the isolation windows (mz, offset lower, offset upper)
    fn get_isolation_windows(&self) -> &Option<(f64, f64, f64)>;

    /// Returns the ions (mz, charge)
    fn get_ions(&self) -> &Vec<(f64, Vec<u8>)>;

    /// Returns the activation method
    fn get_activation(&self) -> &(String, f64);
}

/// Simplest version of a Precursor (for now)
///
pub struct SimplePrecursor {
    parent_id: String,
    /// (mz, offset lower, offset upper)
    isolation_windows: Option<(f64, f64, f64)>,
    /// (mz, charge)
    ions: Vec<(f64, Vec<u8>)>,
    /// Accession
    activation: (String, f64),
}

impl SimplePrecursor {
    pub fn new(
        parent_id: String,
        isolation_windows: Option<(f64, f64, f64)>,
        ions: Vec<(f64, Vec<u8>)>,
        activation: (String, f64),
    ) -> Self {
        Self {
            parent_id,
            isolation_windows,
            ions,
            activation,
        }
    }
}

impl Precursor for SimplePrecursor {
    fn get_parent_id(&self) -> &String {
        &self.parent_id
    }

    fn get_isolation_windows(&self) -> &Option<(f64, f64, f64)> {
        &self.isolation_windows
    }

    fn get_ions(&self) -> &Vec<(f64, Vec<u8>)> {
        &self.ions
    }

    fn get_activation(&self) -> &(String, f64) {
        &self.activation
    }
}

/// Simplest version of a Spectrum
/// Contains mz, intensity, id and ms_level
/// Depending on the file format there might be a whole lot more information available.
///
pub struct SimpleSpectrum {
    id: String,
    ms_level: u8,
    mz: Vec<f64>,
    intensity: Vec<f64>,
}

impl SimpleSpectrum {
    pub fn new(id: String, ms_level: u8, mz: Vec<f64>, intensity: Vec<f64>) -> Self {
        Self {
            id,
            ms_level,
            mz,
            intensity,
        }
    }
}

impl Spectrum for SimpleSpectrum {
    fn get_id(&self) -> &String {
        &self.id
    }

    fn get_ms_level(&self) -> u8 {
        self.ms_level
    }

    fn get_mz(&self) -> &Vec<f64> {
        &self.mz
    }

    fn get_intensity(&self) -> &Vec<f64> {
        &self.intensity
    }
}

/// Simplest version of an MsNSpectrum
/// Contains mz, intensity, id, ms_level, precursor_mz and precursor_charge
/// Depending on the file format there might be a whole lot more information available.
/// But this should be the bare minimum to work with
///
pub struct SimpleMsNSpectrum {
    id: String,
    ms_level: u8,
    mz: Vec<f64>,
    intensity: Vec<f64>,
    precursors: Vec<SimplePrecursor>,
}

impl SimpleMsNSpectrum {
    pub fn new(
        id: String,
        ms_level: u8,
        mz: Vec<f64>,
        intensity: Vec<f64>,
        precursors: Vec<SimplePrecursor>,
    ) -> Self {
        Self {
            id,
            ms_level,
            mz,
            intensity,
            precursors,
        }
    }
}

impl Spectrum for SimpleMsNSpectrum {
    fn get_id(&self) -> &String {
        &self.id
    }

    fn get_ms_level(&self) -> u8 {
        self.ms_level
    }

    fn get_mz(&self) -> &Vec<f64> {
        &self.mz
    }

    fn get_intensity(&self) -> &Vec<f64> {
        &self.intensity
    }
}

impl MsNSpectrum<SimplePrecursor> for SimpleMsNSpectrum {
    fn get_precursors(&self) -> &Vec<SimplePrecursor> {
        &self.precursors
    }
}
