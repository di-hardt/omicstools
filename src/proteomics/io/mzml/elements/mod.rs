pub mod activation;
pub mod analyzer;
pub mod binary;
pub mod binary_data_array;
pub mod binary_data_array_list;
pub mod chromatogram;
pub mod chromatogram_list;
pub mod component_list;
pub mod cv;
pub mod cv_list;
pub mod cv_param;
pub mod data_processing;
pub mod data_processing_list;
pub mod detector;
pub mod file_checksum;
pub mod file_content;
pub mod file_description;
/// Trait describing behavior for elements with cv params
pub mod has_cv_params;
pub mod index;
pub mod index_list;
pub mod index_list_offset;
pub mod indexed_mz_ml;
pub mod instrument_configuration;
pub mod instrument_configuration_list;
pub mod is_element;
pub mod isolation_window;
pub mod mz_ml;
pub mod offset;
pub mod precursor;
pub mod precursor_list;
pub mod processing_method;
pub mod referenceable_param_group;
pub mod referenceable_param_group_list;
pub mod referenceable_param_group_ref;
pub mod run;
pub mod scan;
pub mod scan_list;
pub mod scan_window;
pub mod scan_window_list;
pub mod selected_ion;
pub mod selected_ion_list;
pub mod software;
pub mod software_list;
pub mod source;
pub mod source_file;
pub mod source_file_list;
pub mod spectrum;
pub mod spectrum_list;
pub mod user_param;

/// Trait for list elements
pub mod is_list;
