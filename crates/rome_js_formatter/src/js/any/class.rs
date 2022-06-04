use crate::generated::FormatJsAnyClass;
use rome_formatter::write;

use crate::builders::format_delimited;
use crate::prelude::*;
use rome_js_syntax::JsAnyClass;
use rome_rowan::AstNode;

impl FormatRule<JsAnyClass> for FormatJsAnyClass {
    type Context = JsFormatContext;

    fn fmt(node: &JsAnyClass, f: &mut JsFormatter) -> FormatResult<()> {
        let abstract_token = node.abstract_token();
        let id = node.id();
        let extends = node.extends_clause();
        let implements_clause = node.implements_clause();

        if let Some(abstract_token) = abstract_token {
            write!(f, [abstract_token.format(), space_token()])?;
        }

        write!(f, [node.class_token().format()])?;

        if let Some(id) = id? {
            write!(f, [space_token(), id.format()])?;
        }

        write!(f, [node.type_parameters().format()])?;

        if let Some(extends) = extends {
            write!(f, [space_token(), extends.format()])?;
        }

        if let Some(implements_clause) = implements_clause {
            write!(f, [space_token(), implements_clause.format()])?;
        }

        let members = format_with(|f| {
            let mut join = f.join_nodes_with_hardline();

            for member in node.members() {
                join.entry(member.syntax(), &member.format());
            }

            join.finish()
        });

        write![
            f,
            [
                space_token(),
                format_delimited(&node.l_curly_token()?, &members, &node.r_curly_token()?)
                    .block_indent()
            ]
        ]
    }
}
