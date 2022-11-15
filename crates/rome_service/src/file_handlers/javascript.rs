use crate::{
    settings::{FormatSettings, Language, LanguageSettings, LanguagesSettings, SettingsHandle},
    workspace::{
        server::AnyParse, CodeAction, FixAction, FixFileMode, FixFileResult, GetSyntaxTreeResult,
        PullActionsResult, RenameResult,
    },
    RomeError, Rules,
};
use rome_analyze::{
    AnalysisFilter, AnalyzerOptions, ControlFlow, GroupCategory, Never, QueryMatch,
    RegistryVisitor, RuleCategories, RuleCategory, RuleFilter, RuleGroup,
};
use rome_diagnostics::{v2::category, Applicability, CodeSuggestion};
use rome_formatter::{FormatError, Printed};
use rome_fs::RomePath;
use rome_js_analyze::{analyze, analyze_with_inspect_matcher, visit_registry, RuleError};
use rome_js_formatter::context::{trailing_comma::TrailingComma, QuoteProperties, QuoteStyle};
use rome_js_formatter::{context::JsFormatOptions, format_node};
use rome_js_parser::Parse;
use rome_js_semantic::{semantic_model, SemanticModelOptions};
use rome_js_syntax::{
    JsAnyRoot, JsLanguage, JsSyntaxNode, SourceType, TextRange, TextSize, TokenAtOffset,
};
use rome_rowan::{AstNode, BatchMutationExt, Direction};

