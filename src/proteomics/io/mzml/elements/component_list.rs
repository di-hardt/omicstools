// 3rd party imports
use serde::{Deserialize, Serialize};

// Local imports
use super::{analyzer::Analyzer, detector::Detector, source::Source};

#[derive(Debug, Serialize, Deserialize)]
pub struct ComponentList {
    #[serde(rename = "@count")]
    pub count: usize,
    #[serde(rename = "source")]
    pub sources: Vec<Source>,
    #[serde(rename = "analyzer")]
    pub analyzers: Vec<Analyzer>,
    #[serde(rename = "detector")]
    pub detectors: Vec<Detector>,
}
