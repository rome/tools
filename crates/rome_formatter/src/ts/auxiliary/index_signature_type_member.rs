use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::utils::format_type_member_separator;
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rome_js_syntax::TsIndexSignatureTypeMember;

impl ToFormatElement for TsIndexSignatureTypeMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
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
