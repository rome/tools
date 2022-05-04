use crate::format_traits::{FormatOptional, FormatWith};
use rome_formatter::FormatResult;

use crate::{format_elements, space_token, token, Format, FormatElement, FormatNode, Formatter};

use crate::utils::needs_parenthesis;
use rome_js_syntax::JsExtendsClause;
use rome_js_syntax::JsExtendsClauseFields;
use rome_rowan::AstNode;

impl FormatNode for JsExtendsClause {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsExtendsClauseFields {
            extends_token,
            super_class,
            type_arguments,
        } = self.as_fields();

        let super_class = super_class?;
        let clause_needs_parens = needs_parenthesis(super_class.syntax());
        let super_class = super_class.format_with(formatter, |super_class| {
            if clause_needs_parens {
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
