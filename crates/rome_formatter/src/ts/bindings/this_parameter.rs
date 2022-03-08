use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::TsThisParameter;

impl ToFormatElement for TsThisParameter {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let this = self.this_token().format(formatter)?;
        let type_annotation = self.type_annotation().format_or_empty(formatter)?;
        Ok(format_elements![this, type_annotation,])
    }
}
