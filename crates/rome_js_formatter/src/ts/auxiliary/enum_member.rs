use crate::prelude::*;
use crate::utils::FormatInitializerClause;
use crate::FormatNodeFields;
use rome_formatter::{format_args, write};
use rome_js_syntax::{TsEnumMember, TsEnumMemberFields};

impl FormatNodeFields<TsEnumMember> for FormatNodeRule<TsEnumMember> {
    fn format_fields(node: &TsEnumMember, f: &mut JsFormatter) -> FormatResult<()> {
        let TsEnumMemberFields { name, initializer } = node.as_fields();

        write!(
            f,
            [
                name.format(),
                FormatInitializerClause::new(initializer.as_ref())
            ]
        )
    }
}
