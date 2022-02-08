use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsObjectAssignmentPatternShorthandProperty;

impl ToFormatElement for JsObjectAssignmentPatternShorthandProperty {
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
