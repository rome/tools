use crate::format_traits::FormatOptional;
use crate::utils::format_type_member_separator;
use crate::{
    format_elements, space_token, Format, FormatElement, FormatNode, FormatResult, Formatter,
};
use rome_js_syntax::TsPropertySignatureTypeMember;

impl FormatNode for TsPropertySignatureTypeMember {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let readonly = self.readonly_token().format_or_empty(formatter)?;
        let name = self.name().format(formatter)?;
        let optional = self.optional_token().format_or_empty(formatter)?;
        let type_annotation = self.type_annotation().format_or_empty(formatter)?;
        let separator = format_type_member_separator(self.separator_token(), formatter);

        Ok(format_elements![
            readonly,
            space_token(),
            name,
            optional,
            type_annotation,
            separator
        ])
    }
}
