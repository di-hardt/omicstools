// 3rd party imports
use serde::{Deserialize, Serialize};

// Local imports
use super::source_file::SourceFile;

#[derive(Debug, Serialize, Deserialize)]
pub struct SourceFileList {
    #[serde(rename = "@count")]
    pub count: usize,
    #[serde(default, rename = "sourceFile")]
    pub source_files: Vec<SourceFile>,
}
