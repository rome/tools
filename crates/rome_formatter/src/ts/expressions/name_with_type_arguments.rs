use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::{TsNameWithTypeArguments, TsNameWithTypeArgumentsFields};

impl ToFormatElement for TsNameWithTypeArguments {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsNameWithTypeArgumentsFields {
            name,
            type_arguments,
        } = self.as_fields();

        let name = name.format(formatter)?;
        let type_arguments = type_arguments.format_or_empty(formatter)?;
        Ok(format_elements![name, type_arguments])
    }
}
