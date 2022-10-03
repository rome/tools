//! Generated file, do not edit by hand, see `xtask/codegen`

mod correctness;
pub(super) use self::correctness::Correctness;
mod nursery;
pub(super) use self::nursery::Nursery;
mod style;
pub(super) use self::style::Style;
#[doc = r" The ID of this rule category, used in child modules as `super::CATEGORY`"]
pub(self) const CATEGORY: rome_analyze::RuleCategory = rome_analyze::RuleCategory::Lint;
