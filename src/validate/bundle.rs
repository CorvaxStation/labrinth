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
        let valid_suffixes = ["/", ".yaml", ".yml", "meta.json", ".png", ".ogg"];
        for filename in archive.file_names() {
            if !valid_suffixes.iter().any(|suffix| filename.ends_with(suffix)) {
                return Err(ValidationError::InvalidInput(
                    "Archive can contain only YAML/OGG/PNG files or RSI packs.".into(),
                ));
            }
        }

        Ok(ValidationResult::Pass)
    }
}
