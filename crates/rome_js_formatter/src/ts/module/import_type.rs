use crate::prelude::*;
use crate::utils::FormatLiteralStringToken;
use crate::FormatNodeFields;
use rome_js_syntax::TsImportType;
use rome_js_syntax::TsImportTypeFields;

impl FormatNodeFields<TsImportType> for FormatNodeRule<TsImportType> {
    fn format_fields(node: &TsImportType, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        let TsImportTypeFields {
            typeof_token,
            import_token,
            l_paren_token,
            argument_token,
            r_paren_token,
            qualifier_clause,
            type_arguments,
        } = node.as_fields();

        formatted![
            formatter,
            [
                typeof_token
                    .format()
                    .with_or_empty(|token| formatted![formatter, [token, space_token()]]),
                import_token.format(),
                l_paren_token.format(),
                FormatLiteralStringToken::from_parent_expression(&argument_token?),
                r_paren_token.format(),
                qualifier_clause.format(),
                type_arguments.format(),
            ]
        ]
    }
}
