use crate::ast::{
    JsAnyArrowFunctionParameters, JsAnyBinding, JsAnyClass, JsAnyFormalParameter, JsAnyFunction,
    JsAnyFunctionBody, JsClassMemberList, JsExtendsClause, TsAnyPropertyAnnotation,
    TsAnyPropertyParameter, TsAnyVariableAnnotation, TsImplementsClause, TsReturnTypeAnnotation,
    TsTypeAnnotation, TsTypeParameters,
};
use crate::{SyntaxResult, SyntaxToken};

impl JsAnyClass {
    pub fn class_token(&self) -> SyntaxResult<SyntaxToken> {
        match self {
            JsAnyClass::JsClassDeclaration(declaration) => declaration.class_token(),
            JsAnyClass::JsClassExpression(expression) => expression.class_token(),
            JsAnyClass::JsExportDefaultClassClause(clause) => clause.class_token(),
        }
    }

    pub fn id(&self) -> SyntaxResult<Option<JsAnyBinding>> {
        match self {
            JsAnyClass::JsClassDeclaration(declaration) => declaration.id().map(Some),
            JsAnyClass::JsClassExpression(expression) => Ok(expression.id()),
            JsAnyClass::JsExportDefaultClassClause(clause) => Ok(clause.id()),
        }
    }

    pub fn extends_clause(&self) -> Option<JsExtendsClause> {
        match self {
            JsAnyClass::JsClassDeclaration(declaration) => declaration.extends_clause(),
            JsAnyClass::JsClassExpression(expression) => expression.extends_clause(),
            JsAnyClass::JsExportDefaultClassClause(clause) => clause.extends_clause(),
        }
    }

    pub fn implements_clause(&self) -> Option<TsImplementsClause> {
        match self {
            JsAnyClass::JsClassDeclaration(declaration) => declaration.implements_clause(),
            JsAnyClass::JsClassExpression(expression) => expression.implements_clause(),
            JsAnyClass::JsExportDefaultClassClause(clause) => clause.implements_clause(),
        }
    }

    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        match self {
            JsAnyClass::JsClassDeclaration(declaration) => declaration.l_curly_token(),
            JsAnyClass::JsClassExpression(expression) => expression.l_curly_token(),
            JsAnyClass::JsExportDefaultClassClause(clause) => clause.l_curly_token(),
        }
    }

    pub fn members(&self) -> JsClassMemberList {
        match self {
            JsAnyClass::JsClassDeclaration(declaration) => declaration.members(),
            JsAnyClass::JsClassExpression(expression) => expression.members(),
            JsAnyClass::JsExportDefaultClassClause(clause) => clause.members(),
        }
    }

    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        match self {
            JsAnyClass::JsClassDeclaration(declaration) => declaration.r_curly_token(),
            JsAnyClass::JsClassExpression(expression) => expression.r_curly_token(),
            JsAnyClass::JsExportDefaultClassClause(clause) => clause.r_curly_token(),
        }
    }
}

impl JsAnyFunction {
    pub fn async_token(&self) -> Option<SyntaxToken> {
        match self {
            JsAnyFunction::JsArrowFunctionExpression(expr) => expr.async_token(),
            JsAnyFunction::JsFunctionExpression(expr) => expr.async_token(),
            JsAnyFunction::JsFunctionDeclaration(declaration) => declaration.async_token(),
            JsAnyFunction::JsExportDefaultFunctionClause(clause) => clause.async_token(),
        }
    }

    pub fn is_async(&self) -> bool {
        self.async_token().is_some()
    }

    pub fn function_token(&self) -> SyntaxResult<Option<SyntaxToken>> {
        match self {
            JsAnyFunction::JsArrowFunctionExpression(_) => Ok(None),
            JsAnyFunction::JsFunctionExpression(expr) => expr.function_token().map(Some),
            JsAnyFunction::JsFunctionDeclaration(declaration) => {
                declaration.function_token().map(Some)
            }
            JsAnyFunction::JsExportDefaultFunctionClause(clause) => {
                clause.function_token().map(Some)
            }
        }
    }

    pub fn star_token(&self) -> Option<SyntaxToken> {
        match self {
            JsAnyFunction::JsArrowFunctionExpression(_) => None,
            JsAnyFunction::JsFunctionExpression(expr) => expr.star_token(),
            JsAnyFunction::JsFunctionDeclaration(declaration) => declaration.star_token(),
            JsAnyFunction::JsExportDefaultFunctionClause(clause) => clause.star_token(),
        }
    }

    pub fn is_generator(&self) -> bool {
        self.star_token().is_some()
    }

