use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    empty_element, format_elements, group_elements, indent, join_elements,
    soft_line_break_or_space, space_token, token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};

use rslint_parser::{
    ast::{JsVariableDeclaration, JsVariableDeclarations, JsVariableStatement},
    AstSeparatedList,
};

impl ToFormatElement for JsVariableDeclarations {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let items = self.items();
        let last_index = items.len().saturating_sub(1);

        let items = items
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

        let mut items = items.into_iter();

        let leading_element = items.next();
        let trailing_elements = join_elements(soft_line_break_or_space(), items);

        Ok(format_elements![
            self.kind().format(formatter)?,
            space_token(),
            group_elements(concat_elements(leading_element.into_iter().chain(
                if !trailing_elements.is_empty() {
                    Some(indent(format_elements![
                        soft_line_break_or_space(),
                        trailing_elements
                    ]))
                } else {
                    None
                }
            ))),
        ])
    }
}
