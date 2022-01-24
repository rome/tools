use super::{ExtensionHandler, Mime};
#[derive(Debug, Default, PartialEq, Eq)]
pub struct JsonFileHandler;

#[derive(Default)]
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