use super::{
    AnalyzerCapabilities, DebugCapabilities, ExtensionHandler, FormatterCapabilities, LintParams,
    LintResults, Mime, ParserCapabilities,
};
use crate::configuration::to_analyzer_configuration;
use crate::file_handlers::{FixAllParams, Language as LanguageId};
use indexmap::IndexSet;
use rome_diagnostics::{v2, v2::Diagnostic};
use rome_js_analyze::utils::rename::{RenameError, RenameSymbolExtensions};
use std::borrow::Cow;
use std::fmt::Debug;
use tracing::debug;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct JsFormatSettings {
    pub quote_style: Option<QuoteStyle>,
    pub quote_properties: Option<QuoteProperties>,
    pub trailing_comma: Option<TrailingComma>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct JsLinterSettings {
    pub globals: Vec<String>,
}

impl Language for JsLanguage {
    type FormatSettings = JsFormatSettings;
    type FormatOptions = JsFormatOptions;
    type LinterSettings = JsLinterSettings;

    fn lookup_settings(languages: &LanguagesSettings) -> &LanguageSettings<Self> {
        &languages.javascript
    }

    fn resolve_format_options(
        global: &FormatSettings,
        language: &JsFormatSettings,
        path: &RomePath,
    ) -> JsFormatOptions {
        JsFormatOptions::new(path.as_path().try_into().unwrap_or_default())
            .with_indent_style(global.indent_style.unwrap_or_default())
            .with_line_width(global.line_width.unwrap_or_default())
            .with_quote_style(language.quote_style.unwrap_or_default())
            .with_quote_properties(language.quote_properties.unwrap_or_default())
            .with_trailing_comma(language.trailing_comma.unwrap_or_default())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct JsFileHandler;

impl ExtensionHandler for JsFileHandler {
    fn capabilities(&self) -> super::Capabilities {
        super::Capabilities {
            parser: ParserCapabilities { parse: Some(parse) },
            debug: DebugCapabilities {
                debug_syntax_tree: Some(debug_syntax_tree),
                debug_control_flow: Some(debug_control_flow),
                debug_formatter_ir: Some(debug_formatter_ir),
            },
            analyzer: AnalyzerCapabilities {
                lint: Some(lint),
                code_actions: Some(code_actions),
                fix_all: Some(fix_all),
                rename: Some(rename),
            },
            formatter: FormatterCapabilities {
                format: Some(format),
                format_range: Some(format_range),
                format_on_type: Some(format_on_type),
            },
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

fn parse(rome_path: &RomePath, language_hint: LanguageId, text: &str) -> AnyParse {
    let file_id = rome_path.file_id();

    let source_type =
        SourceType::try_from(rome_path.as_path()).unwrap_or_else(|_| match language_hint {
            LanguageId::JavaScriptReact => SourceType::jsx(),
            LanguageId::TypeScript => SourceType::ts(),
            LanguageId::TypeScriptReact => SourceType::tsx(),
            _ => SourceType::js_module(),
        });

    let parse = rome_js_parser::parse(text, file_id, source_type);
    AnyParse::from(parse)
}

impl<T> From<Parse<T>> for AnyParse
where
    T: AstNode,
    T::Language: 'static,
{
    fn from(parse: Parse<T>) -> Self {
        let root = parse.syntax();
        let diagnostics = parse.into_diagnostics();

        Self {
            // SAFETY: the parser should always return a root node
            root: root.as_send().unwrap(),
            diagnostics,
        }
    }
}

fn debug_syntax_tree(_rome_path: &RomePath, parse: AnyParse) -> GetSyntaxTreeResult {
    let syntax: JsSyntaxNode = parse.syntax();
    let tree: JsAnyRoot = parse.tree();
    GetSyntaxTreeResult {
        cst: format!("{syntax:#?}"),
        ast: format!("{tree:#?}"),
    }
}

fn debug_control_flow(rome_path: &RomePath, parse: AnyParse, cursor: TextSize) -> String {
    let mut control_flow_graph = None;

    let filter = AnalysisFilter {
        categories: RuleCategories::LINT,
        enabled_rules: Some(&[RuleFilter::Rule("js", "noDeadCode")]),
        ..AnalysisFilter::default()
    };
    let options = AnalyzerOptions::default();

    analyze_with_inspect_matcher(
        rome_path.file_id(),
        &parse.tree(),
        filter,
        |match_params| {
            let (cfg, range) = match &match_params.query {
                QueryMatch::ControlFlowGraph(cfg, node) => (cfg, node),
                _ => return,
            };

            if !range.contains(cursor) {
                return;
            }

            match &control_flow_graph {
                None => {
                    control_flow_graph = Some((cfg.to_string(), *range));
                }
                Some((_, prev_range)) => {
                    if range.len() < prev_range.len() {
                        control_flow_graph = Some((cfg.to_string(), *range));
                    }
                }
            }
        },
        &options,
        |_| ControlFlow::<Never>::Continue(()),
    );

    control_flow_graph.map(|(cfg, _)| cfg).unwrap_or_default()
}

fn debug_formatter_ir(
    rome_path: &RomePath,
    parse: AnyParse,
    settings: SettingsHandle,
) -> Result<String, RomeError> {
    let options = settings.format_options::<JsLanguage>(rome_path);

    let tree = parse.syntax();
    let formatted = format_node(options, &tree)?;

    let root_element = formatted.into_document();
    Ok(root_element.to_string())
}

fn lint(params: LintParams) -> LintResults {
    let tree = params.parse.tree();
    let mut diagnostics = params.parse.into_diagnostics();

    let file_id = params.rome_path.file_id();
    let analyzer_options = compute_analyzer_options(&params.settings);

    let mut diagnostic_count = diagnostics.len() as u64;
    let mut errors = diagnostics
        .iter()
        .filter(|diag| diag.severity() <= v2::Severity::Error)
        .count();

    let has_lint = params.filter.categories.contains(RuleCategories::LINT);

    analyze(file_id, &tree, params.filter, &analyzer_options, |signal| {
        if let Some(mut diagnostic) = signal.diagnostic() {
            // Do not report unused suppression comment diagnostics if this is a syntax-only analyzer pass
            if !has_lint && diagnostic.category() == Some(category!("suppressions/unused")) {
                return ControlFlow::<Never>::Continue(());
            }

            diagnostic_count += 1;

            // We do now check if the severity of the diagnostics should be changed.
            // The configuration allows to change the severity of the diagnostics emitted by rules.
            let severity = diagnostic
                .category()
                .filter(|category| category.name().starts_with("lint/"))
                .and_then(|category| params.rules.as_ref()?.get_severity_from_code(category))
                .unwrap_or(v2::Severity::Error);

            if severity <= v2::Severity::Error {
                errors += 1;
            }

            if diagnostic_count <= params.max_diagnostics {
                diagnostic.set_severity(severity);

                if let Some(action) = signal.action() {
                    diagnostic.add_code_suggestion(action.into());
                }

                diagnostics.push(v2::serde::Diagnostic::new(diagnostic));
            }
        }

        ControlFlow::<Never>::Continue(())
    });

    let skipped_diagnostics = diagnostic_count - diagnostics.len() as u64;

    LintResults {
        diagnostics,
        errors,
        skipped_diagnostics,
    }
}

struct ActionsVisitor<'a> {
    enabled_rules: Vec<RuleFilter<'a>>,
}

impl RegistryVisitor<JsLanguage> for ActionsVisitor<'_> {
    fn record_category<C: GroupCategory<Language = JsLanguage>>(&mut self) {
        if matches!(C::CATEGORY, RuleCategory::Action) {
            C::record_groups(self);
        }
    }

    fn record_group<G: RuleGroup<Language = JsLanguage>>(&mut self) {
        G::record_rules(self)
    }

    fn record_rule<R>(&mut self)
    where
        R: rome_analyze::Rule + 'static,
        R::Query: rome_analyze::Queryable<Language = JsLanguage>,
        <R::Query as rome_analyze::Queryable>::Output: Clone,
    {
        self.enabled_rules.push(RuleFilter::Rule(
            <R::Group as RuleGroup>::NAME,
            R::METADATA.name,
        ));
    }
}

fn code_actions(
    rome_path: &RomePath,
    parse: AnyParse,
    range: TextRange,
    rules: Option<&Rules>,
    settings: SettingsHandle,
) -> PullActionsResult {
    let tree = parse.tree();

    let mut actions = Vec::new();

    let enabled_rules: Option<Vec<RuleFilter>> = if let Some(rules) = rules {
        let enabled_rules = rules.as_enabled_rules().into_iter().collect();

        // The rules in the assist category do not have configuration entries,
        // always add them all to the enabled rules list
        let mut visitor = ActionsVisitor { enabled_rules };
        visit_registry(&mut visitor);

        Some(visitor.enabled_rules)
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

    let analyzer_options = compute_analyzer_options(&settings);

    analyze(file_id, &tree, filter, &analyzer_options, |signal| {
        if let Some(action) = signal.action() {
            actions.push(CodeAction {
                category: action.category.clone(),
                group_name: Cow::Borrowed(action.group_name),
                rule_name: Cow::Borrowed(action.rule_name),
                suggestion: CodeSuggestion::from(action),
            });
        }

        ControlFlow::<Never>::Continue(())
    });

    PullActionsResult { actions }
}

/// If applies all the safe fixes to the given syntax tree.
///
/// If `indent_style` is [Some], it means that the formatting should be applied at the end
fn fix_all(params: FixAllParams) -> Result<FixFileResult, RomeError> {
    let FixAllParams {
        rome_path,
        parse,
        rules,
        fix_file_mode,
        settings,
    } = params;

    let mut tree: JsAnyRoot = parse.tree();
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

    filter.categories = RuleCategories::SYNTAX | RuleCategories::LINT;
    let file_id = rome_path.file_id();
    let mut skipped_suggested_fixes = 0;
    let analyzer_options = compute_analyzer_options(&settings);
    loop {
        let action = analyze(file_id, &tree, filter, &analyzer_options, |signal| {
            if let Some(action) = signal.action() {
                match fix_file_mode {
                    FixFileMode::SafeFixes => {
                        if action.applicability == Applicability::MaybeIncorrect {
                            skipped_suggested_fixes += 1;
                        }
                        if action.applicability == Applicability::Always {
                            return ControlFlow::Break(action);
                        }
                    }
                    FixFileMode::SafeAndSuggestedFixes => {
                        if matches!(
                            action.applicability,
                            Applicability::Always | Applicability::MaybeIncorrect
                        ) {
                            return ControlFlow::Break(action);
                        }
                    }
                }
            }

            ControlFlow::Continue(())
        });

        match action {
            Some(action) => {
                if let Some((range, _)) = action.mutation.as_text_edits() {
                    tree = match JsAnyRoot::cast(action.mutation.commit()) {
                        Some(tree) => tree,
                        None => {
                            return Err(RomeError::RuleError(
                                RuleError::ReplacedRootWithNonRootError {
                                    rule_name: Cow::Borrowed(action.rule_name),
                                },
                            ))
                        }
                    };
                    actions.push(FixAction {
                        group_name: Cow::Borrowed(action.group_name),
                        rule_name: Cow::Borrowed(action.rule_name),
                        range,
                    });
                }
            }
            None => {
                return Ok(FixFileResult {
                    code: tree.syntax().to_string(),
                    skipped_suggested_fixes,
                    actions,
                });
            }
        }
    }
}

#[tracing::instrument(level = "debug", skip(parse))]
fn format(
    rome_path: &RomePath,
    parse: AnyParse,
    settings: SettingsHandle,
) -> Result<Printed, RomeError> {
    let options = settings.format_options::<JsLanguage>(rome_path);

    debug!("Format with the following options: \n{}", options);

    let tree = parse.syntax();
    let formatted = format_node(options, &tree)?;

    match formatted.print() {
        Ok(printed) => Ok(printed),
        Err(error) => Err(RomeError::FormatError(error.into())),
    }
}

fn format_range(
    rome_path: &RomePath,
    parse: AnyParse,
    settings: SettingsHandle,
    range: TextRange,
) -> Result<Printed, RomeError> {
    let options = settings.format_options::<JsLanguage>(rome_path);

    let tree = parse.syntax();
    let printed = rome_js_formatter::format_range(options, &tree, range)?;
    Ok(printed)
}

fn format_on_type(
    rome_path: &RomePath,
    parse: AnyParse,
    settings: SettingsHandle,
    offset: TextSize,
) -> Result<Printed, RomeError> {
    let options = settings.format_options::<JsLanguage>(rome_path);

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

    let printed = rome_js_formatter::format_sub_tree(options, &root_node)?;
    Ok(printed)
}

fn rename(
    _rome_path: &RomePath,
    parse: AnyParse,
    symbol_at: TextSize,
    new_name: String,
) -> Result<RenameResult, RomeError> {
    let root = parse.tree();
    let model = semantic_model(&root, SemanticModelOptions::default());

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

fn compute_analyzer_options(settings: &SettingsHandle) -> AnalyzerOptions {
    let configuration = to_analyzer_configuration(
        settings.as_ref().linter(),
        &settings.as_ref().languages,
        |settings| {
            if let Some(globals) = settings.javascript.globals.as_ref() {
                globals
                    .iter()
                    .map(|global| global.to_string())
                    .collect::<Vec<_>>()
            } else {
                vec![]
            }
        },
    );
    AnalyzerOptions { configuration }
}
