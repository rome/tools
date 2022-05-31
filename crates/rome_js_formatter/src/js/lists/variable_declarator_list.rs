use crate::prelude::*;

use crate::generated::FormatJsVariableDeclaratorList;
use crate::AsFormat;
use rome_js_syntax::JsVariableDeclaratorList;
use rome_rowan::AstSeparatedList;

impl FormatRule<JsVariableDeclaratorList> for FormatJsVariableDeclaratorList {
    type Context = JsFormatContext;

    fn format(
        node: &JsVariableDeclaratorList,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let last_index = node.len().saturating_sub(1);

        let declarators = node
            .elements()
            .enumerate()
            .map(|(index, element)| {
                let node = formatted![formatter, [element.node()?.format()]]?;
                let separator = match element.trailing_separator()? {
                    None => {
                        if index == last_index {
                            empty_element()
                        } else {
                            token(",")
                        }
                    }
                    Some(separator) => {
                        if index == last_index {
                            empty_element()
                        } else {
                            formatted![formatter, [separator.format()]]?
                        }
                    }
                };

                Ok(format_elements![group_elements(node), separator])
            })
            .collect::<FormatResult<Vec<_>>>()?;

        let mut items = declarators.into_iter();

        let leading_element = items.next();
        let trailing_elements = join_elements(soft_line_break_or_space(), items);

        Ok(group_elements(concat_elements(
            leading_element
                .into_iter()
                .chain(if !trailing_elements.is_empty() {
                    Some(indent(formatted![
                        formatter,
                        [soft_line_break_or_space(), trailing_elements]
                    ]?))
                } else {
                    None
                }),
        )))
    }
}
