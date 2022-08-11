//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::{AnalysisFilter, RuleRegistry};
use rome_js_syntax::JsLanguage;
pub(crate) fn build_registry(filter: &AnalysisFilter) -> RuleRegistry<JsLanguage> {
    let mut registry = RuleRegistry::default();
    registry.push_group::<crate::analyzers::Js>(filter);
    registry.push_group::<crate::analyzers::Jsx>(filter);
    registry.push_group::<crate::analyzers::Regex>(filter);
    registry.push_group::<crate::analyzers::Ts>(filter);
    registry.push_group::<crate::semantic_analyzers::Js>(filter);
    registry.push_group::<crate::assists::Js>(filter);
    registry
}
