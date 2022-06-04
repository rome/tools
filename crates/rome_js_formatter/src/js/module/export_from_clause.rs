use crate::prelude::*;
use rome_formatter::{format_args, write};

use crate::utils::FormatWithSemicolon;
use crate::FormatNodeFields;
use rome_js_syntax::JsExportFromClause;
use rome_js_syntax::JsExportFromClauseFields;

impl FormatNodeFields<JsExportFromClause> for FormatNodeRule<JsExportFromClause> {
    fn fmt_fields(node: &JsExportFromClause, f: &mut JsFormatter) -> FormatResult<()> {
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
                    space_token(),
                    export_as
                        .format()
                        .with_or_empty(|as_token, f| write![f, [as_token, space_token()]]),
                    from_token.format(),
                    space_token(),
                    source.format(),
                    assertion
                        .format()
                        .with_or_empty(|assertion, f| write![f, [space_token(), assertion]]),
                ),
                semicolon_token.as_ref()
            )]
        )
    }
}
