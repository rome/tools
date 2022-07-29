use crate::prelude::*;
use rome_formatter::{format_args, write};

use crate::utils::FormatWithSemicolon;

use rome_js_syntax::JsExportFromClause;
use rome_js_syntax::JsExportFromClauseFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsExportFromClause;

impl FormatNodeRule<JsExportFromClause> for FormatJsExportFromClause {
    fn fmt_fields(&self, node: &JsExportFromClause, f: &mut JsFormatter) -> FormatResult<()> {
        let JsExportFromClauseFields {
            star_token,
            export_as,
            from_token,
            source,
            assertion,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [FormatWithSemicolon::new(
                &format_args!(
                    star_token.format(),
                    space(),
                    export_as
                        .format()
                        .with_or_empty(|as_token, f| write![f, [as_token, space()]]),
                    from_token.format(),
                    space(),
                    source.format(),
                    assertion
                        .format()
                        .with_or_empty(|assertion, f| write![f, [space(), assertion]]),
                ),
                semicolon_token.as_ref()
            )]
        )
    }
}
