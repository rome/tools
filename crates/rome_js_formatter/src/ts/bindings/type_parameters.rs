use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::{TsTypeParameters, TsTypeParametersFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsTypeParameters;

impl FormatNodeRule<TsTypeParameters> for FormatTsTypeParameters {
    fn fmt_fields(&self, node: &TsTypeParameters, f: &mut JsFormatter) -> FormatResult<()> {
        let TsTypeParametersFields {
            items,
            r_angle_token,
            l_angle_token,
        } = node.as_fields();

        write!(
            f,
            [
                format_delimited(&l_angle_token?, &items.format(), &r_angle_token?,)
                    .soft_block_indent()
            ]
        )
    }
}
