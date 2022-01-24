use super::{ExtensionHandler, Mime};
#[derive(Debug, PartialEq, Eq)]
pub struct JsonFileHandler;

pub struct JsonFileFeatures {}

impl ExtensionHandler for JsonFileHandler {
    fn capabilities(&self) -> super::Capabilities {
        super::Capabilities {
            format: true,
            lint: false,
        }
    }

    fn language(&self) -> super::Language {
        super::Language::Json
    }

    fn mime(&self) -> super::Mime {
        Mime::Json
    }

    fn may_use_tabs(&self) -> bool {
        true
    }
}

impl Default for JsonFileFeatures {
    fn default() -> Self {
        Self {}
    }
}
