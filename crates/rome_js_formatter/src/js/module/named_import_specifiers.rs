use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::JsNamedImportSpecifiers;
use rome_js_syntax::JsNamedImportSpecifiersFields;
use rome_rowan::AstSeparatedElement;

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

        match specifiers.elements().next() {
            Some(AstSeparatedElement {
                node: Ok(node),
                trailing_separator: Ok(separator),
            }) if specifiers.len() == 1 && !f.comments().has_comments(node.syntax()) => {
                write!(
                    f,
                    [space(), group(&soft_space_or_block_indent(&node.format()))]
                )?;

                if let Some(separator) = separator {
                    write!(f, [format_removed(&separator)])?;
                }

                write!(f, [space()])?;
            }
            Some(_) => {
                write!(
                    f,
                    [group(&soft_space_or_block_indent(&specifiers.format()))]
                )?;
            }
            None => {
                write!(
                    f,
                    [format_dangling_comments(node.syntax()).with_soft_block_indent()]
                )?;
            }
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
