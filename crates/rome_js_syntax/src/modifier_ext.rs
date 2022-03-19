use crate::{
    JsAnyMethodModifier, JsAnyPropertyModifier, TsAnyIndexSignatureModifier,
    TsAnyMethodSignatureModifier, TsAnyPropertyParameterModifier, TsAnyPropertySignatureModifier,
};

/// Helpful data structure to make the order modifiers predictable inside the formatter
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Modifiers {
    // modifiers must be sorted by precedence.
    Accessibility,
    Declare,
    Static,
    Abstract,
    Override,
    Readonly,
}

impl From<&TsAnyIndexSignatureModifier> for Modifiers {
    fn from(modifier: &TsAnyIndexSignatureModifier) -> Self {
        match modifier {
            TsAnyIndexSignatureModifier::JsStaticModifier(_) => Modifiers::Static,
            TsAnyIndexSignatureModifier::TsReadonlyModifier(_) => Modifiers::Readonly,
        }
    }
}

impl From<&JsAnyMethodModifier> for Modifiers {
    fn from(modifier: &JsAnyMethodModifier) -> Self {
        match modifier {
            JsAnyMethodModifier::JsStaticModifier(_) => Modifiers::Static,
            JsAnyMethodModifier::TsAccessibilityModifier(_) => Modifiers::Accessibility,
            JsAnyMethodModifier::TsOverrideModifier(_) => Modifiers::Override,
        }
    }
}

impl From<&TsAnyMethodSignatureModifier> for Modifiers {
    fn from(modifier: &TsAnyMethodSignatureModifier) -> Self {
        match modifier {
            TsAnyMethodSignatureModifier::JsStaticModifier(_) => Modifiers::Static,
            TsAnyMethodSignatureModifier::TsAbstractModifier(_) => Modifiers::Abstract,
            TsAnyMethodSignatureModifier::TsAccessibilityModifier(_) => Modifiers::Accessibility,
            TsAnyMethodSignatureModifier::TsOverrideModifier(_) => Modifiers::Override,
        }
    }
}

impl From<&JsAnyPropertyModifier> for Modifiers {
    fn from(modifier: &JsAnyPropertyModifier) -> Self {
        match modifier {
            JsAnyPropertyModifier::JsStaticModifier(_) => Modifiers::Static,
            JsAnyPropertyModifier::TsAccessibilityModifier(_) => Modifiers::Accessibility,
            JsAnyPropertyModifier::TsOverrideModifier(_) => Modifiers::Override,
            JsAnyPropertyModifier::TsReadonlyModifier(_) => Modifiers::Readonly,
        }
    }
}

impl From<&TsAnyPropertyParameterModifier> for Modifiers {
    fn from(modifier: &TsAnyPropertyParameterModifier) -> Self {
        match modifier {
            TsAnyPropertyParameterModifier::TsAccessibilityModifier(_) => Modifiers::Accessibility,
            TsAnyPropertyParameterModifier::TsOverrideModifier(_) => Modifiers::Override,
            TsAnyPropertyParameterModifier::TsReadonlyModifier(_) => Modifiers::Readonly,
        }
    }
}

impl From<&TsAnyPropertySignatureModifier> for Modifiers {
    fn from(modifier: &TsAnyPropertySignatureModifier) -> Self {
        match modifier {
            TsAnyPropertySignatureModifier::TsAccessibilityModifier(_) => Modifiers::Accessibility,
            TsAnyPropertySignatureModifier::TsDeclareModifier(_) => Modifiers::Declare,
            TsAnyPropertySignatureModifier::JsStaticModifier(_) => Modifiers::Static,
            TsAnyPropertySignatureModifier::TsAbstractModifier(_) => Modifiers::Abstract,
            TsAnyPropertySignatureModifier::TsOverrideModifier(_) => Modifiers::Override,
            TsAnyPropertySignatureModifier::TsReadonlyModifier(_) => Modifiers::Readonly,
        }
    }
}
