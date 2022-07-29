use crate::prelude::*;
use rome_formatter::write;
use rome_js_syntax::JsAnyClass;

pub struct FormatClass<'a> {
    class: &'a JsAnyClass,
}

impl FormatClass<'_> {
    fn should_group(&self) -> FormatResult<bool> {
        if let Some(id) = self.class.id()? {
            if id.syntax().has_trailing_comments() {
                return Ok(true);
            }
        }

        if let Some(type_parameters) = self.class.type_parameters() {
            if type_parameters.syntax().has_trailing_comments() {
                return Ok(true);
            }
        }

        if let Some(extends) = self.class.extends_clause() {
            if extends.syntax().has_trailing_comments() {
                return Ok(true);
            }
        }

        if self.class.implements_clause().is_some() {
            return Ok(true);
        }

        Ok(false)
    }
}

impl<'a> From<&'a JsAnyClass> for FormatClass<'a> {
    fn from(class: &'a JsAnyClass) -> Self {
        Self { class }
    }
}

impl Format<JsFormatContext> for FormatClass<'_> {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        let abstract_token = self.class.abstract_token();
        let id = self.class.id()?;
        let extends = self.class.extends_clause();
        let implements_clause = self.class.implements_clause();
        let type_parameters = self.class.type_parameters();
        let class_token = self.class.class_token()?;
        let members = self.class.members();

        let group_mode = self.should_group()?;

        if let Some(abstract_token) = abstract_token {
            write!(f, [abstract_token.format(), space()])?;
        }

        write!(f, [class_token.format()])?;

        let head = format_with(|f| {
            if let Some(id) = &id {
                write!(f, [space(), id.format()])?;
            }

            write!(f, [type_parameters.format()])?;

            if let Some(extends) = &extends {
                if group_mode {
                    write!(f, [soft_line_break_or_space()])?;
                } else {
                    write!(f, [space()])?;
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
                    group(&indent(&head)).with_group_id(Some(heritage_id)),
                    space(),
                ]
            )?;

            if !members.is_empty() {
                write!(
                    f,
                    [if_group_breaks(&hard_line_break()).with_group_id(Some(heritage_id))]
                )?;
            }
        } else {
            write!(f, [head, space()])?;
        }

        write![
            f,
            [format_delimited(
                &self.class.l_curly_token()?,
                &members.format(),
                &self.class.r_curly_token()?
            )
            .block_indent()]
        ]
    }
}
