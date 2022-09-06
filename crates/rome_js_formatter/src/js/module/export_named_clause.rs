use crate::prelude::*;
use rome_formatter::write;

use crate::utils::FormatWithSemicolon;

use crate::builders::format_delimited;
use rome_js_syntax::JsExportNamedClause;
use rome_js_syntax::JsExportNamedClauseFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsExportNamedClause;

impl FormatNodeRule<JsExportNamedClause> for FormatJsExportNamedClause {
    fn fmt_fields(&self, node: &JsExportNamedClause, f: &mut JsFormatter) -> FormatResult<()> {
        let JsExportNamedClauseFields {
            type_token,
            l_curly_token,
            specifiers,
            r_curly_token,
            semicolon_token,
        } = node.as_fields();

        let content = format_with(move |f| {
            if let Some(type_token) = &type_token {
                write!(f, [type_token.format(), space()])?;
            }

            write!(
                f,
                [format_delimited(
                    l_curly_token.as_ref()?,
                    &specifiers.format(),
                    r_curly_token.as_ref()?
                )
                .soft_block_spaces()]
            )
        });

        write!(
            f,
            [FormatWithSemicolon::new(&content, semicolon_token.as_ref())]
        )
    }
}
