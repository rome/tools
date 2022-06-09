use crate::prelude::*;
use rome_formatter::{write, Buffer, VecBuffer};

use crate::utils::{FormatLiteralStringToken, StringLiteralParentKind};
use crate::FormatNodeFields;
use rome_js_syntax::JsExpressionStatement;
use rome_js_syntax::JsStringLiteralExpression;
use rome_js_syntax::JsStringLiteralExpressionFields;
use rome_rowan::AstNode;

impl FormatNodeFields<JsStringLiteralExpression> for FormatNodeRule<JsStringLiteralExpression> {
    fn fmt_fields(node: &JsStringLiteralExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsStringLiteralExpressionFields { value_token } = node.as_fields();

        let value_token = value_token?;
        let syntax_node = node.syntax();
        let parent = syntax_node.parent();

        let needs_parenthesis = parent.and_then(JsExpressionStatement::cast).is_some();

        if needs_parenthesis {
            let mut buffer = VecBuffer::new(f.state_mut());
            write!(
                buffer,
                [FormatLiteralStringToken::new(
                    &value_token,
                    StringLiteralParentKind::Expression
                )]
            )?;

            let formatted_element = buffer.into_element();

            let (leading_trivia, content, trailing_trivia) = formatted_element.split_trivia();

            write!(
                f,
                [format_once(|f| {
                    f.write_element(leading_trivia)?;
                    write!(f, [token("(")])?;
                    f.write_element(content)?;
                    write!(f, [token(")")])?;
                    f.write_element(trailing_trivia)
                })]
            )
        } else {
            write!(
                f,
                [FormatLiteralStringToken::new(
                    &value_token,
                    StringLiteralParentKind::Expression
                )]
            )
        }
    }
}
