use crate::formatter_traits::FormatTokenAndNode;
use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::{TsDecorator, TsDecoratorFields};

impl ToFormatElement for TsDecorator {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsDecoratorFields {
            at_token,
            expression,
        } = self.as_fields();

        Ok(format_elements![
            at_token.format(formatter)?,
            expression.format(formatter)?
        ])
    }
}
