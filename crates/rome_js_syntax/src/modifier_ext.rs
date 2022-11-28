use crate::{
    AnyJsMethodModifier, AnyJsPropertyModifier, AnyTsIndexSignatureModifier,
    AnyTsMethodSignatureModifier, AnyTsPropertyParameterModifier, AnyTsPropertySignatureModifier,
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

impl From<&AnyTsIndexSignatureModifier> for Modifiers {
    fn from(modifier: &AnyTsIndexSignatureModifier) -> Self {
        match modifier {
            AnyTsIndexSignatureModifier::JsStaticModifier(_) => Modifiers::Static,
            AnyTsIndexSignatureModifier::TsReadonlyModifier(_) => Modifiers::Readonly,
        }
    }
}

impl From<&AnyJsMethodModifier> for Modifiers {
    fn from(modifier: &AnyJsMethodModifier) -> Self {
        match modifier {
            AnyJsMethodModifier::JsStaticModifier(_) => Modifiers::Static,
            AnyJsMethodModifier::TsAccessibilityModifier(_) => Modifiers::Accessibility,
            AnyJsMethodModifier::TsOverrideModifier(_) => Modifiers::Override,
        }
    }
}

impl From<&AnyTsMethodSignatureModifier> for Modifiers {
    fn from(modifier: &AnyTsMethodSignatureModifier) -> Self {
        match modifier {
            AnyTsMethodSignatureModifier::JsStaticModifier(_) => Modifiers::Static,
            AnyTsMethodSignatureModifier::TsAbstractModifier(_) => Modifiers::Abstract,
            AnyTsMethodSignatureModifier::TsAccessibilityModifier(_) => Modifiers::Accessibility,
            AnyTsMethodSignatureModifier::TsOverrideModifier(_) => Modifiers::Override,
        }
    }
}

impl From<&AnyJsPropertyModifier> for Modifiers {
    fn from(modifier: &AnyJsPropertyModifier) -> Self {
        match modifier {
            AnyJsPropertyModifier::JsStaticModifier(_) => Modifiers::Static,
            AnyJsPropertyModifier::TsAccessibilityModifier(_) => Modifiers::Accessibility,
            AnyJsPropertyModifier::TsOverrideModifier(_) => Modifiers::Override,
            AnyJsPropertyModifier::TsReadonlyModifier(_) => Modifiers::Readonly,
        }
    }
}

impl From<&AnyTsPropertyParameterModifier> for Modifiers {
    fn from(modifier: &AnyTsPropertyParameterModifier) -> Self {
        match modifier {
            AnyTsPropertyParameterModifier::TsAccessibilityModifier(_) => Modifiers::Accessibility,
            AnyTsPropertyParameterModifier::TsOverrideModifier(_) => Modifiers::Override,
            AnyTsPropertyParameterModifier::TsReadonlyModifier(_) => Modifiers::Readonly,
        }
    }
}

impl From<&AnyTsPropertySignatureModifier> for Modifiers {
    fn from(modifier: &AnyTsPropertySignatureModifier) -> Self {
        match modifier {
            AnyTsPropertySignatureModifier::TsAccessibilityModifier(_) => Modifiers::Accessibility,
            AnyTsPropertySignatureModifier::TsDeclareModifier(_) => Modifiers::Declare,
            AnyTsPropertySignatureModifier::JsStaticModifier(_) => Modifiers::Static,
            AnyTsPropertySignatureModifier::TsAbstractModifier(_) => Modifiers::Abstract,
            AnyTsPropertySignatureModifier::TsOverrideModifier(_) => Modifiers::Override,
            AnyTsPropertySignatureModifier::TsReadonlyModifier(_) => Modifiers::Readonly,
        }
    }
}
