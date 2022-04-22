use crate::format_traits::FormatOptional;
use crate::{format_elements, space_token, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::TsConstructorType;
use rome_js_syntax::TsConstructorTypeFields;

impl FormatNode for TsConstructorType {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsConstructorTypeFields {
            abstract_token,
            new_token,
            type_parameters,
            parameters,
            fat_arrow_token,
            return_type,
        } = self.as_fields();
        let abstract_token = abstract_token.format_with_or_empty(formatter, |element| {
            format_elements![element, space_token()]
        })?;

        Ok(format_elements![
            abstract_token,
            new_token.format(formatter)?,
            type_parameters.format_or_empty(formatter)?,
            parameters.format(formatter)?,
            space_token(),
            fat_arrow_token.format(formatter)?,
            space_token(),
            return_type.format(formatter)?
        ])
    }
}
