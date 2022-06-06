use crate::prelude::*;

use crate::utils::StringLiteralParentKind;
use crate::utils::{FormatLiteralStringToken, JsAnyBinaryLikeExpression};
use crate::FormatNodeFields;
use rome_formatter::{format_args, write};
use rome_js_syntax::JsAnyLiteralExpression;
use rome_js_syntax::JsPropertyObjectMember;
use rome_js_syntax::JsPropertyObjectMemberFields;
use rome_js_syntax::JsSyntaxKind::JS_STRING_LITERAL;
use rome_js_syntax::{JsAnyExpression, JsAnyObjectMemberName};
use rome_rowan::{AstNode, SyntaxResult};
use unicode_width::UnicodeWidthStr;

impl FormatNodeFields<JsPropertyObjectMember> for FormatNodeRule<JsPropertyObjectMember> {
    fn fmt_fields(node: &JsPropertyObjectMember, f: &mut JsFormatter) -> FormatResult<()> {
        let JsPropertyObjectMemberFields {
            name,
            colon_token,
            value,
        } = node.as_fields();

        let name = name?;
        let value = value?;
        let format_content = format_with(|f| {
            let name_width = write_member_name(&name, f)?;
            colon_token.format().fmt(f)?;

            let layout = property_object_member_layout(f, name_width, &value)?;
            match layout {
                PropertyObjectMemberLayout::Fluid => {
                    let group_id = f.group_id("property_object_member");

                    let value = value.format().memoized();

                    write![
                        f,
                        [
                            group_elements(&indent(&soft_line_break_or_space()),)
                                .with_group_id(Some(group_id)),
                            line_suffix_boundary(),
                            if_group_breaks(&indent(&value)).with_group_id(Some(group_id)),
                            if_group_fits_on_line(&value).with_group_id(Some(group_id)),
                        ]
                    ]
                }
                PropertyObjectMemberLayout::BreakAfterColon => {
                    write![
                        f,
                        [
                            space_token(),
                            group_elements(&indent(&format_args![
                                soft_line_break_or_space(),
                                value.format()
                            ])),
                        ]
                    ]
                }
                PropertyObjectMemberLayout::NeverBreakAfterColon => {
                    write![f, [space_token(), value.format(),]]
                }
            }
        });

        write!(f, [group_elements(&format_content)])
    }
}

fn write_member_name(name: &JsAnyObjectMemberName, f: &mut JsFormatter) -> FormatResult<usize> {
    match name {
        name @ JsAnyObjectMemberName::JsLiteralMemberName(literal) => {
            let value = literal.value()?;

            if value.kind() == JS_STRING_LITERAL {
                let format = FormatLiteralStringToken::new(&value, StringLiteralParentKind::Member);
                let cleaned = format.clean_text(f.context());

                cleaned.fmt(f)?;

                Ok(cleaned.width())
            } else {
                name.format().fmt(f)?;

                Ok(value.text_trimmed().width())
            }
        }
        name => {
            write!(f, [group_elements(&name.format())])?;
            Ok(name.text().width())
        }
    }
}

/// Determines how a property object member should be formatted
enum PropertyObjectMemberLayout {
    /// First break right-hand side, then after operator.
    /// ```js
    /// {
    ///   "array-key": [
    ///     {
    ///       "nested-key-1": 1,
    ///       "nested-key-2": 2,
    ///     },
    ///   ]
    /// }
    /// ```
    Fluid,
    /// First break after operator, then the sides are broken independently on their own lines.
    /// There is a soft line break after colon token.
    /// ```js
    /// {
    ///     "enough-long-key-to-break-line":
    ///         1 + 2,
    ///     "not-long-enough-key":
    ///         "but long enough string to break line",
    /// }
    /// ```
    BreakAfterColon,
    /// First break right-hand side, then left-hand side. There are not any soft line breaks
    /// between property name and property value
    /// ```js
    /// {
    ///     key1: "123",
    ///     key2: 123,
    ///     key3: class MyClass {
    ///        constructor() {},
    ///     },
    /// }
    /// ```
    NeverBreakAfterColon,
}

const MIN_OVERLAP_FOR_BREAK: u8 = 3;

/// Returns the layout variant for an object member depending on value expression and name length
/// [Prettier applies]: https://github.com/prettier/prettier/blob/main/src/language-js/print/assignment.js
fn property_object_member_layout(
    formatter: &JsFormatter,
    name_width: usize,
    value: &JsAnyExpression,
) -> FormatResult<PropertyObjectMemberLayout> {
    let text_width_for_break = (formatter.context().tab_width() + MIN_OVERLAP_FOR_BREAK) as usize;
    let is_name_short = name_width < text_width_for_break;

    if is_break_after_colon(value)? {
        Ok(PropertyObjectMemberLayout::BreakAfterColon)
    } else if is_name_short {
        Ok(PropertyObjectMemberLayout::NeverBreakAfterColon)
    } else if matches!(
        value,
        JsAnyExpression::JsAnyLiteralExpression(JsAnyLiteralExpression::JsStringLiteralExpression(
            _
        ))
    ) {
        Ok(PropertyObjectMemberLayout::BreakAfterColon)
    } else if is_never_break_after_colon(value)? {
        Ok(PropertyObjectMemberLayout::NeverBreakAfterColon)
    } else {
        Ok(PropertyObjectMemberLayout::Fluid)
    }
}

fn is_break_after_colon(value: &JsAnyExpression) -> SyntaxResult<bool> {
    if JsAnyBinaryLikeExpression::can_cast(value.syntax().kind()) {
        return Ok(true);
    }

    if matches!(value, JsAnyExpression::JsSequenceExpression(_)) {
        return Ok(true);
    }

    if let JsAnyExpression::JsConditionalExpression(conditional) = &value {
        if JsAnyBinaryLikeExpression::can_cast(conditional.test()?.syntax().kind()) {
            return Ok(true);
        }
    }

    Ok(false)
}

fn is_never_break_after_colon(value: &JsAnyExpression) -> SyntaxResult<bool> {
    if let JsAnyExpression::JsCallExpression(call_expression) = &value {
        if call_expression.callee()?.syntax().text() == "require" {
            return Ok(true);
        }
    }

    if matches!(
        value,
        JsAnyExpression::JsClassExpression(_)
            | JsAnyExpression::JsTemplate(_)
            | JsAnyExpression::JsAnyLiteralExpression(
                JsAnyLiteralExpression::JsBooleanLiteralExpression(_),
            )
            | JsAnyExpression::JsAnyLiteralExpression(
                JsAnyLiteralExpression::JsNumberLiteralExpression(_)
            )
    ) {
        return Ok(true);
    }

    Ok(false)
}
