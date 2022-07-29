use crate::prelude::*;
use rome_formatter::write;
use rome_js_syntax::{JsLanguage, JsSyntaxKind, TsType, TsUnionTypeVariantList};
use rome_rowan::{AstSeparatedElement, AstSeparatedList};

#[derive(Debug, Clone, Default)]
pub struct FormatTsUnionTypeVariantList;

impl FormatRule<TsUnionTypeVariantList> for FormatTsUnionTypeVariantList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &TsUnionTypeVariantList, f: &mut JsFormatter) -> FormatResult<()> {
        let last_index = node.len().saturating_sub(1);

        f.join()
            .entries(
                node.elements()
                    .enumerate()
                    .map(|(index, item)| FormatTypeVariant {
                        last: index == last_index,
                        element: item,
                    }),
            )
            .finish()
    }
}

pub struct FormatTypeVariant {
    pub last: bool,
    pub element: AstSeparatedElement<JsLanguage, TsType>,
}

impl Format<JsFormatContext> for FormatTypeVariant {
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        write!(f, [group(&self.element.node().format())])?;

        let separator = self.element.trailing_separator()?;

        match separator {
            Some(token) => {
                if self.last {
                    write!(f, [format_removed(token)])?;
                } else {
                    write![f, [soft_line_break_or_space(), token.format(), space()]]?;
                }
            }
            None => {
                if !self.last {
                    write![
                        f,
                        [
                            soft_line_break_or_space(),
                            format_inserted(JsSyntaxKind::PIPE),
                            space()
                        ]
                    ]?;
                }
            }
        }

        Ok(())
    }
}