    pub fn id(&self) -> SyntaxResult<Option<JsAnyBinding>> {
        match self {
            JsAnyFunction::JsArrowFunctionExpression(_) => Ok(None),
            JsAnyFunction::JsFunctionExpression(expr) => Ok(expr.id()),
            JsAnyFunction::JsFunctionDeclaration(declaration) => declaration.id().map(Some),
            JsAnyFunction::JsExportDefaultFunctionClause(clause) => Ok(clause.id()),
        }
    }

    pub fn type_parameters(&self) -> Option<TsTypeParameters> {
        match self {
            JsAnyFunction::JsArrowFunctionExpression(expr) => expr.type_parameters(),
            JsAnyFunction::JsFunctionExpression(expr) => expr.type_parameters(),
            JsAnyFunction::JsFunctionDeclaration(declaration) => declaration.type_parameters(),
            JsAnyFunction::JsExportDefaultFunctionClause(clause) => clause.type_parameters(),
        }
    }

    pub fn parameters(&self) -> SyntaxResult<JsAnyArrowFunctionParameters> {
        match self {
            JsAnyFunction::JsArrowFunctionExpression(expr) => expr.parameters(),
            JsAnyFunction::JsFunctionExpression(expr) => expr
                .parameters()
                .map(JsAnyArrowFunctionParameters::JsParameters),
            JsAnyFunction::JsFunctionDeclaration(declaration) => declaration
                .parameters()
                .map(JsAnyArrowFunctionParameters::JsParameters),
            JsAnyFunction::JsExportDefaultFunctionClause(clause) => clause
                .parameters()
                .map(JsAnyArrowFunctionParameters::JsParameters),
        }
    }

    pub fn return_type_annotation(&self) -> Option<TsReturnTypeAnnotation> {
        match self {
            JsAnyFunction::JsArrowFunctionExpression(expr) => expr.return_type_annotation(),
            JsAnyFunction::JsFunctionExpression(expr) => expr.return_type_annotation(),
            JsAnyFunction::JsFunctionDeclaration(declaration) => {
                declaration.return_type_annotation()
            }
            JsAnyFunction::JsExportDefaultFunctionClause(clause) => clause.return_type_annotation(),
        }
    }

    pub fn body(&self) -> SyntaxResult<JsAnyFunctionBody> {
        match self {
            JsAnyFunction::JsArrowFunctionExpression(expr) => expr.body(),
            JsAnyFunction::JsFunctionExpression(expr) => {
                expr.body().map(JsAnyFunctionBody::JsFunctionBody)
            }
            JsAnyFunction::JsFunctionDeclaration(declaration) => {
                declaration.body().map(JsAnyFunctionBody::JsFunctionBody)
            }
            JsAnyFunction::JsExportDefaultFunctionClause(clause) => {
                clause.body().map(JsAnyFunctionBody::JsFunctionBody)
            }
        }
    }
}

impl TsAnyPropertyParameter {
    pub fn accessibility(&self) -> Option<SyntaxToken> {
        match self {
            TsAnyPropertyParameter::TsPropertyParameter(parameter) => {
                parameter.accessibility().ok()
            }
            TsAnyPropertyParameter::TsReadonlyPropertyParameter(parameter) => {
                parameter.accessibility()
            }
        }
    }

    pub fn formal_parameter(&self) -> SyntaxResult<JsAnyFormalParameter> {
        match self {
            TsAnyPropertyParameter::TsPropertyParameter(parameter) => parameter.formal_parameter(),
            TsAnyPropertyParameter::TsReadonlyPropertyParameter(parameter) => {
                parameter.formal_parameter()
            }
        }
    }
}

impl TsAnyVariableAnnotation {
    pub fn type_annotation(&self) -> SyntaxResult<Option<TsTypeAnnotation>> {
        match self {
            TsAnyVariableAnnotation::TsDefiniteVariableAnnotation(definite) => {
                definite.type_annotation().map(Some)
            }
            TsAnyVariableAnnotation::TsTypeAnnotation(type_annotation) => {
                Ok(Some(type_annotation.clone()))
            }
        }
    }
}

impl TsAnyPropertyAnnotation {
    pub fn type_annotation(&self) -> SyntaxResult<Option<TsTypeAnnotation>> {
        match self {
            TsAnyPropertyAnnotation::TsDefinitePropertyAnnotation(definite) => {
                definite.type_annotation().map(Some)
            }
            TsAnyPropertyAnnotation::TsOptionalPropertyAnnotation(optional) => {
                Ok(optional.type_annotation())
            }
            TsAnyPropertyAnnotation::TsTypeAnnotation(type_annotation) => {
                Ok(Some(type_annotation.clone()))
            }
        }
    }
}
