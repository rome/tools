use crate::prelude::*;
use crate::separated::FormatAstSeparatedListExtension;
use rome_formatter::write;
use rome_json_syntax::{AnyJsonValue, JsonArrayElementList};
use rome_rowan::{AstNode, AstSeparatedList};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsonArrayElementList;

impl FormatRule<JsonArrayElementList> for FormatJsonArrayElementList {
    type Context = JsonFormatContext;
    fn fmt(&self, node: &JsonArrayElementList, f: &mut JsonFormatter) -> FormatResult<()> {
        let layout = if can_concisely_print_array_list(node) {
            ArrayLayout::Fill
        } else {
            ArrayLayout::OnePerLine
        };

        match layout {
            ArrayLayout::Fill => {
                let mut filler = f.fill();

                for (element, formatted) in node.iter().zip(node.format_separated(",")) {
                    filler.entry(
                        &format_once(|f| {
                            if get_lines_before(element?.syntax()) > 1 {
                                write!(f, [empty_line()])
                            } else {
                                write!(f, [soft_line_break_or_space()])
                            }
                        }),
                        &formatted,
                    );
                }

                filler.finish()
            }

            ArrayLayout::OnePerLine => {
                let mut join = f.join_nodes_with_soft_line();

                for (element, formatted) in node.elements().zip(node.format_separated(",")) {
                    join.entry(element.node()?.syntax(), &formatted);
                }

                join.finish()
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum ArrayLayout {
    /// Tries to fit as many array elements on a single line as possible.
    ///
    /// ```json
    /// [
    ///     1, 2, 3,
    ///     5, 6,
    /// ]
    /// ```
    Fill,

    /// Prints every element on a single line if the array exceeds the line width, or any
    /// of its elements gets printed in *expanded* mode.
    /// ```json
    /// [
    ///    { "a": 3 },
    ///    4,
    ///    3,
    /// ]
    /// ```
    OnePerLine,
}

/// Returns `true` if the array can be "fill-printed" instead of breaking each element on
/// a different line.
///
/// An array can be "fill printed" if it only contains literal elements.
pub(crate) fn can_concisely_print_array_list(list: &JsonArrayElementList) -> bool {
    if list.is_empty() {
        return false;
    }

    list.iter().all(|node| {
        matches!(
            node,
            Ok(AnyJsonValue::JsonStringValue(_)
                | AnyJsonValue::JsonBooleanValue(_)
                | AnyJsonValue::JsonNullValue(_)
                | AnyJsonValue::JsonNumberValue(_))
        )
    })
}
