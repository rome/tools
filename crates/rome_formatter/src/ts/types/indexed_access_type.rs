use crate::formatter_traits::FormatTokenAndNode;
use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::TsIndexedAccessType;
use rome_js_syntax::TsIndexedAccessTypeFields;

impl ToFormatElement for TsIndexedAccessType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsIndexedAccessTypeFields {
            object_type,
            l_brack_token,
            index_type,
            r_brack_token,
        } = self.as_fields();
        Ok(format_elements![
            object_type.format(formatter)?,
            l_brack_token.format(formatter)?,
            index_type.format(formatter)?,
            r_brack_token.format(formatter)?
        ])
    }
}
