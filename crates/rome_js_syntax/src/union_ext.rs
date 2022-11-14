use crate::{
    JsAnyArrowFunctionParameters, JsAnyBinding, JsAnyClass, JsAnyClassMember, JsAnyClassMemberName,
    JsAnyFunction, JsAnyFunctionBody, JsClassMemberList, JsExtendsClause, JsSyntaxToken,
    TsAnyPropertyAnnotation, TsAnyVariableAnnotation, TsImplementsClause, TsReturnTypeAnnotation,
    TsTypeAnnotation, TsTypeParameters,
};
use rome_rowan::{AstSeparatedList, SyntaxResult};

impl JsAnyClass {
    pub fn abstract_token(&self) -> Option<JsSyntaxToken> {
        match self {
            JsAnyClass::JsClassDeclaration(declaration) => declaration.abstract_token(),
            JsAnyClass::JsClassExpression(_) => None,
            JsAnyClass::JsClassExportDefaultDeclaration(clause) => clause.abstract_token(),
        }
    }

    pub fn class_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            JsAnyClass::JsClassDeclaration(declaration) => declaration.class_token(),
            JsAnyClass::JsClassExpression(expression) => expression.class_token(),
            JsAnyClass::JsClassExportDefaultDeclaration(declaration) => declaration.class_token(),
        }
    }

    pub fn id(&self) -> SyntaxResult<Option<JsAnyBinding>> {
        match self {
            JsAnyClass::JsClassDeclaration(declaration) => declaration.id().map(Some),
            JsAnyClass::JsClassExpression(expression) => Ok(expression.id()),
            JsAnyClass::JsClassExportDefaultDeclaration(declaration) => Ok(declaration.id()),
        }
    }

    pub fn type_parameters(&self) -> Option<TsTypeParameters> {
        match self {
            JsAnyClass::JsClassDeclaration(declaration) => declaration.type_parameters(),
            JsAnyClass::JsClassExpression(expression) => expression.type_parameters(),
            JsAnyClass::JsClassExportDefaultDeclaration(clause) => clause.type_parameters(),
        }
    }

    pub fn extends_clause(&self) -> Option<JsExtendsClause> {
        match self {
            JsAnyClass::JsClassDeclaration(declaration) => declaration.extends_clause(),
            JsAnyClass::JsClassExpression(expression) => expression.extends_clause(),
            JsAnyClass::JsClassExportDefaultDeclaration(declaration) => {
                declaration.extends_clause()
            }
        }
    }

    pub fn implements_clause(&self) -> Option<TsImplementsClause> {
        match self {
            JsAnyClass::JsClassDeclaration(declaration) => declaration.implements_clause(),
            JsAnyClass::JsClassExpression(expression) => expression.implements_clause(),
            JsAnyClass::JsClassExportDefaultDeclaration(declaration) => {
                declaration.implements_clause()
            }
        }
    }

    pub fn l_curly_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            JsAnyClass::JsClassDeclaration(declaration) => declaration.l_curly_token(),
            JsAnyClass::JsClassExpression(expression) => expression.l_curly_token(),
            JsAnyClass::JsClassExportDefaultDeclaration(declaration) => declaration.l_curly_token(),
        }
    }

    pub fn members(&self) -> JsClassMemberList {
        match self {
            JsAnyClass::JsClassDeclaration(declaration) => declaration.members(),
            JsAnyClass::JsClassExpression(expression) => expression.members(),
            JsAnyClass::JsClassExportDefaultDeclaration(declaration) => declaration.members(),
        }
    }

    pub fn r_curly_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            JsAnyClass::JsClassDeclaration(declaration) => declaration.r_curly_token(),
            JsAnyClass::JsClassExpression(expression) => expression.r_curly_token(),
            JsAnyClass::JsClassExportDefaultDeclaration(declaration) => declaration.r_curly_token(),
        }
    }
}

