use crate::utils::{into_sorted_modifiers, SortedModifiers};
use crate::{join_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::{TsAnyPropertyParameterModifier, TsPropertyParameterModifierList};

impl ToFormatElement for TsPropertyParameterModifierList {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let ordered_nodes = into_sorted_modifiers(self.clone(), to_sorted_modifier);
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
