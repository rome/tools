use super::ExtensionHandler;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct UnknownFileHandler {}

impl ExtensionHandler for UnknownFileHandler {
    fn capabilities(&self) -> super::Capabilities {
        super::Capabilities {
            format: false,
            lint: false,
        }
    }

    fn language(&self) -> super::Language {
        super::Language::Unknown
    }

    fn mime(&self) -> super::Mime {
        super::Mime::Text
    }

    fn may_use_tabs(&self) -> bool {
        true
    }
}
impl Default for UnknownFileHandler {
    fn default() -> Self {
        Self {}
    }
}
