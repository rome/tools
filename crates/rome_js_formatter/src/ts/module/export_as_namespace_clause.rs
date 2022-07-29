use crate::prelude::*;
use crate::utils::FormatWithSemicolon;

use rome_formatter::{format_args, write};
use rome_js_syntax::TsExportAsNamespaceClause;
use rome_js_syntax::TsExportAsNamespaceClauseFields;

#[derive(Debug, Clone, Default)]
pub struct FormatTsExportAsNamespaceClause;

impl FormatNodeRule<TsExportAsNamespaceClause> for FormatTsExportAsNamespaceClause {
    fn fmt_fields(
        &self,
        node: &TsExportAsNamespaceClause,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
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
                    space(),
                    namespace_token.format(),
                    space(),
                    name.format()
                ),
                semicolon_token.as_ref()
            )]
        )
    }
}
