use crate::format_traits::FormatOptional;
use crate::utils::format_type_member_separator;
use crate::{format_elements, space_token, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::TsIndexSignatureTypeMember;

impl FormatNode for TsIndexSignatureTypeMember {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let readonly = self
            .readonly_token()
            .format_with_or_empty(formatter, |readonly_token| {
                format_elements![readonly_token, space_token()]
            })?;

        let l_bracket = self.l_brack_token().format(formatter)?;
        let parameter = self.parameter().format(formatter)?;
        let r_bracket = self.r_brack_token().format(formatter)?;

        let type_annotation = self.type_annotation().format(formatter)?;
        let separator = format_type_member_separator(self.separator_token(), formatter);

        Ok(format_elements![
            readonly,
            l_bracket,
            parameter,
            r_bracket,
            type_annotation,
            separator,
        ])
    }
}
