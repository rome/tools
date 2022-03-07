use crate::{
    concat_elements, empty_element, format_elements,
    formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode},
    group_elements, indent, join_elements, soft_line_break_or_space, token, FormatElement,
    FormatResult, Formatter, ToFormatElement,
};
use rome_js_syntax::{AstSeparatedList, JsVariableDeclaratorList};

impl ToFormatElement for JsVariableDeclaratorList {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let last_index = self.len().saturating_sub(1);

        let declarators = self
            .elements()
            .enumerate()
            .map(|(index, element)| {
                let node = element.node().format(formatter)?;
                let separator = element.trailing_separator().format_with_or(
                    formatter,
                    |separator| {
                        if index == last_index {
                            Ok(empty_element())
                        } else {
                            Ok(separator)
                        }
                    },
                    || {
                        if index == last_index {
                            Ok(empty_element())
                        } else {
                            Ok(token(","))
                        }
                    },
                )?;

                Ok(format_elements![node, separator])
            })
            .collect::<FormatResult<Vec<_>>>()?;

        let mut items = declarators.into_iter();

        let leading_element = items.next();
        let trailing_elements = join_elements(soft_line_break_or_space(), items);

        Ok(group_elements(concat_elements(
            leading_element
                .into_iter()
                .chain(if !trailing_elements.is_empty() {
                    Some(indent(format_elements![
                        soft_line_break_or_space(),
                        trailing_elements
                    ]))
                } else {
                    None
                }),
        )))
    }
}
