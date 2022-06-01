use rome_formatter::{write, Buffer, VecBuffer};
use rome_js_syntax::JsSyntaxKind;
use rome_rowan::AstNode;

use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsStaticMemberExpression;
use rome_js_syntax::JsStaticMemberExpressionFields;

impl FormatNodeFields<JsStaticMemberExpression> for FormatNodeRule<JsStaticMemberExpression> {
    fn format_fields(node: &JsStaticMemberExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsStaticMemberExpressionFields {
            object,
            operator_token,
            member,
        } = node.as_fields();

        let content = format_with(|f| {
            let object_syntax = object.clone()?.syntax().clone();

            let is_object_number_literal =
                object_syntax.kind() == JsSyntaxKind::JS_NUMBER_LITERAL_EXPRESSION;

            let has_object_trailing_trivia =
                object_syntax.last_trailing_trivia().unwrap().pieces().len() > 0;
            let has_operator_leading_trivia =
                operator_token.clone()?.leading_trivia().pieces().len() > 0;

            if is_object_number_literal
                && (has_object_trailing_trivia || has_operator_leading_trivia)
            {
                let mut buffer = VecBuffer::new(f.state_mut());
                write![buffer, [object.format()]]?;
                let formatted_object = buffer.into_element();

                let (object_leading, object_content, object_trailing) =
                    formatted_object.split_trivia();

                f.write_element(object_leading)?;
                write!(f, [token("(")])?;
                f.write_element(object_content)?;
                write!(f, [token(")")])?;
                f.write_element(object_trailing)?;

                write!(f, [operator_token.format(), member.format()])
            } else {
                write![
                    f,
                    [object.format(), operator_token.format(), member.format()]
                ]
            }
        });

        write!(f, [group_elements(content)])
    }
}
