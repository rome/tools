use std::convert::Infallible;

use crate::formatter::TrailingSeparator;
use crate::{
    fill_elements,
    format_element::join_elements_soft_line,
    format_elements,
    formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode},
    if_group_breaks, token,
    utils::has_formatter_trivia,
    FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_syntax::{
    AstNode, AstSeparatedList, JsAnyArrayElement, JsAnyExpression, JsArrayElementList,
};

impl ToFormatElement for JsArrayElementList {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        if !has_formatter_trivia(self.syntax()) && can_print_fill(self) {
            return Ok(fill_elements(
                // Using format_separated is valid in this case as can_print_fill does not allow holes
                formatter.format_separated(self, || token(","), TrailingSeparator::default())?,
            ));
        }

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

/// Returns true if the provided JsArrayElementList could
/// be "fill-printed" instead of breaking each element on
/// a different line.
///
/// The underlying logic only allows lists of literal expressions
/// with 10 or less characters, potentially wrapped in a "short"
/// unary expression (+, -, ~ or !)
fn can_print_fill(list: &JsArrayElementList) -> bool {
    use rslint_syntax::JsAnyArrayElement::*;
    use rslint_syntax::JsAnyExpression::*;
    use rslint_syntax::JsUnaryOperation::*;

    list.iter().all(|item| match item {
        Ok(JsAnyExpression(JsUnaryExpression(expr))) => {
            match expr.operation() {
                Ok(Plus | Minus | BitwiseNot | LogicalNot) => {}
                _ => return false,
            }

            if let Ok(expr) = expr.argument() {
                is_short_literal(&expr)
            } else {
                false
            }
        }
        Ok(JsAnyExpression(expr)) => is_short_literal(&expr),
        _ => false,
    })
}

/// Returns true if the provided expression is a literal with 10 or less characters
fn is_short_literal(expr: &JsAnyExpression) -> bool {
    match expr {
        JsAnyExpression::JsAnyLiteralExpression(lit) => {
            let token_len = lit
                .syntax()
                .text()
                .try_fold_chunks::<_, _, Infallible>(0, |sum, chunk| {
                    // Count actual characters instead of byte length
                    Ok(sum + chunk.chars().count())
                })
                .expect("the above fold operation is infallible");

            token_len <= 10
        }
        _ => false,
    }
}
