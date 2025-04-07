use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{
    file_checksum::FileChecksum, index_list::IndexList, index_list_offset::IndexListOffset,
    is_element::IsElement, mz_ml::MzML,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexedMzML<R>
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
    #[serde(rename = "mzML")]
    pub mz_ml: MzML<R>,
    #[serde(rename = "indexList")]
    pub index_list: IndexList,
    #[serde(rename = "indexListOffset")]
    pub index_list_offset: IndexListOffset,
    #[serde(rename = "fileChecksum")]
    pub file_checksum: FileChecksum,
}

impl<R> IsElement for IndexedMzML<R>
where
    R: IsElement,
{
    fn validate(&self) -> Result<()> {
        self.mz_ml.validate()?;
        self.index_list.validate()?;
        self.index_list_offset.validate()?;
        self.file_checksum.validate()?;
        Ok(())
    }
}
