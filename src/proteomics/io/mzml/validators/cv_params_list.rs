// std imports
use anyhow::{bail, Result};

// Local imports
use crate::proteomics::{io::mzml::elements::cv_param::CvParam, ontology::Ontology};

pub trait CvParamsValidator {
    fn get_parent_accession_for_must_once(&self) -> &'static Vec<&'static str>;
    fn get_parent_accession_for_must_once_or_many(&self) -> &'static Vec<&'static str>;
    fn get_parent_accession_for_may_once(&self) -> &'static Vec<&'static str>;
    fn get_parent_accession_for_zero_or_many(&self) -> &'static Vec<&'static str>;

    /// Checks if only one of the children of `get_parent_accession_for_must_once` is present in the element.
    ///
    /// # Arguments
    /// * `cv_params` - List of cvParams to be validated.
    /// * `element_tag` - The tag of the element being validated.
    ///
    fn validate_must_once(&self, cv_params: &[CvParam], element_tag: &str) -> Result<Vec<String>> {
        let mut accepted_children = Vec::new();
        for accession in self.get_parent_accession_for_must_once().iter() {
            let children = Ontology::get_children_of(accession)?;
            let mut child_matches = vec![false; children.len()];

            for (child_idx, child) in children.iter().enumerate() {
                for cv_param in cv_params.iter() {
                    if &cv_param.accession == child {
                        child_matches[child_idx] = true;
                    }
                }
            }

            let num_child_matches = child_matches.iter().filter(|m| **m).count();

            match num_child_matches {
                1 => accepted_children.extend(children),
                0 => bail!(
                    "One of the following cvParams must be present in the <{}> element: {}",
                    element_tag,
                    children.join(", ")
                ),
                _ => bail!(
                    "Only one of the following cvParams must be present in the <{}> element: {}",
                    element_tag,
                    children.join(", ")
                ),
            }
        }
        Ok(accepted_children)
    }

    /// Checks if one or many of the children of `get_parent_accession_for_must_once_or_many` is present in the element.
    ///
    /// # Arguments
    /// * `cv_params` - List of cvParams to be validated.
    /// * `element_tag` - The tag of the element being validated.
    ///
    fn validate_must_once_or_many(
        &self,
        cv_params: &[CvParam],
        element_tag: &str,
    ) -> Result<Vec<String>> {
        let mut accepted_children = Vec::new();
        for accession in self.get_parent_accession_for_must_once_or_many().iter() {
            let children = Ontology::get_children_of(accession)?;
            let mut child_matches = vec![false; children.len()];

            for (child_idx, child) in children.iter().enumerate() {
                for cv_param in cv_params.iter() {
                    if &cv_param.accession == child {
                        child_matches[child_idx] = true;
                    }
                }
            }

            let num_child_matches = child_matches.iter().filter(|m| **m).count();

            match num_child_matches {
                0 => bail!(
                    "At least one of the following cvParams must be present in the <{}> element: {}",
                    element_tag,
                    children.join(", ")
                ),
                _ => accepted_children.extend(children)
            }
        }
        Ok(accepted_children)
    }

    /// Checks if at zero or one of the children of `get_parent_accession_for_may_once` is present
    ///
    /// # Arguments
    /// * `cv_params` - List of cvParams to be validated.
    /// * `element_tag` - The tag of the element being validated.
    ///
    fn validate_may_once(&self, cv_params: &[CvParam], element_tag: &str) -> Result<Vec<String>> {
        let mut accepted_children = Vec::new();
        for accession in self.get_parent_accession_for_may_once().iter() {
            let children = Ontology::get_children_of(accession)?;
            let mut child_matches = vec![false; children.len()];

            for (child_idx, child) in children.iter().enumerate() {
                for cv_param in cv_params.iter() {
                    if &cv_param.accession == child {
                        child_matches[child_idx] = true;
                    }
                }
            }

            let num_child_matches = child_matches.iter().filter(|m| **m).count();

            match num_child_matches {
                1 => accepted_children.extend(children),
                0 => {
                    // No child matches, do nothing
                }
                _ => bail!(
                    "Only zero or one of the following cvParams can be present in the <{}> element: {}",
                    element_tag,
                    children.join(", ")
                ),
            }
        }

        Ok(accepted_children)
    }

    /// Validate the given cvParams list
    ///
    fn validate_cv_params(&self, cv_params: &[CvParam], element_tag: &str) -> Result<()> {
        // Check if all rules (one child of, one or many children of, zero or many children of) are satisfied
        // and collect each acceptable child accession
        self.validate_must_once(cv_params, element_tag)?;
        self.validate_must_once_or_many(cv_params, element_tag)?;
        self.validate_may_once(cv_params, element_tag)?;
        Ok(())
    }
}

/// This macro generates the implementation of the `CvParamsValidator` trait for the given struct.
///
/// # Arguments
/// * `$name` - The name of the struct for which the implementation is being generated.
/// * `$must_once` - Array of cvParam accession from which one child's accession must be present in the list.
/// * `$must_once_or_many` - Array of cvParam accession from which at least one child's accession must be present in the list.
/// * `$may_once` - Array of cvParam accession from which none or one child's accession must be present in the list.
/// * `$may_one_or_many` - Array of cvParam accession from which at least one child's accession must be present in the list.
///
#[macro_export]
macro_rules! build_cv_params_validator {
    ($name:ty, $must_once:tt, $must_once_or_many:tt, $may_once:tt, $may_one_or_many:tt) => {
        use $crate::proteomics::io::mzml::validators::cv_params_list::CvParamsValidator;

        lazy_static! {
            pub static ref CV_PARAMS_MUST_ONCE: Vec<&'static str> = vec!$must_once;
            pub static ref CV_PARAMS_MUST_ONCE_OR_MANY: Vec<&'static str> = vec!$must_once_or_many;
            pub static ref CV_PARAMS_MAY_ONCE: Vec<&'static str> = vec!$may_once;
            pub static ref CV_PARAMS_MAY_ONCE_OR_MANY: Vec<&'static str> = vec!$may_one_or_many;
        }

        impl CvParamsValidator for $name {
            fn get_parent_accession_for_must_once(&self) -> &'static Vec<&'static str> {
                &CV_PARAMS_MUST_ONCE
            }

            fn get_parent_accession_for_must_once_or_many(&self) -> &'static Vec<&'static str> {
                &CV_PARAMS_MUST_ONCE_OR_MANY
            }

            fn get_parent_accession_for_may_once(&self) -> &'static Vec<&'static str> {
                &CV_PARAMS_MAY_ONCE
            }

            fn get_parent_accession_for_zero_or_many(&self) -> &'static Vec<&'static str> {
                &CV_PARAMS_MAY_ONCE_OR_MANY
            }
        }
    };
}
