use crate::validate::{
    SupportedGameVersions, ValidationError, ValidationResult,
};
use std::io::Cursor;
use zip::ZipArchive;

pub struct BundleValidator;

impl super::Validator for BundleValidator {
    fn get_file_extensions(&self) -> &[&str] {
        &["zip"]
    }

    fn get_project_types(&self) -> &[&str] {
        &["bundle"]
    }

    fn get_supported_loaders(&self) -> &[&str] {
        &["launcher"]
    }

    fn get_supported_game_versions(&self) -> SupportedGameVersions { SupportedGameVersions::All }

    fn validate(
        &self,
        archive: &mut ZipArchive<Cursor<bytes::Bytes>>,
    ) -> Result<ValidationResult, ValidationError> {
        for filename in archive.file_names() {
            if !filename.ends_with(".yaml") || !filename.ends_with(".yml") || !filename.ends_with(".rsi") || !filename.ends_with("meta.json") || !filename.ends_with(".png") {
                return Err(ValidationError::InvalidInput(
                    "Archive can contain only YAML files or RSI packs.".into(),
                ));
            }
        }

        Ok(ValidationResult::Pass)
    }
}
