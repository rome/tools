use crate::prelude::*;

use crate::utils::FormatMemberName;
use crate::utils::JsAnyBinaryLikeExpression;
use crate::FormatNodeFields;
use rome_js_syntax::JsAnyExpression;
use rome_js_syntax::JsAnyLiteralExpression;
use rome_js_syntax::JsPropertyObjectMember;
use rome_js_syntax::JsPropertyObjectMemberFields;
use rome_rowan::TextSize;
use rome_rowan::{AstNode, SyntaxResult};

impl FormatNodeFields<JsPropertyObjectMember> for FormatNodeRule<JsPropertyObjectMember> {
    fn format_fields(
        node: &JsPropertyObjectMember,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsPropertyObjectMemberFields {
            name,
            colon_token,
            value,
        } = node.as_fields();

        let name = name?;
        let default_name_length = name.range().len();

        let (format_name, length) = FormatMemberName::from(name).format_member_name(formatter)?;
        let name_length = length.unwrap_or(default_name_length);
        let layout = property_object_member_layout(formatter, name_length, &value.clone()?)?;

        let formatted = match layout {
            PropertyObjectMemberLayout::Fluid => {
                let group_id = formatter.group_id("property_object_member");

                let value = formatted![formatter, [value.format()]]?;
                formatted![
                    formatter,
                    [
                        group_elements(format_name),
                        colon_token.format(),
                        group_elements_with_options(
                            indent(soft_line_break_or_space()),
                            GroupElementsOptions {
                                group_id: Some(group_id),
                            }
                        ),
                        line_suffix_boundary(),
                        if_group_with_id_breaks(indent(value.clone()), group_id),
                        if_group_with_id_fits_on_line(value, group_id),
                    ]
                ]
            }
            PropertyObjectMemberLayout::BreakAfterColon => {
                formatted![
                    formatter,
                    [
                        group_elements(format_name),
                        colon_token.format(),
                        space_token(),
                        group_elements(formatted![
                            formatter,
                            [indent(formatted![
                                formatter,
                                [soft_line_break_or_space(), value.format()]
                            ]?)]
                        ]?),
                    ]
                ]
            }
            PropertyObjectMemberLayout::NeverBreakAfterColon => formatted![
                formatter,
                [
                    group_elements(format_name),
                    colon_token.format(),
                    space_token(),
                    value.format(),
                ]
            ],
        };

        Ok(group_elements(formatted?))
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
    ///   ],
    ///   "function-call-key":
    ///     functionCall(
    ///         1,
    ///         2,
    ///         3,
    ///     ),
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
fn property_object_member_layout(
    formatter: &JsFormatter,
    name_len: TextSize,
    value: &JsAnyExpression,
) -> FormatResult<PropertyObjectMemberLayout> {
    let text_width_for_break = (formatter.context().tab_width() + MIN_OVERLAP_FOR_BREAK) as u32;
    let is_name_short = name_len < TextSize::from(text_width_for_break);

    if is_break_after_colon(value)? {
        return Ok(PropertyObjectMemberLayout::BreakAfterColon);
    }

    if is_name_short {
        return Ok(PropertyObjectMemberLayout::NeverBreakAfterColon);
    } else if matches!(
        value,
        JsAnyExpression::JsAnyLiteralExpression(JsAnyLiteralExpression::JsStringLiteralExpression(
            _
        ))
    ) {
        return Ok(PropertyObjectMemberLayout::BreakAfterColon);
    }

    if is_never_break_after_colon(value)? {
        return Ok(PropertyObjectMemberLayout::NeverBreakAfterColon);
    }

    Ok(PropertyObjectMemberLayout::Fluid)
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
