use crate::prelude::*;
use crate::soft_block_indent;
use crate::utils::jsx::is_jsx_inside_arrow_function_inside_call_inside_expression_child;
use rome_formatter::{format_args, write, FormatResult};
use rome_js_syntax::{JsxElement, JsxElementFields};

#[derive(Debug, Clone, Default)]
pub struct FormatJsxElement;

impl FormatNodeRule<JsxElement> for FormatJsxElement {
    fn fmt_fields(&self, node: &JsxElement, formatter: &mut JsFormatter) -> FormatResult<()> {
        let JsxElementFields {
            opening_element,
            children,
            closing_element,
        } = node.as_fields();

        let expand_if_special_case =
            is_jsx_inside_arrow_function_inside_call_inside_expression_child(node.syntax())
                .then(expand_parent);

        write![
            formatter,
            [group(&format_args![
                opening_element.format(),
                expand_if_special_case,
                soft_block_indent(&children.format()),
                closing_element.format()
            ])]
        ]
    }
}
