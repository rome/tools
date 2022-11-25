use crate::prelude::*;

use crate::js::declarations::function_declaration::should_group_function_parameters;
use rome_formatter::write;
use rome_js_syntax::{
    JsAnyClassMemberName, JsAnyObjectMemberName, JsConstructorClassMember, JsConstructorParameters,
    JsFunctionBody, JsParameters, TsMethodSignatureClassMember, TsMethodSignatureTypeMember,
    TsReturnTypeAnnotation, TsTypeParameters,
};
use rome_js_syntax::{JsMethodClassMember, JsMethodObjectMember, JsSyntaxToken};
use rome_rowan::{declare_node_union, SyntaxResult};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsMethodClassMember;

impl FormatNodeRule<JsMethodClassMember> for FormatJsMethodClassMember {
    fn fmt_fields(&self, node: &JsMethodClassMember, f: &mut JsFormatter) -> FormatResult<()> {
        write![
            f,
            [
                node.modifiers().format(),
                space(),
                FormatJsAnyMethodMember::from(node.clone())
            ]
        ]
    }
}

declare_node_union! {
    /// Formats the type parameters, parameters, and return type annotation of a method
    pub(crate) FormatJsAnyMethodMember =
        JsMethodClassMember |
        JsMethodObjectMember |
        JsConstructorClassMember |
        TsMethodSignatureClassMember |
        TsMethodSignatureTypeMember
}

impl Format<JsFormatContext> for FormatJsAnyMethodMember {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        if let Some(async_token) = self.async_token() {
            write!(f, [async_token.format(), space()])?;
        }

        let type_parameters = self.type_parameters();

        write!(
            f,
            [
                self.star_token().format(),
                self.name(),
                self.question_mark_token().format(),
                type_parameters.format(),
            ]
        )?;

        write!(
            f,
            [group(&format_with(|f| {
                let parameters = self.parameters()?;
                let return_type_annotation = self.return_type_annotation();
                let mut format_return_type_annotation = return_type_annotation.format().memoized();

                if should_group_function_parameters(
                    type_parameters.as_ref(),
                    parameters.len(),
                    return_type_annotation
                        .as_ref()
                        .map(|annotation| annotation.ty()),
                    &mut format_return_type_annotation,
                    f,
                )? {
                    write!(f, [group(&parameters)])?;
                } else {
                    write!(f, [parameters])?;
                }

                write!(f, [format_return_type_annotation])
            }))]
        )?;

        if let Some(body) = self.body()? {
            write!(f, [space(), body.format()])?;
        }

        Ok(())
    }
}

impl FormatJsAnyMethodMember {
    fn async_token(&self) -> Option<JsSyntaxToken> {
        match self {
            FormatJsAnyMethodMember::JsMethodClassMember(member) => member.async_token(),
            FormatJsAnyMethodMember::JsMethodObjectMember(member) => member.async_token(),
            FormatJsAnyMethodMember::JsConstructorClassMember(_) => None,
            FormatJsAnyMethodMember::TsMethodSignatureClassMember(signature) => {
                signature.async_token()
            }
            FormatJsAnyMethodMember::TsMethodSignatureTypeMember(_) => None,
        }
    }

    fn star_token(&self) -> Option<JsSyntaxToken> {
        match self {
            FormatJsAnyMethodMember::JsMethodClassMember(member) => member.star_token(),
            FormatJsAnyMethodMember::JsMethodObjectMember(member) => member.star_token(),
            FormatJsAnyMethodMember::JsConstructorClassMember(_) => None,
            FormatJsAnyMethodMember::TsMethodSignatureClassMember(_) => None,
            FormatJsAnyMethodMember::TsMethodSignatureTypeMember(_) => None,
        }
    }

    fn name(&self) -> SyntaxResult<AnyMemberName> {
        Ok(match self {
            FormatJsAnyMethodMember::JsMethodClassMember(member) => member.name()?.into(),
            FormatJsAnyMethodMember::JsMethodObjectMember(member) => member.name()?.into(),
            FormatJsAnyMethodMember::JsConstructorClassMember(member) => {
                AnyMemberName::from(JsAnyClassMemberName::from(member.name()?))
            }
            FormatJsAnyMethodMember::TsMethodSignatureClassMember(signature) => {
                signature.name()?.into()
            }
            FormatJsAnyMethodMember::TsMethodSignatureTypeMember(member) => member.name()?.into(),
        })
    }

