use std::io::Read;

use anyhow::{bail, Result};
use base64::{prelude::BASE64_STANDARD, Engine};
use flate2::read::ZlibDecoder;
use serde::{Deserialize, Serialize};

use super::{
    binary::Binary, cv_param::CvParam, is_element::IsElement,
    referenceable_param_group_ref::ReferenceableParamGroupRef, user_param::UserParam,
};
use crate::build_cv_params_validator;

#[derive(Debug, Serialize, Deserialize)]
pub struct BinaryDataArray {
    #[serde(rename = "@encodedLength")]
    pub encoded_length: usize,
    #[serde(rename = "@arrayLength")]
    pub array_length: Option<usize>,
    #[serde(rename = "@dataProcessingRef")]
    pub data_processing_ref: Option<String>,
    #[serde(default, rename = "referenceableParamGroupRef")]
    pub referenceable_param_group_ref: Vec<ReferenceableParamGroupRef>,
    #[serde(default, rename = "cvParam")]
    pub cv_params: Vec<CvParam>,
    #[serde(default, rename = "userParam")]
    pub user_params: Vec<UserParam>,
    #[serde(rename = "binary")]
    pub binary: Binary,
}

impl BinaryDataArray {
    pub fn deflate_data(&self) -> Result<Vec<f64>> {
        let compression_cv_param = self.cv_params.iter().find(|cv_param| {
            cv_param.accession == "MS:1000574" || cv_param.accession == "MS:1000576"
        });

        if compression_cv_param.is_none() {
            bail!("Compression cvParam not found");
        }

        let compression_cv_param = compression_cv_param.unwrap();

        let uncompressed_data = match compression_cv_param.accession.as_str() {
            "MS:1000574" => {
                let decoded_data = BASE64_STANDARD.decode(&self.binary.data)?;
                let mut deflated_buffer: Vec<u8> = Vec::new();
                let mut decoder = ZlibDecoder::new(decoded_data.as_slice());
                decoder.read_to_end(&mut deflated_buffer)?;
                deflated_buffer
            }
            "MS:1000576" => BASE64_STANDARD.decode(&self.binary.data)?,
            _ => bail!("Unknown compression cvParam"),
        };

        let type_cv_param = self.cv_params.iter().find(|cv_param| {
            cv_param.accession == "MS:1000521" || cv_param.accession == "MS:1000523"
        });

        if type_cv_param.is_none() {
            bail!("Data type cvParam not found");
        }

        let type_cv_param = type_cv_param.unwrap();

        match type_cv_param.accession.as_str() {
            "MS:1000521" => {
                if uncompressed_data.len() % 4 != 0 {
                    bail!("Uncompressed data array is not a multiple of 4");
                }
                Ok(uncompressed_data
                    .chunks(4)
                    .map(|chunk| f32::from_le_bytes(chunk.try_into().unwrap()) as f64)
                    .collect())
            }
            "MS:1000523" => {
                if uncompressed_data.len() % 8 != 0 {
                    bail!("Uncompressed data array is not a multiple of 8");
                }
                Ok(uncompressed_data
                    .chunks(8)
                    .map(|chunk| f64::from_le_bytes(chunk.try_into().unwrap()))
                    .collect())
            }
            _ => bail!("Unknown data type cvParam"),
        }
    }
}

impl IsElement for BinaryDataArray {
    fn validate(&self) -> Result<()> {
        if self.binary.data.len() != self.encoded_length {
            bail!("Encoded length does not match data length");
        }
        self.validate_cv_params(&self.cv_params, "binaryDataArray")?;
        Ok(())
    }
}

build_cv_params_validator! {
    BinaryDataArray,
    [
        "MS:1000572", // binary data compression type
        "MS:1000513", // binary data array
        "MS:1000518", // binary data type
    ],
    [],
    [],
    []
}
