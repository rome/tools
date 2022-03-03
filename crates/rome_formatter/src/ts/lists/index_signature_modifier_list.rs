use crate::utils::{sort_modifiers_by_precedence, SortedModifiers};
use crate::{join_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::{TsAnyIndexSignatureModifier, TsIndexSignatureModifierList};

impl ToFormatElement for TsIndexSignatureModifierList {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let ordered_nodes = sort_modifiers_by_precedence(self, to_sorted_modifier);

        Ok(join_elements(
            space_token(),
            formatter.format_nodes(ordered_nodes)?,
        ))
    }
}

fn to_sorted_modifier(modifier: &TsAnyIndexSignatureModifier) -> SortedModifiers {
    match modifier {
        TsAnyIndexSignatureModifier::JsStaticModifier(_) => SortedModifiers::Static,
        TsAnyIndexSignatureModifier::TsReadonlyModifier(_) => SortedModifiers::Readonly,
    }
}
