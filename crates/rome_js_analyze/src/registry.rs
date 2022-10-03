//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::{AnalysisFilter, RuleRegistry};
use rome_js_syntax::JsLanguage;
pub(crate) fn build_registry(filter: &AnalysisFilter) -> RuleRegistry<JsLanguage> {
    let mut registry = RuleRegistry::default();
    registry.push_category::<crate::analyzers::Analyzers>(filter);
    registry.push_category::<crate::semantic_analyzers::SemanticAnalyzers>(filter);
    registry.push_category::<crate::assists::Assists>(filter);
    registry
}
