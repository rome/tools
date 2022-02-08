use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, group_elements, join_elements, soft_block_indent, soft_line_break_or_space,
    space_token, token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::{
    JsAnyObjectBindingPatternMember, JsObjectBindingPattern, JsObjectBindingPatternProperty,
    JsObjectBindingPatternRest, JsObjectBindingPatternShorthandProperty,
};

impl ToFormatElement for JsObjectBindingPatternShorthandProperty {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let init_node = self
            .init()
            .format_with_or_empty(formatter, |node| format_elements![space_token(), node])?;
        Ok(format_elements![
            self.identifier().format(formatter)?,
            init_node
        ])
    }
}
