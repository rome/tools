use crate::format_traits::FormatOptional;
use crate::{format_elements, Format, FormatElement, FormatNode, FormatResult, Formatter};
use rome_js_syntax::TsThisParameter;

impl FormatNode for TsThisParameter {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let this = self.this_token().format(formatter)?;
        let type_annotation = self.type_annotation().format_or_empty(formatter)?;
        Ok(format_elements![this, type_annotation,])
    }
}
