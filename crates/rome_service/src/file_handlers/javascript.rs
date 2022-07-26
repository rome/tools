use rome_analyze::{AnalysisFilter, ControlFlow, Never, RuleCategories, RuleFilter};
use rome_diagnostics::{Applicability, CodeSuggestion, Diagnostic};
use rome_formatter::{FormatError, IndentStyle, Printed};
use rome_fs::RomePath;
use rome_js_analyze::analyze;
use rome_js_analyze::utils::rename::RenameError;
use rome_js_formatter::context::QuoteStyle;
use rome_js_formatter::{context::JsFormatContext, format_node};
use rome_js_semantic::semantic_model;
use rome_js_syntax::{JsAnyRoot, JsLanguage, SourceType, TextRange, TextSize, TokenAtOffset};
use rome_rowan::{AstNode, BatchMutationExt, Direction};

use crate::{
    database::AnyParse,
    settings::{FormatSettings, Language, LanguageSettings, LanguagesSettings, SettingsHandle},
    workspace::{CodeAction, FixAction, FixFileResult, PullActionsResult, RenameResult},
    RomeError, Rules,
};

use super::{ExtensionHandler, Mime};
use indexmap::IndexSet;
use std::borrow::Cow;
use std::fmt::Debug;

#[derive(Debug, Clone, Default)]
#[cfg_attr(
    feature = "serde_workspace",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct JsFormatSettings {
    pub quote_style: Option<QuoteStyle>,
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(
    feature = "serde_workspace",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct JsLinterSettings {
    pub globals: Vec<String>,
}

impl Language for JsLanguage {
    type FormatSettings = JsFormatSettings;
    type FormatContext = JsFormatContext;
    type LinterSettings = JsLinterSettings;

    fn lookup_settings(languages: &LanguagesSettings) -> &LanguageSettings<Self> {
        &languages.javascript
    }

    fn resolve_format_context(
        global: &FormatSettings,
        language: &JsFormatSettings,
        editor: IndentStyle,
        path: &RomePath,
    ) -> JsFormatContext {
        JsFormatContext::new(path.as_path().try_into().unwrap_or_default())
            .with_indent_style(global.indent_style.unwrap_or(editor))
            .with_line_width(global.line_width.unwrap_or_default())
            .with_quote_style(language.quote_style.unwrap_or_default())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct JsFileHandler;

impl ExtensionHandler for JsFileHandler {
    fn capabilities(&self) -> super::Capabilities {
        super::Capabilities {
            parse: Some(parse),
            debug_print: Some(debug_print),
            lint: Some(lint),
            format: Some(format),
            code_actions: Some(code_actions),
            fix_all: Some(fix_all),
            format_range: Some(format_range),
            format_on_type: Some(format_on_type),
            rename: Some(rename),
        }
    }

    fn language(&self) -> super::Language {
        super::Language::JavaScript
    }

    fn mime(&self) -> super::Mime {
        Mime::Javascript
    }

    fn may_use_tabs(&self) -> bool {
        true
    }
}

fn parse(rome_path: &RomePath, text: &str) -> (AnyParse, Vec<Diagnostic>) {
    let file_id = rome_path.file_id();

    let source_type =
        SourceType::try_from(rome_path.as_path()).unwrap_or_else(|_| SourceType::js_module());

    let parse = rome_js_parser::parse(text, file_id, source_type);
    let root = parse.syntax();
    let root = root.as_send().unwrap();

    let diagnostics = parse.into_diagnostics();
    (AnyParse::new(root), diagnostics)
}

fn debug_print(_rome_path: &RomePath, parse: AnyParse) -> String {
    let tree: JsAnyRoot = parse.tree();
    format!("{tree:#?}")
}

fn lint(rome_path: &RomePath, parse: AnyParse, filter: AnalysisFilter) -> Vec<Diagnostic> {
    let tree = parse.tree();
    let mut diagnostics = Vec::new();

    let file_id = rome_path.file_id();
    analyze(file_id, &tree, filter, |signal| {
        if let Some(mut diag) = signal.diagnostic() {
            if let Some(action) = signal.action() {
                diag.suggestions.push(action.into());
            }

            diagnostics.push(diag);
        }

        ControlFlow::<Never>::Continue(())
    });

    diagnostics
}

fn code_actions(
    rome_path: &RomePath,
    parse: AnyParse,
    range: TextRange,
    rules: Option<&Rules>,
) -> PullActionsResult {
    let tree = parse.tree();

    let mut actions = Vec::new();

    let enabled_rules: Option<Vec<RuleFilter>> = if let Some(rules) = rules {
        let enabled: IndexSet<RuleFilter> = rules.as_enabled_rules();
        Some(enabled.into_iter().collect())
    } else {
        None
    };

    let mut filter = match &enabled_rules {
        Some(rules) => AnalysisFilter::from_enabled_rules(Some(rules.as_slice())),
        _ => AnalysisFilter::default(),
    };

    filter.categories = RuleCategories::default();
    filter.range = Some(range);

    let file_id = rome_path.file_id();
    analyze(file_id, &tree, filter, |signal| {
        if let Some(action) = signal.action() {
            actions.push(CodeAction {
                category: action.category,
                rule_name: Cow::Borrowed(action.rule_name),
                suggestion: CodeSuggestion::from(action),
            });
        }

        ControlFlow::<Never>::Continue(())
    });

    PullActionsResult { actions }
}

fn fix_all(
    rome_path: &RomePath,
    parse: AnyParse,
    configuration_rules: Option<&Rules>,
) -> FixFileResult {
    let mut tree: JsAnyRoot = parse.tree();
    let mut actions = Vec::new();

    let enabled_rules: Option<Vec<RuleFilter>> = if let Some(rules) = configuration_rules {
        let enabled: IndexSet<RuleFilter> = rules.as_enabled_rules();
        Some(enabled.into_iter().collect())
    } else {
        None
    };

    let mut filter = match &enabled_rules {
        Some(rules) => AnalysisFilter::from_enabled_rules(Some(rules.as_slice())),
        _ => AnalysisFilter::default(),
    };

    filter.categories = RuleCategories::SYNTAX | RuleCategories::LINT;

    let file_id = rome_path.file_id();

    loop {
        let action = analyze(file_id, &tree, filter, |signal| {
            if let Some(action) = signal.action() {
                if action.applicability == Applicability::Always {
                    return ControlFlow::Break(action);
                }
            }

            ControlFlow::Continue(())
        });

        match action {
            Some(action) => {
                if let Some((range, _)) = action.mutation.as_text_edits() {
                    tree = action.mutation.commit();
                    actions.push(FixAction {
                        rule_name: Cow::Borrowed(action.rule_name),
                        range,
                    });
                }
            }
            None => {
                return FixFileResult {
                    code: tree.syntax().to_string(),
                    actions,
                }
            }
        }
    }
}

fn format(
    rome_path: &RomePath,
    parse: AnyParse,
    settings: SettingsHandle<IndentStyle>,
) -> Result<Printed, RomeError> {
    let context = settings.format_context::<JsLanguage>(rome_path);

    let tree = parse.syntax();
    let formatted = format_node(context, &tree)?;
    let printed = formatted.print();
    Ok(printed)
}

fn format_range(
    rome_path: &RomePath,
    parse: AnyParse,
    settings: SettingsHandle<IndentStyle>,
    range: TextRange,
) -> Result<Printed, RomeError> {
    let context = settings.format_context::<JsLanguage>(rome_path);

    let tree = parse.syntax();
    let printed = rome_js_formatter::format_range(context, &tree, range)?;
    Ok(printed)
}

fn format_on_type(
    rome_path: &RomePath,
    parse: AnyParse,
    settings: SettingsHandle<IndentStyle>,
    offset: TextSize,
) -> Result<Printed, RomeError> {
    let context = settings.format_context::<JsLanguage>(rome_path);

    let tree = parse.syntax();

    let range = tree.text_range();
    if offset < range.start() || offset > range.end() {
        return Err(RomeError::FormatError(FormatError::RangeError {
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

    let printed = rome_js_formatter::format_sub_tree(context, &root_node)?;
    Ok(printed)
}

fn rename(
    _rome_path: &RomePath,
    parse: AnyParse,
    symbol_at: TextSize,
    new_name: String,
) -> Result<RenameResult, RomeError> {
    use rome_js_analyze::utils::rename::RenameSymbolExtensions;

    let root = parse.tree();
    let model = semantic_model(&root);

    if let Some(node) = parse
        .syntax()
        .descendants_tokens(Direction::Next)
        .find(|token| token.text_range().contains(symbol_at))
        .and_then(|token| token.parent())
    {
        let original_name = node.text_trimmed();
        match node.try_into() {
            Ok(node) => {
                let mut batch = root.begin();
                let result = batch.rename_any_renamable_node(&model, node, &new_name);
                if !result {
                    Err(RomeError::RenameError(RenameError::CannotBeRenamed {
                        original_name: original_name.to_string(),
                        new_name,
                    }))
                } else {
                    let (range, indels) = batch.as_text_edits().unwrap_or_default();
                    Ok(RenameResult { range, indels })
                }
            }
            Err(err) => Err(RomeError::RenameError(err)),
        }
    } else {
        Err(RomeError::RenameError(RenameError::CannotFindDeclaration))
    }
}
