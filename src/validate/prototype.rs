use crate::validate::{
    SupportedGameVersions, ValidationError, ValidationResult,
};
use std::io::Cursor;
use zip::ZipArchive;

pub struct PrototypeValidator;

impl super::Validator for PrototypeValidator {
    fn get_file_extensions(&self) -> &[&str] {
        &["zip"]
    }

    fn get_project_types(&self) -> &[&str] {
        &["prototype"]
    }

    fn get_supported_loaders(&self) -> &[&str] {
        &["launcher"]
    }

    fn get_supported_game_versions(&self) -> SupportedGameVersions { SupportedGameVersions::All }

    fn validate(
        &self,
        archive: &mut ZipArchive<Cursor<bytes::Bytes>>,
    ) -> Result<ValidationResult, ValidationError> {
        let have_yaml_file = archive.file_names().any(|filename| !filename.ends_with(".yaml") && !filename.ends_with(".yml"));
        if have_yaml_file {
            return Err(ValidationError::InvalidInput(
                "Archive can contain only YAML files with prototypes.".into(),
            ));
        }

        Ok(ValidationResult::Pass)
    }
}
