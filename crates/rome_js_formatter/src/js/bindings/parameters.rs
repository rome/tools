use crate::prelude::*;
use rome_formatter::{write, CstFormatContext};

use crate::js::lists::parameter_list::{
    AnyParameter, FormatJsAnyParameterList, JsAnyParameterList,
};

use crate::utils::test_call::is_test_call_argument;
use rome_js_syntax::{
    JsAnyConstructorParameter, JsAnyFormalParameter, JsConstructorParameters, JsParameters,
    JsSyntaxToken, TsType,
};
use rome_rowan::{declare_node_union, SyntaxResult};

#[derive(Debug, Clone, Default)]
pub struct FormatJsParameters;

impl FormatNodeRule<JsParameters> for FormatJsParameters {
    fn fmt_fields(&self, node: &JsParameters, f: &mut JsFormatter) -> FormatResult<()> {
        FormatJsAnyParameters::from(node.clone()).fmt(f)
    }

    fn fmt_dangling_comments(&self, _: &JsParameters, _: &mut JsFormatter) -> FormatResult<()> {
        // Formatted inside of `FormatJsAnyParameters
        Ok(())
    }
}

declare_node_union! {
    pub(crate) FormatJsAnyParameters = JsParameters | JsConstructorParameters
}

impl Format<JsFormatContext> for FormatJsAnyParameters {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        let list = self.list();

        let has_any_decorated_parameter = list.iter().any(|node| match node {
            Ok(node) => node.syntax().first_token().map_or(false, |token| {
                token
                    .leading_trivia()
                    .pieces()
                    .any(|piece| piece.is_skipped())
            }),
            Err(_) => false,
        });

        let can_hug = should_hug_function_parameters(self, f.context().comments())?
            && !has_any_decorated_parameter;

        let layout = if list.is_empty() {
            ParameterLayout::NoParameters
        } else if can_hug || self.is_in_test_call()? {
            ParameterLayout::Hug
        } else {
            ParameterLayout::Default
        };

        let l_paren_token = self.l_paren_token()?;
        let r_paren_token = self.r_paren_token()?;

        match layout {
            ParameterLayout::NoParameters => {
                write!(
                    f,
                    [
                        l_paren_token.format(),
                        format_dangling_comments(self.syntax()).with_soft_block_indent(),
                        r_paren_token.format()
                    ]
                )
            }
            ParameterLayout::Hug => {
                write!(
                    f,
                    [
                        l_paren_token.format(),
                        FormatJsAnyParameterList::with_layout(&list, ParameterLayout::Hug),
                        &r_paren_token.format()
                    ]
                )
            }
            ParameterLayout::Default => {
                write!(
                    f,
                    [
                        l_paren_token.format(),
                        soft_block_indent(&FormatJsAnyParameterList::with_layout(
                            &list,
                            ParameterLayout::Default
                        )),
                        r_paren_token.format()
                    ]
                )
            }
        }
    }
}

impl FormatJsAnyParameters {
    fn l_paren_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            FormatJsAnyParameters::JsParameters(parameters) => parameters.l_paren_token(),
            FormatJsAnyParameters::JsConstructorParameters(parameters) => {
                parameters.l_paren_token()
            }
        }
    }

    fn list(&self) -> JsAnyParameterList {
        match self {
            FormatJsAnyParameters::JsParameters(parameters) => {
                JsAnyParameterList::from(parameters.items())
            }
            FormatJsAnyParameters::JsConstructorParameters(parameters) => {
                JsAnyParameterList::from(parameters.parameters())
            }
        }
    }

    fn r_paren_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            FormatJsAnyParameters::JsParameters(parameters) => parameters.r_paren_token(),
            FormatJsAnyParameters::JsConstructorParameters(parameters) => {
                parameters.r_paren_token()
            }
        }
    }

    /// Returns `true` for function parameters if the function is an argument of a [test `CallExpression`](is_test_call_expression).
    fn is_in_test_call(&self) -> SyntaxResult<bool> {
        let result = match self {
            FormatJsAnyParameters::JsParameters(parameters) => match parameters.syntax().parent() {
                Some(function) => is_test_call_argument(&function)?,
                None => false,
            },
            FormatJsAnyParameters::JsConstructorParameters(_) => false,
        };

        Ok(result)
    }
}

