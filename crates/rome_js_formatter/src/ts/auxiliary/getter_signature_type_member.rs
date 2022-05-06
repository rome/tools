use crate::format_traits::FormatOptional;
use crate::utils::format_type_member_separator;
use crate::{space_token, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::TsGetterSignatureTypeMember;

impl FormatNode for TsGetterSignatureTypeMember {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let get = self.get_token().format(formatter)?;
        let name = self.name().format(formatter)?;
        let l_paren = self.l_paren_token().format(formatter)?;
        let r_paren = self.r_paren_token().format(formatter)?;
        let type_annotation = self.type_annotation().format_or_empty(formatter)?;
        let separator = format_type_member_separator(self.separator_token(), formatter);

        formatted![
            formatter,
            get,
            space_token(),
            name,
            l_paren,
            r_paren,
            type_annotation,
            separator
        ]
    }
}
