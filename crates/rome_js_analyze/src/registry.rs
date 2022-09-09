//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::{AnalysisFilter, RuleRegistry};
use rome_js_syntax::JsLanguage;
pub(crate) fn build_registry(filter: &AnalysisFilter) -> RuleRegistry<JsLanguage> {
    let mut registry = RuleRegistry::default();
    registry.push_group::<crate::analyzers::Correctness>(filter);
    registry.push_group::<crate::analyzers::Nursery>(filter);
    registry.push_group::<crate::analyzers::Style>(filter);
    registry.push_group::<crate::semantic_analyzers::Correctness>(filter);
    registry.push_group::<crate::semantic_analyzers::Nursery>(filter);
    registry.push_group::<crate::semantic_analyzers::Style>(filter);
    registry.push_group::<crate::assists::Refactor>(filter);
    registry
}
