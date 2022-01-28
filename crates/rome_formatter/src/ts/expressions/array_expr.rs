use crate::{
    empty_element, format_element::join_elements_soft_line, format_elements, group_elements,
    if_group_breaks, soft_indent, token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::{
    ast::{JsAnyArrayElement, JsArrayExpression, JsArrayHole},
    AstSeparatedList,
};

impl ToFormatElement for JsArrayExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let elements = self.elements();

        Ok(group_elements(formatter.format_delimited(
            &self.l_brack_token()?,
            |open_token_trailing, close_token_leading| {
                // Specifically do not use format_separated as array expressions need
                // separators inserted after empty expressions regardless of the
                // formatting since this makes a semantic difference
                let last_index = elements.len().saturating_sub(1);
                let results = elements
                    .elements()
                    .enumerate()
                    .map(|(index, element)| {
                        let node = element.node()?;
                        let is_hole = matches!(node, JsAnyArrayElement::JsArrayHole(_));

                        let elem = formatter.format_node(&node)?;
                        let separator = if is_hole || index != last_index {
                            // If the previous element was empty or this is not the last element, always print a separator
                            if let Some(separator) = element.trailing_separator()? {
                                formatter.format_token(&separator)?
                            } else {
                                token(",")
                            }
                        } else if let Some(separator) = element.trailing_separator()? {
                            formatter.format_replaced(&separator, if_group_breaks(token(",")))?
                        } else {
                            if_group_breaks(token(","))
                        };

                        Ok((node, format_elements![elem, separator]))
                    })
                    .collect::<FormatResult<Vec<_>>>()?;

                Ok(soft_indent(format_elements![
                    open_token_trailing,
                    join_elements_soft_line(results),
                    close_token_leading,
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
