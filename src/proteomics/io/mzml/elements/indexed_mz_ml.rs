use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{
    file_checksum::FileChecksum,
    index_list::IndexList,
    index_list_offset::IndexListOffset,
    is_element::IsElement,
    mz_ml::MzML,
    run::{IndexedRun, IsRun, Run},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IndexedMzML<R>
where
    R: IsRun,
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
    R: IsRun,
{
    fn validate(&self) -> Result<()> {
        self.mz_ml.validate()?;
        self.index_list.validate()?;
        self.index_list_offset.validate()?;
        self.file_checksum.validate()?;
        Ok(())
    }
}

impl From<IndexedMzML<IndexedRun>> for IndexedMzML<Run> {
    fn from(indexed_mz_ml: IndexedMzML<IndexedRun>) -> Self {
        Self {
            xmlns: indexed_mz_ml.xmlns,
            xmlns_xsi: indexed_mz_ml.xmlns_xsi,
            xsi_schema_location: indexed_mz_ml.xsi_schema_location,
            mz_ml: MzML::from(indexed_mz_ml.mz_ml),
            index_list: indexed_mz_ml.index_list,
            index_list_offset: indexed_mz_ml.index_list_offset,
            file_checksum: indexed_mz_ml.file_checksum,
        }
    }
}
