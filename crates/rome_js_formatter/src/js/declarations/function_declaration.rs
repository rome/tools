use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::{
    JsAnyBinding, JsFunctionBody, JsFunctionDeclaration, JsFunctionExportDefaultDeclaration,
    JsFunctionExpression, JsParameters, JsSyntaxToken, TsReturnTypeAnnotation, TsTypeParameters,
};
use rome_rowan::{declare_node_union, SyntaxResult};

#[derive(Debug, Clone, Default)]
pub struct FormatJsFunctionDeclaration;

impl FormatNodeRule<JsFunctionDeclaration> for FormatJsFunctionDeclaration {
    fn fmt_fields(&self, node: &JsFunctionDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
        write![f, [FormatFunction::from(node.clone())]]
    }
}

declare_node_union! {
    pub(crate) FormatFunction = JsFunctionDeclaration | JsFunctionExpression | JsFunctionExportDefaultDeclaration
}

impl FormatFunction {
    fn async_token(&self) -> Option<JsSyntaxToken> {
        match self {
            FormatFunction::JsFunctionDeclaration(declaration) => declaration.async_token(),
            FormatFunction::JsFunctionExpression(expression) => expression.async_token(),
            FormatFunction::JsFunctionExportDefaultDeclaration(declaration) => {
                declaration.async_token()
            }
        }
    }

    fn function_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            FormatFunction::JsFunctionDeclaration(declaration) => declaration.function_token(),
            FormatFunction::JsFunctionExpression(expression) => expression.function_token(),
            FormatFunction::JsFunctionExportDefaultDeclaration(declaration) => {
                declaration.function_token()
            }
        }
    }

    fn star_token(&self) -> Option<JsSyntaxToken> {
        match self {
            FormatFunction::JsFunctionDeclaration(declaration) => declaration.star_token(),
            FormatFunction::JsFunctionExpression(expression) => expression.star_token(),
            FormatFunction::JsFunctionExportDefaultDeclaration(declaration) => {
                declaration.star_token()
            }
        }
    }

    fn id(&self) -> SyntaxResult<Option<JsAnyBinding>> {
        match self {
            FormatFunction::JsFunctionDeclaration(declaration) => {
                declaration.id().map(|binding| Some(binding))
            }
            FormatFunction::JsFunctionExpression(expression) => Ok(expression.id()),
            FormatFunction::JsFunctionExportDefaultDeclaration(declaration) => Ok(declaration.id()),
        }
    }

    fn type_parameters(&self) -> Option<TsTypeParameters> {
        match self {
            FormatFunction::JsFunctionDeclaration(declaration) => declaration.type_parameters(),
            FormatFunction::JsFunctionExpression(expression) => expression.type_parameters(),
            FormatFunction::JsFunctionExportDefaultDeclaration(declaration) => {
                declaration.type_parameters()
            }
        }
    }

    fn parameters(&self) -> SyntaxResult<JsParameters> {
        match self {
            FormatFunction::JsFunctionDeclaration(declaration) => declaration.parameters(),
            FormatFunction::JsFunctionExpression(expression) => expression.parameters(),
            FormatFunction::JsFunctionExportDefaultDeclaration(declaration) => {
                declaration.parameters()
            }
        }
    }

    fn return_type_annotation(&self) -> Option<TsReturnTypeAnnotation> {
        match self {
            FormatFunction::JsFunctionDeclaration(declaration) => {
                declaration.return_type_annotation()
            }
            FormatFunction::JsFunctionExpression(expression) => expression.return_type_annotation(),
            FormatFunction::JsFunctionExportDefaultDeclaration(declaration) => {
                declaration.return_type_annotation()
            }
        }
    }

    fn body(&self) -> SyntaxResult<JsFunctionBody> {
        match self {
            FormatFunction::JsFunctionDeclaration(declaration) => declaration.body(),
            FormatFunction::JsFunctionExpression(expression) => expression.body(),
            FormatFunction::JsFunctionExportDefaultDeclaration(declaration) => declaration.body(),
        }
    }
}

impl Format<JsFormatContext> for FormatFunction {
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        if let Some(async_token) = self.async_token() {
            write!(f, [async_token.format(), space()])?;
        }

        write!(
            f,
            [self.function_token().format(), self.star_token().format()]
        )?;

        match self.id()? {
            Some(id) => {
                write!(f, [space(), id.format()])?;
            }
            None => {
                write!(f, [space()])?;
            }
        }

        write!(f, [self.type_parameters().format()])?;

        write!(
            f,
            [group(&format_with(|f| {
                write![
                    f,
                    [
                        self.parameters().format(),
                        self.return_type_annotation().format(),
                        space()
                    ]
                ]
            }))]
        )?;

        write!(f, [self.body().format()])
    }
}
