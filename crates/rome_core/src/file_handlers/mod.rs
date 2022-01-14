pub mod javascript;
pub mod json;
pub mod unknown;

pub enum Language {
    Js,
    Json,
    Ts,
    Unknown,
}

pub enum Mime {
    Javascript,
    Json,
    Css,
    Text,
}

impl std::fmt::Display for Mime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mime::Css => write!(f, "text/css"),
            Mime::Json => write!(f, "application/json"),
            Mime::Javascript => write!(f, "application/javascript"),
            Mime::Text => write!(f, "text/plain"),
        }
    }
}

#[derive(Debug)]
pub struct Capabilities {
    pub lint: bool,
    pub format: bool,
}

/// Main trait to use to add a new language to Rome
pub trait ExtensionHandler {
    /// The language of the file. It can be a super language.
    /// For example, a ".js" file can have [Language::Ts]
    fn language(&self) -> Language;

    /// MIME types used to identify a certain language
    fn mime(&self) -> Mime;

    /// A file that can support tabs inside its content
    fn may_use_tabs(&self) -> bool {
        true
    }

    /// Capabilities that can applied to a file
    fn capabilities(&self) -> Capabilities {
        Capabilities {
            format: false,
            lint: false,
        }
    }

    /// How a file should be treated. Usually an asset doesn't posses a parser.
    ///
    /// An image should me parked as asset.
    fn is_asset(&self) -> bool {
        false
    }
}
