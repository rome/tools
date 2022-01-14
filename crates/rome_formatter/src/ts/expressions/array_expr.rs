use crate::{
    empty_element, format_elements, group_elements, if_group_breaks, join_elements, soft_indent,
    soft_line_break_or_space, token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::{
    ast::{JsArrayExpression, JsArrayHole},
    AstSeparatedList,
};

impl ToFormatElement for JsArrayExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let elements = self.elements();

        Ok(group_elements(formatter.format_delimited_group(
            &self.l_brack_token()?,
            |leading, trailing| {
                // Specifically do not use format_separated as array expressions need
                // separators inserted after empty expressions regardless of the
                // formatting since this makes a semantic difference
                let last_index = elements.len().saturating_sub(1);
                let results = elements
                    .elements()
                    .enumerate()
                    .map(|(index, element)| {
                        let node = formatter.format_node(element.node()?)?;

                        let separator = if node.is_empty() || index != last_index {
                            // If the previous element was empty or this is not the last element, always print a separator
                            if let Some(separator) = element.trailing_separator()? {
                                formatter.format_token(&separator)?
                            } else {
                                token(",")
                            }
                        } else {
                            if let Some(separator) = element.trailing_separator()? {
                                formatter.format_replaced_token(
                                    &separator,
                                    if_group_breaks(token(",")),
                                )?
                            } else {
                                if_group_breaks(token(","))
                            }
                        };

                        Ok(format_elements![node, separator])
                    })
                    .collect::<FormatResult<Vec<_>>>()?;

                Ok(soft_indent(format_elements![
                    leading,
                    join_elements(soft_line_break_or_space(), results),
                    trailing,
                ]))
            },
            &self.r_brack_token()?,
        )?))
    }
}

impl ToFormatElement for JsArrayHole {
    fn to_format_element(&self, _: &Formatter) -> FormatResult<FormatElement> {
        Ok(empty_element())
    }
}
