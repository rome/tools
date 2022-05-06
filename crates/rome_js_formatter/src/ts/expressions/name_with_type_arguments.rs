use crate::format_traits::FormatOptional;
use crate::{Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::{TsNameWithTypeArguments, TsNameWithTypeArgumentsFields};

impl FormatNode for TsNameWithTypeArguments {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsNameWithTypeArgumentsFields {
            name,
            type_arguments,
        } = self.as_fields();

        let name = name.format(formatter)?;
        let type_arguments = type_arguments.format_or_empty(formatter)?;
        formatted![formatter, name, type_arguments]
    }
}
