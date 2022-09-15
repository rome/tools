use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::JsNamedImportSpecifiers;
use rome_js_syntax::JsNamedImportSpecifiersFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsNamedImportSpecifiers;

impl FormatNodeRule<JsNamedImportSpecifiers> for FormatJsNamedImportSpecifiers {
    fn fmt_fields(&self, node: &JsNamedImportSpecifiers, f: &mut JsFormatter) -> FormatResult<()> {
        let JsNamedImportSpecifiersFields {
            l_curly_token,
            specifiers,
            r_curly_token,
        } = node.as_fields();

        write!(f, [l_curly_token.format()])?;

        if specifiers.is_empty() {
            write!(
                f,
                [format_dangling_comments(node.syntax()).with_soft_block_indent()]
            )?;
        } else {
            write!(
                f,
                [group(&soft_line_indent_or_spaced(&specifiers.format()))]
            )?;
        }

        write!(f, [r_curly_token.format()])
    }

    fn fmt_dangling_comments(
        &self,
        _: &JsNamedImportSpecifiers,
        _: &mut JsFormatter,
    ) -> FormatResult<()> {
        // Handled inside of `fmt_fields`
        Ok(())
    }
}
