use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::TsDecoratorList;
impl ToFormatElement for TsDecoratorList {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(formatter.format_list(self.clone()))
    }
}
