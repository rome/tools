use crate::format_traits::FormatOptional;
use crate::utils::format_type_member_separator;
use crate::{space_token, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::{TsIndexSignatureTypeMember, TsIndexSignatureTypeMemberFields};

impl FormatNode for TsIndexSignatureTypeMember {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsIndexSignatureTypeMemberFields {
            readonly_token,
            l_brack_token,
            parameter,
            r_brack_token,
            type_annotation,
            separator_token,
        } = self.as_fields();

        let readonly = readonly_token
            .with_or_empty(|readonly_token| formatted![formatter, readonly_token, space_token()]);

        let l_bracket = l_brack_token.format(formatter)?;
        let parameter = parameter.format(formatter)?;
        let r_bracket = r_brack_token.format(formatter)?;

        let type_annotation = type_annotation.format(formatter)?;
        let separator = format_type_member_separator(separator_token, formatter);

        formatted![
            formatter,
            readonly,
            l_bracket,
            parameter,
            r_bracket,
            type_annotation,
            separator,
        ]
    }
}
