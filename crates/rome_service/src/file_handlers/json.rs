use super::{ExtensionHandler, Mime};
use crate::file_handlers::javascript::JsonParserSettings;
use crate::file_handlers::{
    AnalyzerCapabilities, Capabilities, FixAllParams, FormatterCapabilities, LintParams,
    LintResults, ParserCapabilities,
};
use crate::file_handlers::{DebugCapabilities, Language as LanguageId};
use crate::settings::{
    FormatSettings, Language, LanguageSettings, LanguagesSettings, SettingsHandle,
};
use crate::workspace::{
    FixFileResult, GetSyntaxTreeResult, OrganizeImportsResult, PullActionsResult,
};
use crate::{Configuration, Rules, WorkspaceError};
use rome_deserialize::json::deserialize_from_json_ast;
use rome_diagnostics::{Diagnostic, Severity};
use rome_formatter::{FormatError, Printed};
use rome_fs::{RomePath, CONFIG_NAME};
use rome_json_formatter::context::JsonFormatOptions;
use rome_json_formatter::format_node;
use rome_json_parser::JsonParserOptions;
use rome_json_syntax::{JsonFileSource, JsonLanguage, JsonRoot, JsonSyntaxNode};
use rome_parser::AnyParse;
use rome_rowan::{AstNode, FileSource, NodeCache};
use rome_rowan::{TextRange, TextSize, TokenAtOffset};

impl Language for JsonLanguage {
    type FormatterSettings = ();
    type LinterSettings = ();
    type OrganizeImportsSettings = ();
    type FormatOptions = JsonFormatOptions;
    type ParserSettings = JsonParserSettings;
    fn lookup_settings(language: &LanguagesSettings) -> &LanguageSettings<Self> {
        &language.json
    }

    fn resolve_format_options(
        global: &FormatSettings,
        _language: &Self::FormatterSettings,
        _path: &RomePath,
    ) -> Self::FormatOptions {
        JsonFormatOptions::default()
            .with_indent_style(global.indent_style.unwrap_or_default())
            .with_line_width(global.line_width.unwrap_or_default())
    }
}

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
                debug_formatter_ir: Some(debug_formatter_ir),
            },
            analyzer: AnalyzerCapabilities {
                lint: Some(lint),
                code_actions: Some(code_actions),
                rename: None,
                fix_all: Some(fix_all),
                organize_imports: Some(organize_imports),
            },
            formatter: FormatterCapabilities {
                format: Some(format),
                format_range: Some(format_range),
                format_on_type: Some(format_on_type),
            },
        }
    }
}

fn parse(
    rome_path: &RomePath,
    language_hint: LanguageId,
    text: &str,
    settings: SettingsHandle,
    cache: &mut NodeCache,
) -> AnyParse {
    let parser = &settings.as_ref().languages.json.parser;
    let options: JsonParserOptions = JsonParserOptions {
        allow_comments: parser.allow_comments,
    };
    let source_type =
        JsonFileSource::try_from(rome_path.as_path()).unwrap_or_else(|_| match language_hint {
            LanguageId::Json => {
                if parser.allow_comments {
                    JsonFileSource::jsonc()
                } else {
                    JsonFileSource::json()
                }
            }
            LanguageId::Jsonc => JsonFileSource::jsonc(),
            _ => JsonFileSource::json(),
        });
    let parse = rome_json_parser::parse_json_with_cache(text, cache, options);
    let root = parse.syntax();
    let diagnostics = parse.into_diagnostics();
    AnyParse::new(
        // SAFETY: the parser should always return a root node
        root.as_send().unwrap(),
        diagnostics,
        source_type.as_any_file_source(),
    )
}

fn debug_syntax_tree(_rome_path: &RomePath, parse: AnyParse) -> GetSyntaxTreeResult {
    let syntax: JsonSyntaxNode = parse.syntax();
    let tree: JsonRoot = parse.tree();
    GetSyntaxTreeResult {
        cst: format!("{syntax:#?}"),
        ast: format!("{tree:#?}"),
    }
}

