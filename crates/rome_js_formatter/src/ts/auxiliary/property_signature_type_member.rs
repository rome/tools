use crate::prelude::*;
use crate::utils::format_type_member_separator;
use rome_js_syntax::TsPropertySignatureTypeMember;

impl FormatNode for TsPropertySignatureTypeMember {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let name = self.name().format(formatter)?;
        let separator = format_type_member_separator(self.separator_token(), formatter);

        formatted![
            formatter,
            self.readonly_token(),
            space_token(),
            name,
            self.optional_token(),
            self.type_annotation(),
            separator
        ]
    }
}
