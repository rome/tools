use super::{ExtensionHandler, Mime};
use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq)]
pub struct JsFileHandler;

/// Features that belong to the language
#[derive(Debug)]
pub struct JsFileFeatures {
    /// Whether the file is executed as a script
    pub script: bool,
    /// Whether the file is executed as a module
    pub module: bool,
    /// Whether the TS grammar is supported
    pub typescript: bool,
    /// Whether the JSX syntax is supported
    pub jsx: bool,
    /// Whether the TSX syntax is supported
    pub tsx: bool,
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

impl Default for JsFileFeatures {
    fn default() -> Self {
        Self {
            module: true,
            script: false,
            jsx: false,
            tsx: false,
            typescript: false,
        }
    }
}

impl JsFileFeatures {
    /// Mark the file as module
    pub fn module(mut self) -> Self {
        self.module = true;
        self.script = false;
        self
    }

    /// Mark the file as a script
    ///
    /// With this, most of the other features are turned off
    pub fn script(mut self) -> Self {
        self.script = true;
        self.module = false;
        self.typescript = false;
        self.tsx = false;
        self.jsx = false;
        self
    }

    /// Enables TypeScript
    pub fn typescript(mut self) -> Self {
        self.script = false;
        self.module = true;
        self.typescript = true;
        self
    }
}
