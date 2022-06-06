use crate::generated::FormatJsxChildList;
use crate::prelude::*;
use crate::utils::jsx_utils::contains_meaningful_jsx_text;
use crate::{FormatElement, JsFormatter};
use rome_formatter::{fill_elements, FormatResult};
use rome_js_syntax::JsxChildList;

impl FormatRule<JsxChildList> for FormatJsxChildList {
    type Context = JsFormatContext;

    fn format(node: &JsxChildList, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        let mut flattened_children = Vec::new();
        // Because fill printing cannot fill print nested lists, we flatten the formatted children.
        // This is because fill printing uses print_all, so when it encounters a FormatElement::List,
        // it prints the entire list at once.
        for child in formatter.format_all(node.iter().formatted())? {
            match child {
                FormatElement::List(list) => flattened_children.extend(list.into_vec()),
                item => flattened_children.push(item),
            }
        }

        if contains_meaningful_jsx_text(node) {
            Ok(fill_elements(soft_line_break(), flattened_children))
        } else {
            Ok(join_elements(soft_line_break(), flattened_children))
        }
    }
}
