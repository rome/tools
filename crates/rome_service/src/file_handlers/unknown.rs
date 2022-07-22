use super::ExtensionHandler;

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct UnknownFileHandler {}

impl ExtensionHandler for UnknownFileHandler {
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
        super::Language::Unknown
    }

    fn mime(&self) -> super::Mime {
        super::Mime::Text
    }

    fn may_use_tabs(&self) -> bool {
        true
    }
}
