use crate::prelude::*;

use crate::utils::FormatMemberName;
use crate::utils::JsAnyBinaryLikeExpression;
use crate::FormatNodeFields;
use rome_js_syntax::JsAnyExpression;
use rome_js_syntax::JsAnyLiteralExpression;
use rome_js_syntax::JsPropertyObjectMember;
use rome_js_syntax::JsPropertyObjectMemberFields;
use rome_rowan::SyntaxResult;
use rome_rowan::TextSize;

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
        let layout = property_object_member_layout(formatter, node)?;

        let name = formatted![formatter, [FormatMemberName::from(name?)]]?;

        let formatted = match layout {
            PropertyObjectMemberLayout::Fluid => {
                let group_id = formatter.group_id("property_object_member");

                let value = formatted![formatter, [value.format()]]?;
                formatted![
                    formatter,
                    [
                        group_elements(name),
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
            PropertyObjectMemberLayout::BreakAfterOperator => {
                formatted![
                    formatter,
                    [
                        group_elements(name),
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
            PropertyObjectMemberLayout::NeverBreakAfterOperator => formatted![
                formatter,
                [
                    group_elements(name),
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
    /// First break right-hand side, then after operator
    Fluid,
    /// First break after operator, then the sides are broken independently on their own lines
    BreakAfterOperator,
    /// First break right-hand side, then left-hand side
    NeverBreakAfterOperator,
}

const MIN_OVERLAP_FOR_BREAK: u8 = 3;

fn property_object_member_layout(
    formatter: &JsFormatter,
    node: &JsPropertyObjectMember,
) -> SyntaxResult<PropertyObjectMemberLayout> {
    let JsPropertyObjectMemberFields {
        name,
        colon_token: _,
        value,
    } = node.as_fields();

    let name = name?;
    let value = value?;

    let text_width_for_break = (formatter.context().tab_width() + MIN_OVERLAP_FOR_BREAK) as u32;
    let is_name_short = name.range().len() < TextSize::from(text_width_for_break);

    if is_break_after_operator(&value)? {
        return Ok(PropertyObjectMemberLayout::BreakAfterOperator);
    }

    if is_name_short {
        return Ok(PropertyObjectMemberLayout::NeverBreakAfterOperator);
    } else {
        if matches!(
            value,
            JsAnyExpression::JsAnyLiteralExpression(
                JsAnyLiteralExpression::JsStringLiteralExpression(_)
            )
        ) {
            return Ok(PropertyObjectMemberLayout::BreakAfterOperator);
        }
    }

    if is_never_break_after_operator(&value)? {
        return Ok(PropertyObjectMemberLayout::NeverBreakAfterOperator);
    }

    Ok(PropertyObjectMemberLayout::Fluid)
}

fn is_break_after_operator(value: &JsAnyExpression) -> SyntaxResult<bool> {
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

fn is_never_break_after_operator(value: &JsAnyExpression) -> SyntaxResult<bool> {
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
