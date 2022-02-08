use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    empty_element, format_element::join_elements_soft_line, format_elements, group_elements,
    if_group_breaks, soft_block_indent, token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};

use rslint_parser::{
    ast::{JsAnyArrayElement, JsArrayExpression, JsArrayHole},
    AstSeparatedList,
};

impl ToFormatElement for JsArrayHole {
    fn to_format_element(&self, _: &Formatter) -> FormatResult<FormatElement> {
        Ok(empty_element())
    }
}
