use crate::prelude::*;
use rome_formatter::{format_args, write, FormatRuleWithOptions, GroupId};
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

        if items.is_empty() {
            write!(
                f,
                [
                    l_angle_token.format(),
                    space(),
                    format_dangling_comments(node.syntax()),
                    space(),
                    r_angle_token.format()
                ]
            )
        } else {
            write!(
                f,
                [group(&format_args![
                    l_angle_token.format(),
                    soft_block_indent(&items.format()),
                    r_angle_token.format()
                ])
                .with_group_id(self.group_id)]
            )
        }
    }

    fn fmt_dangling_comments(&self, _: &TsTypeParameters, _: &mut JsFormatter) -> FormatResult<()> {
        // Formatted inside of `fmt_fields`
        Ok(())
    }
}
