use crate::prelude::*;
use rome_formatter::{write, Buffer};
use rome_js_syntax::{JsSyntaxKind, TsAnyTypeMember, TsTypeMemberList};

use rome_rowan::AstNodeList;

#[derive(Debug, Clone, Default)]
pub struct FormatTsTypeMemberList;

impl FormatRule<TsTypeMemberList> for FormatTsTypeMemberList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &TsTypeMemberList, f: &mut JsFormatter) -> FormatResult<()> {
        let items = node.iter();
        let last_index = items.len().saturating_sub(1);

        f.join_with(&soft_line_break_or_space())
            .entries(items.enumerate().map(|(index, member)| TsTypeMemberItem {
                last: index == last_index,
                member,
            }))
            .finish()
    }
}

struct TsTypeMemberItem {
    last: bool,
    member: TsAnyTypeMember,
}

impl Format<JsFormatContext> for TsTypeMemberItem {
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        let mut is_verbatim = false;

        write!(
            f,
            [group(&format_once(|f| {
                write!(f, [self.member.format()])?;

                is_verbatim = f.elements().last().map_or(false, |last| {
                    matches!(last.last_element(), Some(FormatElement::Verbatim(_)))
                });

                Ok(())
            }))]
        )?;

        if !is_verbatim {
            // Children don't format the separator on purpose, so it's up to the parent - this node,
            // to decide to print their separator
            if self.last {
                write!(
                    f,
                    [if_group_breaks(&format_inserted(JsSyntaxKind::SEMICOLON))]
                )?;
            } else {
                format_inserted(JsSyntaxKind::SEMICOLON).fmt(f)?;
            }
        }

        Ok(())
    }
}
