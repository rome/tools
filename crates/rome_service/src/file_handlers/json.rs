use super::{ExtensionHandler, Mime};
#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct JsonFileHandler;

impl ExtensionHandler for JsonFileHandler {
    fn capabilities(&self) -> super::Capabilities {
        super::Capabilities {
            parse: None,
            debug_print: None,
            format: None,
            lint: None,
            code_actions: None,
            fix_all: None,
            format_range: None,
            format_on_type: None,
            rename: None,
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
