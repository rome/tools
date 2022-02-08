use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, group_elements, join_elements, soft_block_indent, soft_line_break_or_space,
    space_token, token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::{
    JsAnyConstructorParameter, JsConstructorClassMember, JsConstructorParameters,
};

impl ToFormatElement for JsConstructorClassMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.name().format(formatter)?,
            self.parameters().format(formatter)?,
            space_token(),
            self.body().format(formatter)?
        ])
    }
}
