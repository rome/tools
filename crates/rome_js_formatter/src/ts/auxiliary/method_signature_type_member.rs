use crate::prelude::*;
use crate::utils::format_type_member_separator;
use rome_js_syntax::TsMethodSignatureTypeMember;

impl FormatNode for TsMethodSignatureTypeMember {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let name = self.name().format(formatter)?;
        let optional_token = self.optional_token();
        let type_arguments = self.type_parameters();
        let parameters = self.parameters().format(formatter)?;
        let return_type_annotation = self.return_type_annotation();
        let separator = format_type_member_separator(self.separator_token(), formatter);
        formatted![
            formatter,
            name,
            optional_token,
            type_arguments,
            parameters,
            return_type_annotation,
            separator
        ]
    }
}
