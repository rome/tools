use crate::prelude::*;
use crate::utils::{FormatLiteralStringToken, StringLiteralParentKind};
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::TsImportType;
use rome_js_syntax::TsImportTypeFields;

impl FormatNodeFields<TsImportType> for FormatNodeRule<TsImportType> {
    fn fmt_fields(node: &TsImportType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsImportTypeFields {
            typeof_token,
            import_token,
            l_paren_token,
            argument_token,
            r_paren_token,
            qualifier_clause,
            type_arguments,
        } = node.as_fields();

        if let Some(typeof_token) = typeof_token {
            write!(f, [typeof_token.format(), space_token()])?;
        }

        write![
            f,
            [
                import_token.format(),
                l_paren_token.format(),
                FormatLiteralStringToken::new(
                    &argument_token?,
                    StringLiteralParentKind::Expression
                ),
                r_paren_token.format(),
                qualifier_clause.format(),
                type_arguments.format(),
            ]
        ]
    }
}
