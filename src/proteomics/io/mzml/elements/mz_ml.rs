use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{
    cv_list::CvList, data_processing_list::DataProcessingList, file_description::FileDescription,
    instrument_configuration_list::InstrumentConfigurationList, is_element::IsElement,
    referenceable_param_group_list::ReferenceableParamGroupList, software_list::SoftwareList,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct MzML<R>
where
    R: IsElement,
{
    #[serde(rename = "@xmlns")]
    pub xmlns: String,
    #[serde(rename = "@xmlns:xsi")]
    pub xmlns_xsi: String,
    // This is a workaround to get xsi-attributes running, see:
    // https://github.com/tafia/quick-xml/issues/553#issuecomment-1432966843
    #[serde(rename = "@xsi:schemaLocation")]
    #[serde(alias = "@schemaLocation")]
    pub xsi_schema_location: String,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@version")]
    pub version: String,
    #[serde(rename = "cvList")]
    pub cv_list: CvList,
    #[serde(rename = "fileDescription")]
    pub file_description: FileDescription,
    #[serde(rename = "referenceableParamGroupList")]
    pub referenceable_param_group_list: ReferenceableParamGroupList,
    #[serde(rename = "softwareList")]
    pub software_list: SoftwareList,
    #[serde(rename = "instrumentConfigurationList")]
    pub instrument_configuration_list: InstrumentConfigurationList,
    #[serde(rename = "dataProcessingList")]
    pub data_processing_list: DataProcessingList,
    #[serde(rename = "run")]
    pub run: R,
}

impl<R> IsElement for MzML<R>
where
    R: IsElement,
{
    fn validate(&self) -> Result<()> {
        for cv_list in &self.cv_list.cv {
            cv_list.validate()?;
        }
        self.file_description.validate()?;
        self.referenceable_param_group_list.validate()?;
        self.software_list.validate()?;
        self.instrument_configuration_list.validate()?;
        self.data_processing_list.validate()?;
        self.run.validate()?;
        Ok(())
    }
}
