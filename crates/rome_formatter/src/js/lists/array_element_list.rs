use crate::{
    format_element::join_elements_soft_line,
    format_elements,
    formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode},
    if_group_breaks, token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::{
    ast::{JsAnyArrayElement, JsArrayElementList},
    AstSeparatedList,
};

impl ToFormatElement for JsArrayElementList {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        // Specifically do not use format_separated as array expressions need
        // separators inserted after empty expressions regardless of the
        // formatting since this makes a semantic difference
        let last_index = self.len().saturating_sub(1);
        let results = self
            .elements()
            .enumerate()
            .map(|(index, element)| {
                let node = element.node()?;
                let is_hole = matches!(node, JsAnyArrayElement::JsArrayHole(_));

                let elem = node.format(formatter)?;
                let separator = if is_hole || index != last_index {
                    // If the previous element was empty or this is not the last element, always print a separator
                    element
                        .trailing_separator()
                        .format_or(formatter, || token(","))?
                } else if let Some(separator) = element.trailing_separator()? {
                    formatter.format_replaced(&separator, if_group_breaks(token(",")))?
                } else {
                    if_group_breaks(token(","))
                };

                Ok((node, format_elements![elem, separator]))
            })
            .collect::<FormatResult<Vec<_>>>()?;

        Ok(join_elements_soft_line(results))
    }
}
