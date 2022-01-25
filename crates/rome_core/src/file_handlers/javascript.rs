use super::{ExtensionHandler, Mime};
use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq)]
pub struct JsFileHandler;

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
