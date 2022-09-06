use crate::prelude::*;

use crate::builders::format_delimited;
use rome_formatter::{write, FormatRuleWithOptions, GroupId};
use rome_js_syntax::{TsTypeParameters, TsTypeParametersFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsTypeParameters {
    group_id: Option<GroupId>,
}

impl FormatRuleWithOptions<TsTypeParameters> for FormatTsTypeParameters {
    type Options = Option<GroupId>;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.group_id = options;
        self
    }
}

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
                format_delimited(&l_angle_token?, &items.format(), &r_angle_token?)
                    .soft_block_indent_with_group_id(self.group_id)
            ]
        )
    }
}