#[derive(Copy, Debug, Clone, Eq, PartialEq)]
pub enum ParameterLayout {
    /// ```javascript
    /// function test() {}
    /// ```
    NoParameters,

    /// Enforce that the opening and closing parentheses aren't separated from the first token of the parameter.
    /// For example, to enforce that the `{`  and `}` of an object expression are formatted on the same line
    /// as the `(` and `)` tokens even IF the object expression itself breaks across multiple lines.
    ///
    /// ```javascript
    /// function test({
    ///     aVeryLongObjectBinding,
    ///     thatContinuesAndExceeds,
    ///     theLineWidth
    /// }) {}
    /// ```
    Hug,

    /// The default layout formats all parameters on the same line if they fit or breaks after the `(`
    /// and before the `(`.
    /// ```javascript
    /// function test(
    ///     firstParameter,
    ///     secondParameter,
    ///     thirdParameter
    /// ) {}
    /// ```
    Default,
}

pub(crate) fn should_hug_function_parameters(
    parameters: &FormatJsAnyParameters,
    comments: &JsComments,
) -> FormatResult<bool> {
    use rome_js_syntax::{
        JsAnyBinding::*, JsAnyBindingPattern::*, JsAnyExpression::*, JsAnyFormalParameter::*,
        JsAnyParameter::*,
    };

    let list = parameters.list();

    if list.len() != 1 {
        return Ok(false);
    }

    // SAFETY: Safe because of the length check above
    let only_parameter = list.first().unwrap()?;

    if comments.has_comments(only_parameter.syntax()) {
        return Ok(false);
    }

    /// Returns true if the first parameter should be forced onto the same line as the `(` and `)` parentheses.
    /// See the `[ParameterLayout::Hug] documentation.
    fn hug_formal_parameter(parameter: &self::JsAnyFormalParameter) -> FormatResult<bool> {
        let result = match parameter {
            JsFormalParameter(parameter) => {
                match parameter.initializer() {
                    None => {
                        match parameter.binding()? {
                            // always true for `[a]` or `{a}`
                            JsArrayBindingPattern(_) | JsObjectBindingPattern(_) => true,
                            // only if the type parameter is an object type
                            // `a: { prop: string }`
                            JsAnyBinding(JsIdentifierBinding(_)) => parameter
                                .type_annotation()
                                .map_or(false, |type_annotation| {
                                    matches!(type_annotation.ty(), Ok(TsType::TsObjectType(_)))
                                }),
                            JsAnyBinding(JsUnknownBinding(_)) => {
                                return Err(FormatDiagnostic::SyntaxError)
                            }
                        }
                    }

                    Some(initializer) => {
                        // only for `[a] = []`, `{a} = {}`
                        let object_or_array_binding = matches!(
                            parameter.binding()?,
                            JsArrayBindingPattern(_) | JsObjectBindingPattern(_)
                        );
                        let should_hug_right = match initializer.expression()? {
                            JsObjectExpression(object) => object.members().is_empty(),
                            JsArrayExpression(array) => array.elements().is_empty(),
                            JsIdentifierExpression(_) => true,
                            _ => false,
                        };

                        object_or_array_binding && should_hug_right
                    }
                }
            }
            JsUnknownParameter(_) => return Err(FormatDiagnostic::SyntaxError),
        };

        Ok(result)
    }

    let result = match only_parameter {
        AnyParameter::JsAnyParameter(parameter) => match parameter {
            JsAnyFormalParameter(formal_parameter) => hug_formal_parameter(&formal_parameter)?,
            JsRestParameter(_) => false,
            TsThisParameter(this) => this.type_annotation().map_or(false, |type_annotation| {
                matches!(type_annotation.ty(), Ok(TsType::TsObjectType(_)))
            }),
        },
        AnyParameter::JsAnyConstructorParameter(constructor_parameter) => {
            match constructor_parameter {
                JsAnyConstructorParameter::JsAnyFormalParameter(formal_parameter) => {
                    hug_formal_parameter(&formal_parameter)?
                }
                JsAnyConstructorParameter::JsRestParameter(_)
                | JsAnyConstructorParameter::TsPropertyParameter(_) => false,
            }
        }
    };

    Ok(result)
}
