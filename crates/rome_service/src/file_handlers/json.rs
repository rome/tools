use super::{ExtensionHandler, Mime};
use crate::file_handlers::{Capabilities, ParserCapabilities};
use crate::file_handlers::{DebugCapabilities, Language as LanguageId};
use crate::workspace::server::AnyParse;
use crate::workspace::GetSyntaxTreeResult;
use rome_fs::RomePath;
use rome_json_parser::JsonParse;
use rome_json_syntax::{JsonRoot, JsonSyntaxNode};

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct JsonFileHandler;

impl ExtensionHandler for JsonFileHandler {
    fn language(&self) -> super::Language {
        super::Language::Json
    }

    fn mime(&self) -> super::Mime {
        Mime::Json
    }

    fn may_use_tabs(&self) -> bool {
        true
    }

    fn capabilities(&self) -> Capabilities {
        Capabilities {
            parser: ParserCapabilities { parse: Some(parse) },
            debug: DebugCapabilities {
                debug_syntax_tree: Some(debug_syntax_tree),
                debug_control_flow: None,
                debug_formatter_ir: None,
            },
            analyzer: Default::default(),
            formatter: Default::default(),
        }
    }
}

fn parse(rome_path: &RomePath, _: LanguageId, text: &str) -> AnyParse {
    let file_id = rome_path.file_id();

    let parse = rome_json_parser::parse_json(text, file_id);
    AnyParse::from(parse)
}

fn debug_syntax_tree(_rome_path: &RomePath, parse: AnyParse) -> GetSyntaxTreeResult {
    let syntax: JsonSyntaxNode = parse.syntax();
    let tree: JsonRoot = parse.tree();
    GetSyntaxTreeResult {
        cst: format!("{syntax:#?}"),
        ast: format!("{tree:#?}"),
    }
}

impl From<JsonParse> for AnyParse {
    fn from(parse: JsonParse) -> Self {
        let root = parse.syntax();
        let diagnostics = parse.into_diagnostics();

        Self {
            // SAFETY: the parser should always return a root node
            root: root.as_send().unwrap(),
            diagnostics,
        }
    }
}
