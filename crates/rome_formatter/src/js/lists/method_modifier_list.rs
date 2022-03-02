use crate::utils::{into_sorted_modifiers, SortedModifiers};
use crate::{join_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::{JsAnyMethodModifier, JsMethodModifierList};

impl ToFormatElement for JsMethodModifierList {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let ordered_nodes = into_sorted_modifiers(self.clone(), to_sorted_modifier);
        Ok(join_elements(
            space_token(),
            formatter.format_nodes(ordered_nodes)?,
        ))
    }
}

fn to_sorted_modifier(modifier: &JsAnyMethodModifier) -> SortedModifiers {
    match modifier {
        JsAnyMethodModifier::JsStaticModifier(_) => SortedModifiers::Static,
        JsAnyMethodModifier::TsAccessibilityModifier(_) => SortedModifiers::Accessibility,
        JsAnyMethodModifier::TsOverrideModifier(_) => SortedModifiers::Override,
    }
}
