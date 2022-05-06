use crate::format_traits::FormatOptional;
use crate::{
    hard_group_elements, space_token, Format, FormatElement, FormatNode, Formatter,
};
use rome_formatter::FormatResult;
use rome_js_syntax::TsFunctionType;
use rome_js_syntax::TsFunctionTypeFields;

impl FormatNode for TsFunctionType {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsFunctionTypeFields {
            parameters,
            fat_arrow_token,
            type_parameters,
            return_type,
        } = self.as_fields();

        Ok(hard_group_elements(formatted![
            formatter,
            type_parameters.format_or_empty(formatter)?,
            parameters.format(formatter)?,
            space_token(),
            fat_arrow_token.format(formatter)?,
            space_token(),
            return_type.format(formatter)?
        ]?))
    }
}
