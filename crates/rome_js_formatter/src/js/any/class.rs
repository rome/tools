use crate::generated::FormatJsAnyClass;
use rome_formatter::write;

use crate::builders::format_delimited;
use crate::prelude::*;
use rome_js_syntax::JsAnyClass;

impl FormatRule<JsAnyClass> for FormatJsAnyClass {
    type Context = JsFormatContext;

    fn fmt(node: &JsAnyClass, f: &mut JsFormatter) -> FormatResult<()> {
        let abstract_token = node.abstract_token();
        let id = node.id();
        let extends = node.extends_clause();
        let implements_clause = node.implements_clause();
        let id = id?;

        let group_mode = should_group_class(node)?;

        if let Some(abstract_token) = abstract_token {
            write!(f, [abstract_token.format(), space_token()])?;
        }

        write!(f, [node.class_token().format()])?;

        let head = format_with(|f| {
            if let Some(id) = &id {
                write!(f, [space_token(), id.format()])?;
            }

            write!(f, [node.type_parameters().format()])?;

            if let Some(extends) = &extends {
                if group_mode {
                    write!(f, [soft_line_break_or_space()])?;
                } else {
                    write!(f, [space_token()])?;
                }

                write!(f, [extends.format()])?;
            }

            if let Some(implements_clause) = &implements_clause {
                write!(f, [soft_line_break_or_space(), implements_clause.format()])?;
            }

            Ok(())
        });

        if group_mode {
            let heritage_id = f.group_id("heritageGroup");

            write!(
                f,
                [
                    group_elements(&indent(&head)).with_group_id(Some(heritage_id)),
                    space_token(),
                ]
            )?;

            if !node.members().is_empty() {
                write!(
                    f,
                    [if_group_breaks(&hard_line_break()).with_group_id(Some(heritage_id))]
                )?;
            }
        } else {
            write!(f, [head, space_token()])?;
        }

        write![
            f,
            [format_delimited(
                &node.l_curly_token()?,
                &node.members().format(),
                &node.r_curly_token()?
            )
            .block_indent()]
        ]
    }
}

fn should_group_class(class: &JsAnyClass) -> FormatResult<bool> {
    if let Some(id) = class.id()? {
        if id.syntax().has_trailing_comments() {
            return Ok(true);
        }
    }

    if let Some(type_parameters) = class.type_parameters() {
        if type_parameters.syntax().has_trailing_comments() {
            return Ok(true);
        }
    }

    if let Some(extends) = class.extends_clause() {
        if extends.syntax().has_trailing_comments() {
            return Ok(true);
        }
    }

    if class.implements_clause().is_some() {
        return Ok(true);
    }

    Ok(false)
}
