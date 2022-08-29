use crate::prelude::*;
use rome_formatter::{write, FormatRuleWithOptions, GroupId};

use crate::utils::array::write_array_node;

use crate::utils::{has_token_trailing_line_comment, has_trailing_line_comment};
use rome_js_syntax::{JsArrayElementList, JsSyntaxKind};
use rome_rowan::{AstNode, AstSeparatedList};

#[derive(Debug, Clone, Default)]
pub struct FormatJsArrayElementList {
    group_id: Option<GroupId>,
}

impl FormatRuleWithOptions<JsArrayElementList> for FormatJsArrayElementList {
    type Options = Option<GroupId>;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.group_id = options;
        self
    }
}

impl FormatRule<JsArrayElementList> for FormatJsArrayElementList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &JsArrayElementList, f: &mut JsFormatter) -> FormatResult<()> {
        let layout = if can_print_fill(node) {
            ArrayLayout::Fill
        } else {
            ArrayLayout::OnePerLine
        };

        match layout {
            ArrayLayout::Fill => {
                let mut filler = f.fill();

                // Using format_separated is valid in this case as can_print_fill does not allow holes
                for (element, formatted) in node.iter().zip(
                    node.format_separated(JsSyntaxKind::COMMA)
                        .with_group_id(self.group_id),
                ) {
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
            ArrayLayout::OnePerLine => write_array_node(node, f),
        }
    }
}

#[derive(Copy, Clone)]
enum ArrayLayout {
    /// Tries to fit as many array elements on a single line as possible.
    ///
    /// ```javascript
    /// [
    ///     1, 2, 3,
    ///     5, 6,
    /// ]
    /// ```
    Fill,

    /// Prints every element on a single line if the whole array expression exceeds the line width
    /// ```javascript
    /// [
    ///     a.b(),
    ///     4,
    ///     3,
    /// ]
    /// ```
    OnePerLine,
}

/// Returns true if the provided JsArrayElementList could
/// be "fill-printed" instead of breaking each element on
/// a different line.
///
/// The underlying logic only allows lists of literal expressions
/// with 10 or less characters, potentially wrapped in a "short"
/// unary expression (+, -, ~ or !)
fn can_print_fill(list: &JsArrayElementList) -> bool {
    use rome_js_syntax::JsAnyArrayElement::*;
    use rome_js_syntax::JsAnyExpression::*;
    use rome_js_syntax::JsUnaryOperator::*;

    if list.is_empty() {
        return false;
    }

    list.elements().all(|item| {
        let separator_has_trailing = item.trailing_separator().map_or(true, |separator| {
            separator.map_or(false, |separator| {
                has_token_trailing_line_comment(separator)
            })
        });

        if separator_has_trailing {
            return false;
        }

        let syntax = match item.into_node() {
            Ok(JsAnyExpression(JsAnyLiteralExpression(
                rome_js_syntax::JsAnyLiteralExpression::JsNumberLiteralExpression(literal),
            ))) => literal.into_syntax(),

            Ok(JsAnyExpression(JsUnaryExpression(expr))) => {
                let signed = matches!(expr.operator(), Ok(Plus | Minus));
                let argument = expr.argument();

                match argument {
                    Ok(JsAnyLiteralExpression(
                        rome_js_syntax::JsAnyLiteralExpression::JsNumberLiteralExpression(literal),
                    )) => {
                        let has_operator_comments = expr
                            .operator_token()
                            .map_or(false, |operator| operator.has_trailing_comments());

                        if signed
                            && !literal.syntax().has_leading_comments()
                            && !has_operator_comments
                        {
                            literal.into_syntax()
                        } else {
                            return false;
                        }
                    }
                    _ => {
                        return false;
                    }
                }
            }

            _ => {
                return false;
            }
        };

        !has_trailing_line_comment(&syntax)
    })
}
