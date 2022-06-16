use crate::prelude::*;
use crate::utils::{compute_expression_layout, AssignmentLikeLayout};
use crate::FormatNodeFields;
use rome_formatter::{format_args, write};
use rome_js_syntax::JsAssignmentExpressionFields;
use rome_js_syntax::{JsAnyAssignmentPattern, JsAssignmentExpression};

impl FormatNodeFields<JsAssignmentExpression> for FormatNodeRule<JsAssignmentExpression> {
    fn fmt_fields(node: &JsAssignmentExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsAssignmentExpressionFields {
            left,
            operator_token,
            right,
        } = node.as_fields();

        let right = right?;
        let left = left?;
        let layout = compute_expression_layout(f, None, &right)?;

        dbg!(&layout);
        match layout {
            AssignmentLikeLayout::Fluid => {
                let group_id = f.group_id("assignment");
                let formatted_right = right.format().memoized();
                write!(
                    f,
                    [&group_elements(&format_args![
                        &group_elements(&left.format()),
                        space_token(),
                        operator_token.format(),
                        group_elements(&if_group_breaks(&indent(&hard_line_break())))
                            .with_group_id(Some(group_id)),
                        line_suffix_boundary(),
                        group_elements(&format_args![
                            if_group_fits_on_line(&format_args![space_token(), &formatted_right]),
                            if_group_breaks(&indent(&format_args![
                                &soft_line_break(),
                                formatted_right
                            ]))
                        ])
                        .with_group_id(Some(group_id)),
                    ])]
                )
            }
            AssignmentLikeLayout::BreakAfterColon => {
                write!(
                    f,
                    [&group_elements(&format_args![
                        group_elements(&left.format()),
                        space_token(),
                        operator_token.format(),
                        group_elements(&indent(&format_args![
                            &soft_line_break_or_space(),
                            &right.format()
                        ]))
                    ])]
                )
            }
            AssignmentLikeLayout::NeverBreakAfterColon => {
                write!(
                    f,
                    [&group_elements(&format_args![
                        group_elements(&left.format()),
                        space_token(),
                        operator_token.format(),
                        space_token(),
                        &right.format(),
                    ])]
                )
            }
        }
    }
}
