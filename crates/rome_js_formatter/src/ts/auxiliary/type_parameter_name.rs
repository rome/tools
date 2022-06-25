use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::{TsTypeParameterName, TsTypeParameterNameFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsTypeParameterName;

impl FormatNodeRule<TsTypeParameterName> for FormatTsTypeParameterName {
    fn fmt_fields(&self, node: &TsTypeParameterName, f: &mut JsFormatter) -> FormatResult<()> {
        let TsTypeParameterNameFields { ident_token } = node.as_fields();

        write![f, [ident_token.format()]]
    }
}
