use serde::{Deserialize, Serialize};

use super::{analyzer::Analyzer, detector::Detector, is_element::IsElement, source::Source};

#[derive(Clone, Debug, Serialize, Deserialize)]
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

impl IsElement for ComponentList {
    fn validate(&self) -> anyhow::Result<()> {
        for source in &self.sources {
            source.validate()?;
        }
        for analyzer in &self.analyzers {
            analyzer.validate()?;
        }
        for detector in &self.detectors {
            detector.validate()?;
        }
        Ok(())
    }
}
