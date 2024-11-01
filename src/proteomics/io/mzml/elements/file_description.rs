// 3rd party imports
use serde::{Deserialize, Serialize};

use super::{file_content::FileContent, source_file_list::SourceFileList};

// Local imports

#[derive(Debug, Serialize, Deserialize)]
pub struct FileDescription {
    #[serde(rename = "fileContent")]
    pub file_content: FileContent,
    #[serde(rename = "sourceFileList")]
    pub source_file_list: SourceFileList,
}
