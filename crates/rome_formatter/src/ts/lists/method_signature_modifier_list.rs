use crate::utils::{sort_modifiers_by_precedence, SortedModifiers};
use crate::{join_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::{TsAnyMethodSignatureModifier, TsMethodSignatureModifierList};

impl ToFormatElement for TsMethodSignatureModifierList {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let ordered_nodes = sort_modifiers_by_precedence(self, to_sorted_modifier);

        Ok(join_elements(
            space_token(),
            formatter.format_nodes(ordered_nodes)?,
        ))
    }
}

fn to_sorted_modifier(modifier: &TsAnyMethodSignatureModifier) -> SortedModifiers {
    match modifier {
        TsAnyMethodSignatureModifier::JsStaticModifier(_) => SortedModifiers::Static,
        TsAnyMethodSignatureModifier::TsAbstractModifier(_) => SortedModifiers::Abstract,
        TsAnyMethodSignatureModifier::TsAccessibilityModifier(_) => SortedModifiers::Accessibility,
        TsAnyMethodSignatureModifier::TsOverrideModifier(_) => SortedModifiers::Override,
    }
}
