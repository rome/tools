use crate::prelude::*;

use crate::AsFormat;
use rome_js_syntax::{
    JsAnyArrayAssignmentPatternElement, JsAnyArrayBindingPatternElement, JsAnyArrayElement,
    JsLanguage,
};
use rome_rowan::{AstNode, AstSeparatedList};

/// Utility function to print array-like nodes (array expressions, array bindings and assignment patterns)
pub(crate) fn format_array_node<N, I>(
    node: &N,
    formatter: &JsFormatter,
) -> FormatResult<FormatElement>
where
    N: AstSeparatedList<Language = JsLanguage, Node = I>,
    for<'a> I: ArrayNodeElement + AsFormat<'a>,
{
    // Specifically do not use format_separated as arrays need separators
    // inserted after holes regardless of the formatting since this makes a
    // semantic difference
    let last_index = node.len().saturating_sub(1);
    let results = node
        .elements()
        .enumerate()
        .map(|(index, element)| {
            let node = element.node()?;
            let separator_mode = node.separator_mode();

            let is_disallow = matches!(separator_mode, TrailingSeparatorMode::Disallow);
            let is_force = matches!(separator_mode, TrailingSeparatorMode::Force);

            let formatted_element = formatted![formatter, [node.format()]]?;
            let separator = if is_disallow {
                // Trailing separators are disallowed, replace it with an empty element
                if let Some(separator) = element.trailing_separator()? {
                    formatter.format_replaced(separator, empty_element())
                } else {
                    empty_element()
                }
            } else if is_force || index != last_index {
                // In forced separator mode or if this element is not the last in the list, print the separator
                formatted![
                    formatter,
                    [&element
                        .trailing_separator()
                        .format()
                        .or_format(|| token(","))]
                ]?
            } else if let Some(separator) = element.trailing_separator()? {
                formatter.format_replaced(separator, if_group_breaks(token(",")))
            } else {
                if_group_breaks(token(","))
            };

            Ok((
                node.syntax().clone(),
                format_elements![group_elements(formatted_element), separator],
            ))
        })
        .collect::<FormatResult<Vec<_>>>()?;

    Ok(join_elements_soft_line(results))
}

/// Determines if a trailing separator should be inserted after an array element
pub(crate) enum TrailingSeparatorMode {
    /// Trailing separators are not allowed after this element (eg. rest elements)
    Disallow,
    /// Trailing separators are inserted after this element except if its the
    /// last element and the group is not breaking
    Auto,
    /// Trailing separators will always be inserted after this element (eg. hole elements)
    Force,
}

pub(crate) trait ArrayNodeElement: AstNode<Language = JsLanguage> {
    /// Determines how the trailing separator should be printer for this element
    fn separator_mode(&self) -> TrailingSeparatorMode;
}

impl ArrayNodeElement for JsAnyArrayElement {
    fn separator_mode(&self) -> TrailingSeparatorMode {
        match self {
            Self::JsArrayHole(_) => TrailingSeparatorMode::Force,
            _ => TrailingSeparatorMode::Auto,
        }
    }
}

impl ArrayNodeElement for JsAnyArrayAssignmentPatternElement {
    fn separator_mode(&self) -> TrailingSeparatorMode {
        match self {
            Self::JsArrayHole(_) => TrailingSeparatorMode::Force,
            Self::JsArrayAssignmentPatternRestElement(_) => TrailingSeparatorMode::Disallow,
            _ => TrailingSeparatorMode::Auto,
        }
    }
}

impl ArrayNodeElement for JsAnyArrayBindingPatternElement {
    fn separator_mode(&self) -> TrailingSeparatorMode {
        match self {
            Self::JsArrayHole(_) => TrailingSeparatorMode::Force,
            Self::JsArrayBindingPatternRestElement(_) => TrailingSeparatorMode::Disallow,
            _ => TrailingSeparatorMode::Auto,
        }
    }
}
