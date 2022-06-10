use crate::prelude::*;

use crate::utils::{property_object_member_layout, write_member_name, PropertyObjectMemberLayout};
use crate::FormatNodeFields;
use rome_formatter::{format_args, write};
use rome_js_syntax::JsPropertyObjectMember;
use rome_js_syntax::JsPropertyObjectMemberFields;

impl FormatNodeFields<JsPropertyObjectMember> for FormatNodeRule<JsPropertyObjectMember> {
    fn fmt_fields(node: &JsPropertyObjectMember, f: &mut JsFormatter) -> FormatResult<()> {
        let JsPropertyObjectMemberFields {
            name,
            colon_token,
            value,
        } = node.as_fields();

        let name = name?;
        let value = value?;
        let format_content = format_with(|f| {
            let name_width = write_member_name(&name, f)?;
            colon_token.format().fmt(f)?;

            let layout = property_object_member_layout(f, name_width, &value)?;
            match layout {
                PropertyObjectMemberLayout::Fluid => {
                    let group_id = f.group_id("property_object_member");

                    let value = value.format().memoized();

                    write![
                        f,
                        [
                            group_elements(&indent(&soft_line_break_or_space()),)
                                .with_group_id(Some(group_id)),
                            line_suffix_boundary(),
                            if_group_breaks(&indent(&value)).with_group_id(Some(group_id)),
                            if_group_fits_on_line(&value).with_group_id(Some(group_id)),
                        ]
                    ]
                }
                PropertyObjectMemberLayout::BreakAfterColon => {
                    write![
                        f,
                        [
                            space_token(),
                            group_elements(&indent(&format_args![
                                soft_line_break_or_space(),
                                value.format()
                            ])),
                        ]
                    ]
                }
                PropertyObjectMemberLayout::NeverBreakAfterColon => {
                    write![f, [space_token(), value.format(),]]
                }
            }
        });

        write!(f, [group_elements(&format_content)])
    }
}
