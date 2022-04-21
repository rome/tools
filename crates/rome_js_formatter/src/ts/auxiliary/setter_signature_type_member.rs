use crate::utils::format_type_member_separator;
use crate::{
    format_elements, space_token, Format, FormatElement, FormatNode, FormatResult, Formatter,
};
use rome_js_syntax::TsSetterSignatureTypeMember;

impl FormatNode for TsSetterSignatureTypeMember {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let set = self.set_token().format(formatter)?;
        let name = self.name().format(formatter)?;
        let l_paren = self.l_paren_token().format(formatter)?;
        let parameter = self.parameter().format(formatter)?;
        let r_paren = self.r_paren_token().format(formatter)?;
        let separator = format_type_member_separator(self.separator_token(), formatter);

        Ok(format_elements![
            set,
            space_token(),
            name,
            l_paren,
            parameter,
            r_paren,
            separator
        ])
    }
}
