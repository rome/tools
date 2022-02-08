use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, group_elements, join_elements, soft_block_indent, soft_line_break_or_space,
    token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::{
    JsAnyArrayAssignmentPatternElement, JsArrayAssignmentPattern,
    JsArrayAssignmentPatternRestElement,
};

impl ToFormatElement for JsArrayAssignmentPatternRestElement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.dotdotdot_token().format(formatter)?,
            self.pattern().format(formatter)?
        ])
    }
}
