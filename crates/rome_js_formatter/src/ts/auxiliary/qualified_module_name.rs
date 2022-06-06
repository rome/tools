use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::TsQualifiedModuleName;
use rome_js_syntax::TsQualifiedModuleNameFields;

impl FormatNodeFields<TsQualifiedModuleName> for FormatNodeRule<TsQualifiedModuleName> {
    fn fmt_fields(node: &TsQualifiedModuleName, f: &mut JsFormatter) -> FormatResult<()> {
        let TsQualifiedModuleNameFields {
            left,
            dot_token,
            right,
        } = node.as_fields();

        write![f, [left.format(), dot_token.format(), right.format(),]]
    }
}
