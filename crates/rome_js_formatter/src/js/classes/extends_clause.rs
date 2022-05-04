use crate::format_traits::{FormatOptional, FormatWith};
use crate::{format_elements, space_token, token, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::JsExtendsClauseFields;
use rome_js_syntax::{JsExtendsClause, JsSyntaxKind};
use rome_rowan::AstNode;

impl FormatNode for JsExtendsClause {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsExtendsClauseFields {
            extends_token,
            super_class,
            type_arguments,
        } = self.as_fields();

        let super_class = super_class?;
        let needs_parens = matches!(
            super_class.syntax().kind(),
            JsSyntaxKind::JS_NEW_EXPRESSION
                | JsSyntaxKind::JS_YIELD_EXPRESSION
                | JsSyntaxKind::JS_OBJECT_EXPRESSION
                | JsSyntaxKind::TS_NON_NULL_ASSERTION_EXPRESSION
        );
        let super_class = super_class.format_with(formatter, |super_class| {
            if needs_parens {
                format_elements![token("("), super_class, token(")")]
            } else {
                super_class
            }
        })?;

        Ok(format_elements![
            extends_token.format(formatter)?,
            space_token(),
            super_class,
            type_arguments.format_or_empty(formatter)?,
        ])
    }
}
