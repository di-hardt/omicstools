#[cfg(test)]
mod test {
    use std::{fs::read_to_string, path::PathBuf};

    use crate::proteomics::io::mzml::elements::indexed_mz_ml::IndexedMzML;

    #[test]
    fn test_reader_new() {
        let mzml_path = PathBuf::from("test_files/spectra_small.mzML");
        let mzml_content = read_to_string(mzml_path).unwrap();
        let mzml = quick_xml::de::from_str::<IndexedMzML>(&mzml_content).unwrap();
        // println!("{:?}", mzml.mz_ml);
        println!("\n\n");
        println!("{:?}", mzml.mz_ml.data_processing_list);
    }
}
