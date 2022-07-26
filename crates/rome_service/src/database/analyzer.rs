use indexmap::IndexSet;
use rome_analyze::{AnalysisFilter, RuleCategories, RuleFilter};
use rome_fs::RomePath;
use rome_js_syntax::{TextRange, TextSize};
use salsa::query_group;

use crate::{
    database::Parser,
    workspace::{FixFileResult, PullActionsResult, PullDiagnosticsResult, RenameResult},
    RomeError,
};

#[query_group(AnalyzerStorage)]
pub(crate) trait Analyzer: Parser {
    fn lint(
        &self,
        name: RomePath,
        categories: RuleCategories,
    ) -> Result<PullDiagnosticsResult, RomeError>;
    fn code_actions(
        &self,
        name: RomePath,
        range: TextRange,
    ) -> Result<PullActionsResult, RomeError>;
    fn fix_all(&self, name: RomePath) -> Result<FixFileResult, RomeError>;
    fn rename(
        &self,
        name: RomePath,
        symbol_at: TextSize,
        new_name: String,
    ) -> Result<RenameResult, RomeError>;
}

fn lint(
    db: &dyn Analyzer,
    name: RomePath,
    categories: RuleCategories,
) -> Result<PullDiagnosticsResult, RomeError> {
    let settings = db.settings(());
    let features = db.language_features(());

    let capabilities = features.get_capabilities(&name);
    let linter = capabilities
        .lint
        .ok_or_else(|| RomeError::SourceFileNotSupported(name.clone()))?;

    let parse = db.syntax(name.clone())?;

    let rules = settings.linter.rules.as_ref();
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

    filter.categories = categories;

    let mut diagnostics = db.diagnostics(name.clone())?;
    diagnostics.extend(linter(&name, parse, filter));

    Ok(PullDiagnosticsResult { diagnostics })
}

fn code_actions(
    db: &dyn Analyzer,
    name: RomePath,
    range: TextRange,
) -> Result<PullActionsResult, RomeError> {
    let features = db.language_features(());
    let settings = db.settings(());

    let capabilities = features.get_capabilities(&name);
    let code_actions = capabilities
        .code_actions
        .ok_or_else(|| RomeError::SourceFileNotSupported(name.clone()))?;

    let parse = db.syntax(name.clone())?;
    let rules = settings.linter.rules.as_ref();

    Ok(code_actions(&name, parse, range, rules))
}

fn fix_all(db: &dyn Analyzer, name: RomePath) -> Result<FixFileResult, RomeError> {
    let features = db.language_features(());
    let settings = db.settings(());

    let capabilities = features.get_capabilities(&name);
    let fix_all = capabilities
        .fix_all
        .ok_or_else(|| RomeError::SourceFileNotSupported(name.clone()))?;

    let parse = db.syntax(name.clone())?;
    let rules = settings.linter.rules.as_ref();
    Ok(fix_all(&name, parse, rules))
}

fn rename(
    db: &dyn Analyzer,
    name: RomePath,
    symbol_at: TextSize,
    new_name: String,
) -> Result<RenameResult, RomeError> {
    let features = db.language_features(());

    let capabilities = features.get_capabilities(&name);
    let rename = capabilities
        .rename
        .ok_or_else(|| RomeError::SourceFileNotSupported(name.clone()))?;

    let parse = db.syntax(name.clone())?;
    rename(&name, parse, symbol_at, new_name)
}
