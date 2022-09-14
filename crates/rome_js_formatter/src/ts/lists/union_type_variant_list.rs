use crate::prelude::*;
use crate::utils::should_hug_type;
use rome_formatter::write;
use rome_js_syntax::{JsLanguage, TsType, TsUnionTypeVariantList};
use rome_rowan::{AstSeparatedElement, AstSeparatedList};

#[derive(Debug, Clone, Default)]
pub struct FormatTsUnionTypeVariantList;

impl FormatRule<TsUnionTypeVariantList> for FormatTsUnionTypeVariantList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &TsUnionTypeVariantList, f: &mut JsFormatter) -> FormatResult<()> {
        // ```ts
        // {
        //   a: string
        // } | null | void
        // ```
        // should be inlined and not be printed in the multi-line variant
        let should_hug = node
            .parent::<TsType>()
            .as_ref()
            .map_or(false, should_hug_type);

        let last_index = node.len().saturating_sub(1);

        f.join_with(space())
            .entries(
                node.elements()
                    .enumerate()
                    .map(|(index, item)| FormatTypeVariant {
                        last: index == last_index,
                        element: item,
                        should_hug,
                    }),
            )
            .finish()
    }
}

pub struct FormatTypeVariant {
    pub last: bool,
    pub should_hug: bool,
    pub element: AstSeparatedElement<JsLanguage, TsType>,
}

impl Format<JsFormatContext> for FormatTypeVariant {
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        let separator = self.element.trailing_separator()?;

        if self.should_hug {
            write!(f, [self.element.node().format()])?;
        } else {
            write!(f, [align(2, &self.element.node().format())])?;
        }

        if let Some(token) = separator {
            if self.last {
                write!(f, [format_removed(token)])?;
            } else {
                if self.should_hug {
                    write!(f, [space()])?;
                } else {
                    write!(f, [soft_line_break_or_space()])?;
                }
                write![f, [token.format()]]?;
            }
        }

        Ok(())
    }
}
