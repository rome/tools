use crate::utils::{sort_modifiers_by_precedence, SortedModifiers};
use crate::{join_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::{TsAnyPropertyParameterModifier, TsPropertyParameterModifierList};

impl ToFormatElement for TsPropertyParameterModifierList {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let ordered_nodes = sort_modifiers_by_precedence(self, to_sorted_modifier);
        Ok(join_elements(
            space_token(),
            formatter.format_nodes(ordered_nodes)?,
        ))
    }
}

fn to_sorted_modifier(modifier: &TsAnyPropertyParameterModifier) -> SortedModifiers {
    match modifier {
        TsAnyPropertyParameterModifier::TsAccessibilityModifier(_) => {
            SortedModifiers::Accessibility
        }
        TsAnyPropertyParameterModifier::TsOverrideModifier(_) => SortedModifiers::Override,
        TsAnyPropertyParameterModifier::TsReadonlyModifier(_) => SortedModifiers::Readonly,
    }
}
