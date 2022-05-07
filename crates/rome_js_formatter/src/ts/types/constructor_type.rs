use crate::prelude::*;
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
        let abstract_token =
            abstract_token.with_or_empty(|element| formatted![formatter, element, space_token()]);

        formatted![
            formatter,
            abstract_token,
            new_token.format(formatter)?,
            type_parameters,
            parameters.format(formatter)?,
            space_token(),
            fat_arrow_token.format(formatter)?,
            space_token(),
            return_type.format(formatter)?
        ]
    }
}
