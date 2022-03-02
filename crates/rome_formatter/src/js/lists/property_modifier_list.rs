use crate::utils::{into_sorted_modifiers, SortedModifiers};
use crate::{join_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::{JsAnyPropertyModifier, JsPropertyModifierList};

impl ToFormatElement for JsPropertyModifierList {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let ordered_nodes = into_sorted_modifiers(self.clone(), to_sorted_modifier);

        Ok(join_elements(
            space_token(),
            formatter.format_nodes(ordered_nodes)?,
        ))
    }
}

fn to_sorted_modifier(modifier: &JsAnyPropertyModifier) -> SortedModifiers {
    match modifier {
        JsAnyPropertyModifier::JsStaticModifier(_) => SortedModifiers::Static,
        JsAnyPropertyModifier::TsAccessibilityModifier(_) => SortedModifiers::Accessibility,
        JsAnyPropertyModifier::TsOverrideModifier(_) => SortedModifiers::Override,
        JsAnyPropertyModifier::TsReadonlyModifier(_) => SortedModifiers::Readonly,
    }
}
