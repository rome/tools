use crate::formatter_traits::FormatTokenAndNode;
use crate::{
    empty_element, format_elements, hard_line_break, FormatElement, FormatResult, Formatter,
};
use rome_js_syntax::{AstNodeList, TsDecoratorList};

pub(crate) fn format_decorators(
    decorators: TsDecoratorList,
    formatter: &Formatter,
) -> FormatResult<FormatElement> {
    if decorators.is_empty() {
        // No line break if the list is empty
        Ok(empty_element())
    } else {
        decorators.format_with(formatter, |decorators| {
            format_elements![decorators, hard_line_break()]
        })
    }
}
