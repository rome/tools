use bitflags::bitflags;

#[derive(Copy, Clone, Debug)]
pub enum RuleCategory {
    /// This rule checks the syntax according to the language specification
    /// and emits error diagnostics accordingly
    Syntax,
    /// This rule performs static analysis of the source code to detect
    /// invalid or error-prone patterns, and emits diagnostics along with
    /// proposed fixes
    Lint,
    /// This rule detects refactoring opportunities and emits code action
    /// signals
    Action,
}

bitflags! {
    pub struct RuleCategories: u8 {
        const SYNTAX = 1 << RuleCategory::Syntax as u8;
        const LINT = 1 << RuleCategory::Lint as u8;
        const ACTION = 1 << RuleCategory::Action as u8;
    }

    pub struct ActionCategory: u8 {
        const SAFE_FIX = 1 << 0;
        const SUGGESTION = 1 << 1;
        const REFACTOR = 1 << 2;
    }
}

impl Default for RuleCategories {
    fn default() -> Self {
        Self::all()
    }
}

impl From<RuleCategory> for RuleCategories {
    fn from(input: RuleCategory) -> Self {
        match input {
            RuleCategory::Syntax => RuleCategories::SYNTAX,
            RuleCategory::Lint => RuleCategories::LINT,
            RuleCategory::Action => RuleCategories::ACTION,
        }
    }
}
