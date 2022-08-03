use crate::prelude::*;

use crate::js::declarations::function_declaration::should_group_function_parameters;
use rome_formatter::write;
use rome_js_syntax::TsFunctionType;
use rome_js_syntax::TsFunctionTypeFields;

#[derive(Debug, Clone, Default)]
pub struct FormatTsFunctionType;

impl FormatNodeRule<TsFunctionType> for FormatTsFunctionType {
    fn fmt_fields(&self, node: &TsFunctionType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsFunctionTypeFields {
            parameters,
            fat_arrow_token,
            type_parameters,
            return_type,
        } = node.as_fields();

        let format_inner = format_with(|f| {
            write![f, [type_parameters.format()]]?;

            let mut format_return_type = return_type.format().memoized();
            let should_group_parameters = should_group_function_parameters(
                type_parameters.as_ref(),
                parameters.as_ref()?.items().len(),
                Some(return_type.clone()),
                &mut format_return_type,
                f,
            )?;

            if should_group_parameters {
                write!(f, [group(&parameters.format())])?;
            } else {
                write!(f, [parameters.format()])?;
            }

            write![
                f,
                [
                    space(),
                    fat_arrow_token.format(),
                    space(),
                    format_return_type
                ]
            ]
        });

        write!(f, [group(&format_inner)])
    }
}