impl JsAnyClassMember {
    pub fn name(&self) -> SyntaxResult<Option<JsAnyClassMemberName>> {
        match self {
            JsAnyClassMember::JsConstructorClassMember(constructor) => constructor
                .name()
                .map(|name| Some(JsAnyClassMemberName::from(name))),
            JsAnyClassMember::JsEmptyClassMember(_) => Ok(None),
            JsAnyClassMember::JsGetterClassMember(getter) => getter.name().map(Some),
            JsAnyClassMember::JsMethodClassMember(method) => method.name().map(Some),
            JsAnyClassMember::JsPropertyClassMember(property) => property.name().map(Some),
            JsAnyClassMember::JsSetterClassMember(setter) => setter.name().map(Some),
            JsAnyClassMember::JsStaticInitializationBlockClassMember(_) => Ok(None),
            JsAnyClassMember::JsUnknownMember(_) => Ok(None),
            JsAnyClassMember::TsConstructorSignatureClassMember(constructor) => constructor
                .name()
                .map(|name| Some(JsAnyClassMemberName::from(name))),
            JsAnyClassMember::TsGetterSignatureClassMember(getter) => getter.name().map(Some),
            JsAnyClassMember::TsIndexSignatureClassMember(_) => Ok(None),
            JsAnyClassMember::TsMethodSignatureClassMember(method) => method.name().map(Some),
            JsAnyClassMember::TsPropertySignatureClassMember(property) => property.name().map(Some),
            JsAnyClassMember::TsSetterSignatureClassMember(setter) => setter.name().map(Some),
        }
    }

    pub fn has_name(&self, name: &str) -> SyntaxResult<bool> {
        match self.name()? {
            Some(JsAnyClassMemberName::JsLiteralMemberName(literal)) => {
                Ok(literal.value()?.text_trimmed() == name)
            }
            _ => Ok(false),
        }
    }
}

impl JsAnyClassMemberName {
    pub const fn is_computed(&self) -> bool {
        matches!(self, JsAnyClassMemberName::JsComputedMemberName(_))
    }
}

impl JsAnyFunction {
    pub fn async_token(&self) -> Option<JsSyntaxToken> {
        match self {
            JsAnyFunction::JsArrowFunctionExpression(expr) => expr.async_token(),
            JsAnyFunction::JsFunctionExpression(expr) => expr.async_token(),
            JsAnyFunction::JsFunctionDeclaration(declaration) => declaration.async_token(),
            JsAnyFunction::JsFunctionExportDefaultDeclaration(declaration) => {
                declaration.async_token()
            }
        }
    }

    pub fn is_async(&self) -> bool {
        self.async_token().is_some()
    }

    pub fn function_token(&self) -> SyntaxResult<Option<JsSyntaxToken>> {
        match self {
            JsAnyFunction::JsArrowFunctionExpression(_) => Ok(None),
            JsAnyFunction::JsFunctionExpression(expr) => expr.function_token().map(Some),
            JsAnyFunction::JsFunctionDeclaration(declaration) => {
                declaration.function_token().map(Some)
            }
            JsAnyFunction::JsFunctionExportDefaultDeclaration(declaration) => {
                declaration.function_token().map(Some)
            }
        }
    }

    pub fn star_token(&self) -> Option<JsSyntaxToken> {
        match self {
            JsAnyFunction::JsArrowFunctionExpression(_) => None,
            JsAnyFunction::JsFunctionExpression(expr) => expr.star_token(),
            JsAnyFunction::JsFunctionDeclaration(declaration) => declaration.star_token(),
            JsAnyFunction::JsFunctionExportDefaultDeclaration(declaration) => {
                declaration.star_token()
            }
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
            JsAnyFunction::JsFunctionExportDefaultDeclaration(declaration) => Ok(declaration.id()),
        }
    }

    pub fn type_parameters(&self) -> Option<TsTypeParameters> {
        match self {
            JsAnyFunction::JsArrowFunctionExpression(expr) => expr.type_parameters(),
            JsAnyFunction::JsFunctionExpression(expr) => expr.type_parameters(),
            JsAnyFunction::JsFunctionDeclaration(declaration) => declaration.type_parameters(),
            JsAnyFunction::JsFunctionExportDefaultDeclaration(declaration) => {
                declaration.type_parameters()
            }
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
            JsAnyFunction::JsFunctionExportDefaultDeclaration(declaration) => declaration
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
            JsAnyFunction::JsFunctionExportDefaultDeclaration(declaration) => {
                declaration.return_type_annotation()
            }
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
            JsAnyFunction::JsFunctionExportDefaultDeclaration(declaration) => {
                declaration.body().map(JsAnyFunctionBody::JsFunctionBody)
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

impl JsAnyArrowFunctionParameters {
    pub fn len(&self) -> usize {
        match self {
            JsAnyArrowFunctionParameters::JsAnyBinding(_) => 1,
            JsAnyArrowFunctionParameters::JsParameters(parameters) => parameters.items().len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