fn debug_formatter_ir(
    rome_path: &RomePath,
    parse: AnyParse,
    settings: SettingsHandle,
) -> Result<String, WorkspaceError> {
    let options = settings.format_options::<JsonLanguage>(rome_path);

    let tree = parse.syntax();
    let formatted = format_node(options, &tree)?;

    let root_element = formatted.into_document();
    Ok(root_element.to_string())
}

#[tracing::instrument(level = "debug", skip(parse))]
fn format(
    rome_path: &RomePath,
    parse: AnyParse,
    settings: SettingsHandle,
) -> Result<Printed, WorkspaceError> {
    let options = settings.format_options::<JsonLanguage>(rome_path);

    tracing::debug!("Format with the following options: \n{}", options);

    let tree = parse.syntax();
    let formatted = format_node(options, &tree)?;

    match formatted.print() {
        Ok(printed) => Ok(printed),
        Err(error) => Err(WorkspaceError::FormatError(error.into())),
    }
}

fn format_range(
    rome_path: &RomePath,
    parse: AnyParse,
    settings: SettingsHandle,
    range: TextRange,
) -> Result<Printed, WorkspaceError> {
    let options = settings.format_options::<JsonLanguage>(rome_path);

    let tree = parse.syntax();
    let printed = rome_json_formatter::format_range(options, &tree, range)?;
    Ok(printed)
}

fn format_on_type(
    rome_path: &RomePath,
    parse: AnyParse,
    settings: SettingsHandle,
    offset: TextSize,
) -> Result<Printed, WorkspaceError> {
    let options = settings.format_options::<JsonLanguage>(rome_path);

    let tree = parse.syntax();

    let range = tree.text_range();
    if offset < range.start() || offset > range.end() {
        return Err(WorkspaceError::FormatError(FormatError::RangeError {
            input: TextRange::at(offset, TextSize::from(0)),
            tree: range,
        }));
    }

    let token = match tree.token_at_offset(offset) {
        // File is empty, do nothing
        TokenAtOffset::None => panic!("empty file"),
        TokenAtOffset::Single(token) => token,
        // The cursor should be right after the closing character that was just typed,
        // select the previous token as the correct one
        TokenAtOffset::Between(token, _) => token,
    };

    let root_node = match token.parent() {
        Some(node) => node,
        None => panic!("found a token with no parent"),
    };

    let printed = rome_json_formatter::format_sub_tree(options, &root_node)?;
    Ok(printed)
}

fn lint(params: LintParams) -> LintResults {
    let root: JsonRoot = params.parse.tree();
    let mut diagnostics = params.parse.into_diagnostics();

    // if we're parsing the `rome.json` file, we deserialize it, so we can emit diagnostics for
    // malformed configuration
    if params.path.ends_with(CONFIG_NAME) {
        let deserialized = deserialize_from_json_ast::<Configuration>(&root);
        diagnostics.extend(
            deserialized
                .into_diagnostics()
                .into_iter()
                .map(rome_diagnostics::serde::Diagnostic::new)
                .collect::<Vec<_>>(),
        );
    }

    let diagnostic_count = diagnostics.len() as u64;
    let errors = diagnostics
        .iter()
        .filter(|diag| diag.severity() <= Severity::Error)
        .count();

    let skipped_diagnostics = diagnostic_count - diagnostics.len() as u64;

    LintResults {
        diagnostics,
        errors,
        skipped_diagnostics,
    }
}

fn code_actions(
    _parse: AnyParse,
    _range: TextRange,
    _rules: Option<&Rules>,
    _settings: SettingsHandle,
    _path: &RomePath,
) -> PullActionsResult {
    PullActionsResult {
        actions: Vec::new(),
    }
}

fn fix_all(params: FixAllParams) -> Result<FixFileResult, WorkspaceError> {
    let tree: JsonRoot = params.parse.tree();
    Ok(FixFileResult {
        actions: vec![],
        errors: 0,
        skipped_suggested_fixes: 0,
        code: tree.syntax().to_string(),
    })
}

fn organize_imports(parse: AnyParse) -> Result<OrganizeImportsResult, WorkspaceError> {
    Ok(OrganizeImportsResult {
        code: parse.syntax::<JsonLanguage>().to_string(),
    })
}
