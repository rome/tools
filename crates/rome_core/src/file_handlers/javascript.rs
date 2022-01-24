use super::{ExtensionHandler, Mime};

#[derive(Debug, PartialEq, Eq)]
pub struct JsFileHandler;

#[derive(Debug)]
pub struct JsFileFeatures {
    pub script: bool,
    pub module: bool,
}

impl ExtensionHandler for JsFileHandler {
    fn capabilities(&self) -> super::Capabilities {
        super::Capabilities {
            format: true,
            lint: false,
        }
    }

    fn language(&self) -> super::Language {
        super::Language::Js
    }

    fn mime(&self) -> super::Mime {
        Mime::Javascript
    }

    fn may_use_tabs(&self) -> bool {
        true
    }
}

impl JsFileFeatures {
    pub fn module() -> Self {
        Self {
            script: false,
            module: true,
        }
    }
    pub fn script() -> Self {
        Self {
            script: true,
            module: false,
        }
    }
}
