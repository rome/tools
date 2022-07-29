use crate::prelude::*;
use rome_formatter::{format_args, write};

use crate::AsFormat;
use rome_js_syntax::{JsSyntaxKind, JsVariableDeclaratorList};
use rome_rowan::AstSeparatedList;

#[derive(Debug, Clone, Default)]
pub struct FormatJsVariableDeclaratorList;

impl FormatRule<JsVariableDeclaratorList> for FormatJsVariableDeclaratorList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &JsVariableDeclaratorList, f: &mut JsFormatter) -> FormatResult<()> {
        let last_index = node.len().saturating_sub(1);

        let mut declarators = node.elements().enumerate().map(|(index, element)| {
            format_with(move |f| {
                write!(f, [&element.node().format()])?;

                match element.trailing_separator()? {
                    None => {
                        if index != last_index {
                            format_inserted(JsSyntaxKind::COMMA).fmt(f)?;
                        }
                    }
                    Some(separator) => {
                        if index != last_index {
                            write!(f, [separator.format()])?;
                        }
                    }
                };

                Ok(())
            })
        });

        let leading_element = declarators.next().ok_or(FormatError::SyntaxError)?;

        let other_declarators = format_once(|f| {
            if node.len() == 1 {
                // No more declarators, avoid single line break
                return Ok(());
            }

            write!(f, [soft_line_break_or_space()])?;

            f.join_with(&soft_line_break_or_space())
                .entries(declarators)
                .finish()
        });

        write!(
            f,
            [group(&format_args!(
                leading_element,
                indent(&other_declarators)
            ))]
        )
    }
}
