use crate::utils::{into_sorted_modifiers, SortedModifiers};
use crate::{join_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::{TsAnyPropertySignatureModifier, TsPropertySignatureModifierList};

impl ToFormatElement for TsPropertySignatureModifierList {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let ordered_nodes = into_sorted_modifiers(self.clone(), to_sorted_modifier);

        Ok(join_elements(
            space_token(),
            formatter.format_nodes(ordered_nodes)?,
        ))
    }
}

fn to_sorted_modifier(modifier: &TsAnyPropertySignatureModifier) -> SortedModifiers {
    match modifier {
        TsAnyPropertySignatureModifier::TsAccessibilityModifier(_) => {
            SortedModifiers::Accessibility
        }
        TsAnyPropertySignatureModifier::TsDeclareModifier(_) => SortedModifiers::Declare,
        TsAnyPropertySignatureModifier::JsStaticModifier(_) => SortedModifiers::Static,
        TsAnyPropertySignatureModifier::TsAbstractModifier(_) => SortedModifiers::Abstract,
        TsAnyPropertySignatureModifier::TsOverrideModifier(_) => SortedModifiers::Override,
        TsAnyPropertySignatureModifier::TsReadonlyModifier(_) => SortedModifiers::Readonly,
    }
}
