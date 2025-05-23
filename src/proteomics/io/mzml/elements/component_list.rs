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

// Makes no sense do implement IsList for ComponentList attributes
// as they need to be called like `<ComponentList as IsList<'_, Analyzer>>::iter(&config.component_list)`
//
// impl IsList<'_, Source> for ComponentList {
//     fn iter(&self) -> Elements<'_, Source> {
//         Elements::new(&self.sources)
//     }
// }
//
// impl IsList<'_, Analyzer> for ComponentList {
//     fn iter(&self) -> Elements<'_, Analyzer> {
//         Elements::new(&self.analyzers)
//     }
// }
//
// impl IsList<'_, Detector> for ComponentList {
//     fn iter(&self) -> Elements<'_, Detector> {
//         Elements::new(&self.detectors)
//     }
// }
