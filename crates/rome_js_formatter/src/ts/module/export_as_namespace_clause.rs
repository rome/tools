use crate::prelude::*;
use crate::utils::FormatWithSemicolon;
use crate::FormatNodeFields;
use rome_formatter::{format_args, write};
use rome_js_syntax::TsExportAsNamespaceClause;
use rome_js_syntax::TsExportAsNamespaceClauseFields;

impl FormatNodeFields<TsExportAsNamespaceClause> for FormatNodeRule<TsExportAsNamespaceClause> {
    fn fmt_fields(node: &TsExportAsNamespaceClause, f: &mut JsFormatter) -> FormatResult<()> {
        let TsExportAsNamespaceClauseFields {
            as_token,
            namespace_token,
            name,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [FormatWithSemicolon::new(
                &format_args!(
                    as_token.format(),
                    space_token(),
                    namespace_token.format(),
                    space_token(),
                    name.format()
                ),
                semicolon_token.as_ref()
            )]
        )
    }
}
