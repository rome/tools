use crate::{Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;

use rome_formatter::format_elements;
use rome_formatter::soft_block_indent;
use rome_formatter::space_token;
use rome_js_syntax::JsNamedImportSpecifiers;
use rome_js_syntax::JsNamedImportSpecifiersFields;

impl FormatNode for JsNamedImportSpecifiers {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsNamedImportSpecifiersFields {
            l_curly_token,
            specifiers,
            r_curly_token,
        } = self.as_fields();

        let specifiers = specifiers.format(formatter)?;

        formatter.format_delimited_without_group(
            &l_curly_token?,
            move |trailing_trivia, leading_trivia| {
                soft_block_indent(format_elements![
                    trailing_trivia,
                    if specifiers.is_empty() {
                        specifiers
                    } else {
                        format_elements![space_token(), specifiers, space_token()]
                    },
                    leading_trivia
                ])
            },
            &r_curly_token?,
        )
        // formatter.format_delimited_soft_block_indent(
        //     &l_curly_token?,
        //     &r_curly_token?,
        // )
    }
}
