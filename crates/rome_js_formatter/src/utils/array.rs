use crate::prelude::*;
use crate::AsFormat;

use rome_formatter::write;
use rome_js_syntax::{
    JsAnyArrayAssignmentPatternElement, JsAnyArrayBindingPatternElement, JsAnyArrayElement,
    JsLanguage, JsSyntaxKind,
};
use rome_rowan::{AstNode, AstSeparatedList};

/// Utility function to print array-like nodes (array expressions, array bindings and assignment patterns)
pub(crate) fn write_array_node<N, I>(node: &N, f: &mut JsFormatter) -> FormatResult<()>
where
    N: AstSeparatedList<Language = JsLanguage, Node = I>,
    for<'a> I: ArrayNodeElement + AsFormat<'a>,
{
    // Specifically do not use format_separated as arrays need separators
    // inserted after holes regardless of the formatting since this makes a
    // semantic difference

    let mut join = f.join_nodes_with_soft_line();
    let last_index = node.len().saturating_sub(1);

    for (index, element) in node.elements().enumerate() {
        let node = element.node()?;
        let separator_mode = node.separator_mode();

        let is_disallow = matches!(separator_mode, TrailingSeparatorMode::Disallow);
        let is_force = matches!(separator_mode, TrailingSeparatorMode::Force);

        join.entry(
            node.syntax(),
            &format_with(|f| {
                write!(f, [group(&node.format())])?;

                if is_disallow {
                    // Trailing separators are disallowed, replace it with an empty element
                    if let Some(separator) = element.trailing_separator()? {
                        write!(f, [format_removed(separator)])?;
                    }
                } else if is_force || index != last_index {
                    // In forced separator mode or if this element is not the last in the list, print the separator
                    match element.trailing_separator()? {
                        Some(trailing) => write!(f, [trailing.format()])?,
                        None => format_inserted(JsSyntaxKind::COMMA).fmt(f)?,
                    };
                } else if let Some(separator) = element.trailing_separator()? {
                    write!(f, [format_only_if_breaks(separator, &separator.format())])?;
                } else {
                    write!(f, [if_group_breaks(&format_inserted(JsSyntaxKind::COMMA))])?;
                };

                Ok(())
            }),
        );
    }

    join.finish()
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
