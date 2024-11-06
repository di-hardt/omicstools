// std imports

use std::collections::HashSet;

use anyhow::{bail, Result};

// Local imports
use crate::proteomics::{io::mzml::elements::cv_param::CvParam, ontology::get_children_of};

pub trait CvParamsValidator {
    fn get_parent_accession_for_one_child_of(&self) -> &'static Vec<&'static str>;
    fn get_parent_accession_for_one_or_many_children_of(&self) -> &'static Vec<&'static str>;
    fn get_parent_accession_for_zero_or_many_children_of(&self) -> &'static Vec<&'static str>;

    /// Checks if only one of the children of `get_parent_accession_for_one_child_of`` is present in the element.
    ///
    /// # Arguments
    /// * `cv_params` - List of cvParams to be validated.
    /// * `element_tag` - The tag of the element being validated.
    ///
    fn validate_one_child_of(
        &self,
        cv_params: &Vec<CvParam>,
        element_tag: &str,
    ) -> Result<Vec<String>> {
        let mut accepted_children = Vec::new();
        for accession in self.get_parent_accession_for_one_child_of().iter() {
            let children = get_children_of(accession)?;
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
        return Ok(accepted_children);
    }

    /// Checks if at least one of the children of `get_parent_accession_for_one_or_many_children_of` is present
    ///
    /// # Arguments
    /// * `cv_params` - List of cvParams to be validated.
    /// * `element_tag` - The tag of the element being validated.
    ///
    fn validate_one_or_many_children(
        &self,
        cv_params: &Vec<CvParam>,
        element_tag: &str,
    ) -> Result<Vec<String>> {
        let mut accepted_children = Vec::new();
        'accession_loop: for accession in self
            .get_parent_accession_for_one_or_many_children_of()
            .iter()
        {
            let children = get_children_of(accession)?;
            for child in children.iter() {
                for cv_param in cv_params.iter() {
                    if &cv_param.accession == child {
                        accepted_children.extend(children);
                        continue 'accession_loop;
                    }
                }
            }
            bail!(
                "At least one of the following cvParams must be present in the <{}> element: {}",
                element_tag,
                children.join(", ")
            );
        }

        return Ok(accepted_children);
    }

    /// Returns the child accession of `get_parent_accession_for_zero_or_many_children_of`
    ///
    fn get_zero_or_many_child_accessions(&self) -> Result<Vec<String>> {
        Ok(self
            .get_parent_accession_for_zero_or_many_children_of()
            .iter()
            .map(|accession| {
                let children = get_children_of(accession)?;
                Ok(children)
            })
            .collect::<Result<Vec<Vec<String>>>>()?
            .into_iter()
            .flatten()
            .collect())
    }

    /// Validate the given cvParams list
    ///
    fn validate_cv_params(&self, cv_params: &Vec<CvParam>, element_tag: &str) -> Result<()> {
        // Check if all rules (one child of, one or many children of, zero or many children of) are satisfied
        // and collect each acceptable child accession
        let mut acceptable_children = self.validate_one_child_of(cv_params, element_tag)?;
        acceptable_children.extend(self.validate_one_or_many_children(cv_params, element_tag)?);
        acceptable_children.extend(self.get_zero_or_many_child_accessions()?);
        let acceptable_children: HashSet<String> = acceptable_children.into_iter().collect();

        // Check if only acceptable children are present in the cvParams list
        for cv_param in cv_params.iter() {
            if !acceptable_children.contains(&cv_param.accession) {
                bail!(
                    "The cvParam <{}> is not allowed in the <{}> element",
                    cv_param.accession,
                    element_tag
                );
            }
        }
        Ok(())
    }
}

/// This macro generates the implementation of the `CvParamsValidator` trait for the given struct.
///
/// # Arguments
/// * `$name` - The name of the struct for which the implementation is being generated.
/// * `$once` - Array of cvParam accession from which one child's accession must be present in the list.
/// * `$one_or_many` - Array of cvParam accession from which at least one child's accession must be present in the list.
/// * `$zero_or_many` - Array of cvParam accession from which zero or many child's accession must be present in the list.
///
#[macro_export]
macro_rules! build_cv_params_validator {
    ($name:ty, $once:tt, $one_or_many:tt, $zero_or_many:tt) => {
        use crate::proteomics::io::mzml::validators::cv_params_list::CvParamsValidator;

        lazy_static! {
            pub static ref CV_PARAMS_ONCE: Vec<&'static str> = vec!$once;
            pub static ref CV_PARAMS_ONE_OR_MANY: Vec<&'static str> = vec!$one_or_many;
            pub static ref CV_PARAMS_ZERO_OR_MANY: Vec<&'static str> = vec!$zero_or_many;
        }

        impl CvParamsValidator for $name {
            fn get_parent_accession_for_one_child_of(&self) -> &'static Vec<&'static str> {
                &CV_PARAMS_ONCE
            }

            fn get_parent_accession_for_one_or_many_children_of(
                &self,
            ) -> &'static Vec<&'static str> {
                &CV_PARAMS_ONE_OR_MANY
            }

            fn get_parent_accession_for_zero_or_many_children_of(
                &self,
            ) -> &'static Vec<&'static str> {
                &CV_PARAMS_ZERO_OR_MANY
            }
        }
    };
}
