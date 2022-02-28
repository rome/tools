use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsTemplateChunkElement;
use rslint_parser::ast::TsTemplateChunkElementFields;

impl ToFormatElement for TsTemplateChunkElement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsTemplateChunkElementFields {
            template_chunk_token,
        } = self.as_fields();

        template_chunk_token.format(formatter)
    }
}
