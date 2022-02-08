use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, group_elements, join_elements, soft_block_indent, soft_line_break_or_space,
    space_token, token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::{
    JsAnyObjectAssignmentPatternMember, JsObjectAssignmentPattern,
    JsObjectAssignmentPatternProperty, JsObjectAssignmentPatternRest,
    JsObjectAssignmentPatternShorthandProperty,
};

impl ToFormatElement for JsObjectAssignmentPatternRest {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.dotdotdot_token().format(formatter)?,
            self.target().format(formatter)?,
        ])
    }
}
