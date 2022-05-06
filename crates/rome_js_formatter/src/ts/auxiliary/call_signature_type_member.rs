use crate::format_traits::FormatOptional;
use crate::utils::format_type_member_separator;
use crate::{Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::TsCallSignatureTypeMember;

impl FormatNode for TsCallSignatureTypeMember {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let type_parameters = self.type_parameters().format_or_empty(formatter)?;
        let parameters = self.parameters().format(formatter)?;
        let return_type_annotation = self.return_type_annotation().format_or_empty(formatter)?;
        let separator = format_type_member_separator(self.separator_token(), formatter);

        formatted![
            formatter,
            type_parameters,
            parameters,
            return_type_annotation,
            separator
        ]
    }
}