    fn type_parameters(&self) -> Option<TsTypeParameters> {
        match self {
            FormatJsAnyMethodMember::JsMethodClassMember(member) => member.type_parameters(),
            FormatJsAnyMethodMember::JsMethodObjectMember(member) => member.type_parameters(),
            FormatJsAnyMethodMember::JsConstructorClassMember(_) => None,
            FormatJsAnyMethodMember::TsMethodSignatureClassMember(signature) => {
                signature.type_parameters()
            }
            FormatJsAnyMethodMember::TsMethodSignatureTypeMember(member) => {
                member.type_parameters()
            }
        }
    }

    fn parameters(&self) -> SyntaxResult<MethodParameters> {
        Ok(match self {
            FormatJsAnyMethodMember::JsMethodClassMember(member) => member.parameters()?.into(),
            FormatJsAnyMethodMember::JsMethodObjectMember(member) => member.parameters()?.into(),
            FormatJsAnyMethodMember::JsConstructorClassMember(member) => {
                member.parameters()?.into()
            }
            FormatJsAnyMethodMember::TsMethodSignatureClassMember(signature) => {
                signature.parameters()?.into()
            }
            FormatJsAnyMethodMember::TsMethodSignatureTypeMember(member) => {
                member.parameters()?.into()
            }
        })
    }

    fn return_type_annotation(&self) -> Option<TsReturnTypeAnnotation> {
        match self {
            FormatJsAnyMethodMember::JsMethodClassMember(member) => member.return_type_annotation(),
            FormatJsAnyMethodMember::JsMethodObjectMember(member) => {
                member.return_type_annotation()
            }
            FormatJsAnyMethodMember::JsConstructorClassMember(_) => None,
            FormatJsAnyMethodMember::TsMethodSignatureClassMember(signature) => {
                signature.return_type_annotation()
            }
            FormatJsAnyMethodMember::TsMethodSignatureTypeMember(member) => {
                member.return_type_annotation()
            }
        }
    }

    fn question_mark_token(&self) -> Option<JsSyntaxToken> {
        match self {
            FormatJsAnyMethodMember::JsMethodClassMember(member) => member.question_mark_token(),
            FormatJsAnyMethodMember::JsMethodObjectMember(_) => None,
            FormatJsAnyMethodMember::JsConstructorClassMember(_) => None,
            FormatJsAnyMethodMember::TsMethodSignatureClassMember(signature) => {
                signature.question_mark_token()
            }
            FormatJsAnyMethodMember::TsMethodSignatureTypeMember(member) => member.optional_token(),
        }
    }

    fn body(&self) -> SyntaxResult<Option<JsFunctionBody>> {
        Ok(match self {
            FormatJsAnyMethodMember::JsMethodClassMember(member) => Some(member.body()?),
            FormatJsAnyMethodMember::JsMethodObjectMember(member) => Some(member.body()?),
            FormatJsAnyMethodMember::JsConstructorClassMember(member) => Some(member.body()?),
            FormatJsAnyMethodMember::TsMethodSignatureClassMember(_) => None,
            FormatJsAnyMethodMember::TsMethodSignatureTypeMember(_) => None,
        })
    }
}

declare_node_union! {
     AnyMemberName = JsAnyClassMemberName | JsAnyObjectMemberName
}

impl Format<JsFormatContext> for AnyMemberName {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        match self {
            AnyMemberName::JsAnyClassMemberName(name) => name.format().fmt(f),
            AnyMemberName::JsAnyObjectMemberName(name) => name.format().fmt(f),
        }
    }
}

declare_node_union! {
    MethodParameters = JsParameters | JsConstructorParameters
}

impl MethodParameters {
    pub fn len(&self) -> usize {
        match self {
            MethodParameters::JsParameters(parameters) => parameters.items().len(),
            MethodParameters::JsConstructorParameters(parameters) => parameters.parameters().len(),
        }
    }
}

impl Format<JsFormatContext> for MethodParameters {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        match self {
            MethodParameters::JsParameters(parameters) => parameters.format().fmt(f),
            MethodParameters::JsConstructorParameters(parameters) => parameters.format().fmt(f),
        }
    }
}
