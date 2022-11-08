//! Extensions for things which are not easily generated in ast expr nodes
use crate::numbers::parse_js_number;
use crate::{
    JsAnyCallArgument, JsAnyExpression, JsAnyLiteralExpression, JsAnyTemplateElement,
    JsArrayExpression, JsArrayHole, JsAssignmentExpression, JsBinaryExpression, JsCallExpression,
    JsComputedMemberExpression, JsIdentifierExpression, JsLiteralMemberName, JsLogicalExpression,
    JsNewExpression, JsNumberLiteralExpression, JsObjectExpression, JsPostUpdateExpression,
    JsReferenceIdentifier, JsRegexLiteralExpression, JsStaticMemberExpression,
    JsStringLiteralExpression, JsSyntaxKind, JsSyntaxToken, JsTemplate, JsUnaryExpression,
    OperatorPrecedence, T,
};
use crate::{JsPreUpdateExpression, JsSyntaxKind::*};
use core::iter;
use rome_rowan::{
    AstNode, AstSeparatedList, NodeOrToken, SyntaxResult, SyntaxTokenText, TextRange, TextSize,
};
use std::collections::HashSet;

impl JsReferenceIdentifier {
    /// Returns `true` if this identifier refers to the `undefined` symbol.
    ///
    /// ## Examples
    ///
    /// ```
    /// use rome_js_factory::make::{js_reference_identifier, ident};
    ///
    /// assert!(js_reference_identifier(ident("undefined")).is_undefined());
    /// assert!(!js_reference_identifier(ident("x")).is_undefined());
    /// ```
    pub fn is_undefined(&self) -> bool {
        self.has_name("undefined")
    }

    /// Returns `true` if this identifier has the given name.
    ///
    /// ## Examples
    ///
    /// ```
    /// use rome_js_factory::make::{js_reference_identifier, ident};
    ///
    /// assert!(js_reference_identifier(ident("foo")).has_name("foo"));
    /// assert!(!js_reference_identifier(ident("bar")).has_name("foo"));
    /// ```
    pub fn has_name(&self, name: &str) -> bool {
        self.value_token()
            .map(|token| token.text_trimmed() == name)
            .unwrap_or_default()
    }
}

impl JsLiteralMemberName {
    /// Returns the name of the member as a syntax text
    ///
    /// ## Examples
    ///
    /// Getting the name of a static member containing a string literal
    ///
    /// ```
    /// use rome_js_syntax::{JsSyntaxKind, JsLanguage, JsSyntaxNode, JsLiteralMemberName};
    /// use rome_js_factory::JsSyntaxTreeBuilder;
    /// use rome_rowan::AstNode;
    ///
    /// let node: JsSyntaxNode =
    ///     JsSyntaxTreeBuilder::wrap_with_node(JsSyntaxKind::JS_LITERAL_MEMBER_NAME, |builder| {
    ///         builder.token(JsSyntaxKind::JS_STRING_LITERAL, "\"abcd\"");
    ///     });
    ///
    /// let static_member_name = JsLiteralMemberName::unwrap_cast(node);
    ///
    /// assert_eq!("abcd", static_member_name.name().unwrap());
    /// ```
    ///
    /// Getting the name of a static member containing a number literal
    ///
    /// ```
    /// use rome_js_syntax::{JsSyntaxKind, JsLanguage, JsSyntaxNode, JsLiteralMemberName};
    /// use rome_js_factory::JsSyntaxTreeBuilder;
    /// use rome_rowan::AstNode;
    ///
    /// let node: JsSyntaxNode =
    ///     JsSyntaxTreeBuilder::wrap_with_node(JsSyntaxKind::JS_LITERAL_MEMBER_NAME, |builder| {
    ///         builder.token(JsSyntaxKind::JS_NUMBER_LITERAL, "5");
    ///     });
    ///
    /// let static_member_name = JsLiteralMemberName::unwrap_cast(node);
    ///
    /// assert_eq!("5", static_member_name.name().unwrap());
    /// ```
    ///
    /// Getting the name of a static member containing an identifier
    ///
    /// ```
    /// use rome_js_syntax::{JsSyntaxKind, JsLanguage, JsSyntaxNode, JsLiteralMemberName};
    /// use rome_js_factory::JsSyntaxTreeBuilder;
    /// use rome_rowan::AstNode;
    ///
    /// let node: JsSyntaxNode =
    ///     JsSyntaxTreeBuilder::wrap_with_node(JsSyntaxKind::JS_LITERAL_MEMBER_NAME, |builder| {
    ///         builder.token(JsSyntaxKind::IDENT, "abcd");
    ///     });
    ///
    /// let static_member_name = JsLiteralMemberName::unwrap_cast(node);
    ///
    /// assert_eq!("abcd", static_member_name.name().unwrap());
    /// ```
    pub fn name(&self) -> SyntaxResult<String> {
        let value = self.value()?;
        let name = value.text_trimmed();

        let result = match value.kind() {
            JS_STRING_LITERAL => String::from(&name[1..name.len() - 1]),
            _ => String::from(name),
        };

        Ok(result)
    }
}

/// A binary operation applied to two expressions
///
/// The variants are ordered based on their precedence
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum JsBinaryOperator {
    /// `<`
    LessThan,
    /// `>`
    GreaterThan,
    /// `<=`
    LessThanOrEqual,
    /// `>=`
    GreaterThanOrEqual,
    /// `==`
    Equality,
    /// `===`
    StrictEquality,
    /// `!=`
    Inequality,
    /// `!==`
    StrictInequality,
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `*`
    Times,
    /// `/`
    Divide,
    /// `%`
    Remainder,
    /// `**`
    Exponent,
    /// `<<`
    LeftShift,
    /// `>>`
    RightShift,
    /// `>>>`
    UnsignedRightShift,
    /// `&`
    BitwiseAnd,
    /// `|`
    BitwiseOr,
    /// `^`
    BitwiseXor,
}

impl JsBinaryOperator {
    pub const fn precedence(&self) -> OperatorPrecedence {
        match self {
            JsBinaryOperator::LessThan
            | JsBinaryOperator::GreaterThan
            | JsBinaryOperator::LessThanOrEqual
            | JsBinaryOperator::GreaterThanOrEqual => OperatorPrecedence::Relational,

            JsBinaryOperator::Equality
            | JsBinaryOperator::StrictEquality
            | JsBinaryOperator::Inequality
            | JsBinaryOperator::StrictInequality => OperatorPrecedence::Equality,

            JsBinaryOperator::Plus | JsBinaryOperator::Minus => OperatorPrecedence::Additive,

            JsBinaryOperator::Times | JsBinaryOperator::Divide | JsBinaryOperator::Remainder => {
                OperatorPrecedence::Multiplicative
            }
            JsBinaryOperator::Exponent => OperatorPrecedence::Exponential,

            JsBinaryOperator::LeftShift
            | JsBinaryOperator::RightShift
            | JsBinaryOperator::UnsignedRightShift => OperatorPrecedence::Shift,

            JsBinaryOperator::BitwiseAnd => OperatorPrecedence::BitwiseAnd,
            JsBinaryOperator::BitwiseOr => OperatorPrecedence::BitwiseOr,
            JsBinaryOperator::BitwiseXor => OperatorPrecedence::BitwiseXor,
        }
    }
}

impl JsBinaryExpression {
    pub fn operator(&self) -> SyntaxResult<JsBinaryOperator> {
        let kind = match self.operator_token()?.kind() {
            T![<] => JsBinaryOperator::LessThan,
            T![>] => JsBinaryOperator::GreaterThan,
            T![<=] => JsBinaryOperator::LessThanOrEqual,
            T![>=] => JsBinaryOperator::GreaterThanOrEqual,
            T![==] => JsBinaryOperator::Equality,
            T![===] => JsBinaryOperator::StrictEquality,
            T![!=] => JsBinaryOperator::Inequality,
            T![!==] => JsBinaryOperator::StrictInequality,
            T![+] => JsBinaryOperator::Plus,
            T![-] => JsBinaryOperator::Minus,
            T![*] => JsBinaryOperator::Times,
            T![/] => JsBinaryOperator::Divide,
            T![%] => JsBinaryOperator::Remainder,
            T![**] => JsBinaryOperator::Exponent,
            T![<<] => JsBinaryOperator::LeftShift,
            T![>>] => JsBinaryOperator::RightShift,
            T![>>>] => JsBinaryOperator::UnsignedRightShift,
            T![&] => JsBinaryOperator::BitwiseAnd,
            T![|] => JsBinaryOperator::BitwiseOr,
            T![^] => JsBinaryOperator::BitwiseXor,
            _ => unreachable!(),
        };

        Ok(kind)
    }

    /// Whether this is a comparison operation, such as `>`, `<`, `==`, `!=`, `===`, etc.
    pub fn is_comparison_operator(&self) -> bool {
        matches!(
            self.operator_token().map(|t| t.kind()),
            Ok(T![>] | T![<] | T![>=] | T![<=] | T![==] | T![===] | T![!=] | T![!==])
        )
    }

    /// Whether this is a comparison operation similar to the optional chain
    /// ```js
    /// foo === undefined;
    /// foo == undefined;
    /// foo === null;
    /// foo == null;
    ///```
    pub fn is_optional_chain_like(&self) -> SyntaxResult<bool> {
        if matches!(
            self.operator(),
            Ok(JsBinaryOperator::StrictInequality | JsBinaryOperator::Inequality)
        ) {
            let right = self.right()?;

            let is_right_null_expression = right
                .as_js_any_literal_expression()
                .map_or(false, |expression| {
                    expression.as_js_null_literal_expression().is_some()
                });

            if is_right_null_expression {
                return Ok(true);
            }

            let is_right_undefined_expression = right
                .as_js_identifier_expression()
                .map(|expression| expression.is_undefined())
                .transpose()?
                .unwrap_or(false);

            if is_right_undefined_expression {
                return Ok(true);
            }
        }
        Ok(false)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum JsLogicalOperator {
    /// `??`
    NullishCoalescing,
    /// `||`
    LogicalOr,
    /// `&&`
    LogicalAnd,
}

impl JsLogicalOperator {
    pub const fn precedence(&self) -> OperatorPrecedence {
        match self {
            JsLogicalOperator::NullishCoalescing => OperatorPrecedence::Coalesce,
            JsLogicalOperator::LogicalOr => OperatorPrecedence::LogicalOr,
            JsLogicalOperator::LogicalAnd => OperatorPrecedence::LogicalAnd,
        }
    }
}

impl JsLogicalExpression {
    pub fn operator(&self) -> SyntaxResult<JsLogicalOperator> {
        let kind = match self.operator_token()?.kind() {
            T![&&] => JsLogicalOperator::LogicalAnd,
            T![||] => JsLogicalOperator::LogicalOr,
            T![??] => JsLogicalOperator::NullishCoalescing,
            _ => unreachable!(),
        };

        Ok(kind)
    }
}

impl JsArrayHole {
    pub fn hole_token(&self) -> Option<JsSyntaxToken> {
        None
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum JsUnaryOperator {
    /// `delete`
    Delete,
    /// `void`
    Void,
    /// `typeof`
    Typeof,
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `~`
    BitwiseNot,
    /// `!`
    LogicalNot,
}

impl JsUnaryOperator {
    pub const fn precedence(&self) -> OperatorPrecedence {
        OperatorPrecedence::Unary
    }
}

impl JsUnaryExpression {
    pub fn operator(&self) -> SyntaxResult<JsUnaryOperator> {
        let operator = self.operator_token()?;

        Ok(match operator.kind() {
            T![+] => JsUnaryOperator::Plus,
            T![-] => JsUnaryOperator::Minus,
            T![~] => JsUnaryOperator::BitwiseNot,
            T![!] => JsUnaryOperator::LogicalNot,
            T![typeof] => JsUnaryOperator::Typeof,
            T![void] => JsUnaryOperator::Void,
            T![delete] => JsUnaryOperator::Delete,
            _ => unreachable!(),
        })
    }

    /// This function checks that `JsUnaryExpression` is a signed numeric literal:
    /// ```js
    ///     +123
    ///     -321
    /// ```
    pub fn is_signed_numeric_literal(&self) -> SyntaxResult<bool> {
        let argument = self.argument()?;

        let is_signed = matches!(
            self.operator()?,
            JsUnaryOperator::Plus | JsUnaryOperator::Minus
        );

        let is_numeric_literal = matches!(
            argument,
            JsAnyExpression::JsAnyLiteralExpression(
                JsAnyLiteralExpression::JsNumberLiteralExpression(_)
            )
        );

        Ok(is_signed && is_numeric_literal)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum JsPreUpdateOperator {
    /// `++`
    Increment,
    /// `--`
    Decrement,
}

impl JsPreUpdateOperator {
    pub const fn precedence(&self) -> OperatorPrecedence {
        OperatorPrecedence::Unary
    }
}

impl JsPreUpdateExpression {
    pub fn operator(&self) -> SyntaxResult<JsPreUpdateOperator> {
        let operator = self.operator_token()?;

        Ok(match operator.kind() {
            T![++] => JsPreUpdateOperator::Increment,
            T![--] => JsPreUpdateOperator::Decrement,
            _ => unreachable!(),
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum JsPostUpdateOperator {
    /// `++`
    Increment,
    /// `--`
    Decrement,
}

impl JsPostUpdateOperator {
    pub const fn precedence(&self) -> OperatorPrecedence {
        OperatorPrecedence::Unary
    }
}

impl JsPostUpdateExpression {
    pub fn operator(&self) -> SyntaxResult<JsPostUpdateOperator> {
        let operator = self.operator_token()?;

        Ok(match operator.kind() {
            T![++] => JsPostUpdateOperator::Increment,
            T![--] => JsPostUpdateOperator::Decrement,
            _ => unreachable!(),
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum JsAssignmentOperator {
    Assign,
    AddAssign,
    SubtractAssign,
    TimesAssign,
    RemainderAssign,
    ExponentAssign,
    LeftShiftAssign,
    RightShiftAssign,
    UnsignedRightShiftAssign,
    BitwiseAndAssign,
    BitwiseOrAssign,
    BitwiseXorAssign,
    LogicalAndAssign,
    LogicalOrAssign,
    NullishCoalescingAssign,
}

impl JsAssignmentExpression {
    pub fn operator(&self) -> SyntaxResult<JsAssignmentOperator> {
        let operator = match self.operator_token()?.kind() {
            T![=] => JsAssignmentOperator::Assign,
            T![+=] => JsAssignmentOperator::AddAssign,
            T![-=] => JsAssignmentOperator::SubtractAssign,
            T![*=] => JsAssignmentOperator::TimesAssign,
            T![%=] => JsAssignmentOperator::RemainderAssign,
            T![**=] => JsAssignmentOperator::ExponentAssign,
            T![>>=] => JsAssignmentOperator::LeftShiftAssign,
            T![<<=] => JsAssignmentOperator::RightShiftAssign,
            T![>>>=] => JsAssignmentOperator::UnsignedRightShiftAssign,
            T![&=] => JsAssignmentOperator::BitwiseAndAssign,
            T![|=] => JsAssignmentOperator::BitwiseOrAssign,
            T![^=] => JsAssignmentOperator::BitwiseXorAssign,
            T![&&=] => JsAssignmentOperator::LogicalAndAssign,
            T![||=] => JsAssignmentOperator::LogicalOrAssign,
            T![??=] => JsAssignmentOperator::NullishCoalescingAssign,
            _ => unreachable!(),
        };

        Ok(operator)
    }
}

impl JsArrayExpression {
    pub fn has_trailing_comma(&self) -> bool {
        self.elements().trailing_separator().is_some()
    }
}

impl JsObjectExpression {
    pub fn has_trailing_comma(&self) -> bool {
        self.members().trailing_separator().is_some()
    }

    pub fn is_empty(&self) -> bool {
        self.members().is_empty()
    }
}

impl JsNumberLiteralExpression {
    pub fn as_number(&self) -> Option<f64> {
        parse_js_number(self.value_token().unwrap().text())
    }
}

impl JsStringLiteralExpression {
    /// Get the inner text of a string not including the quotes
    ///
    /// ## Examples
    ///
    /// ```
    /// use rome_js_factory::make::{js_string_literal_expression, ident};
    /// use rome_rowan::TriviaPieceKind;
    ///
    ///let string = js_string_literal_expression(ident("foo")
    ///     .with_leading_trivia(vec![(TriviaPieceKind::Whitespace, " ")]));
    /// assert_eq!(string.inner_string_text().unwrap().text(), "foo");
    /// ```
    pub fn inner_string_text(&self) -> SyntaxResult<SyntaxTokenText> {
        let value = self.value_token()?;
        let mut text = value.token_text_trimmed();

        static QUOTES: [char; 2] = ['"', '\''];

        if text.starts_with(QUOTES) {
            let range = text.range().add_start(TextSize::from(1));
            text = text.slice(range);
        }

        if text.ends_with(QUOTES) {
            let range = text.range().sub_end(TextSize::from(1));
            text = text.slice(range);
        }

        Ok(text)
    }
}

impl JsTemplate {
    /// The string chunks of the template. aka:
    /// `foo ${bar} foo` breaks down into:
    /// `QUASIS ELEMENT{EXPR} QUASIS`
    pub fn quasis(&self) -> impl Iterator<Item = JsSyntaxToken> {
        self.syntax()
            .children_with_tokens()
            .filter_map(NodeOrToken::into_token)
            .filter(|t| t.kind() == TEMPLATE_CHUNK)
    }

    pub fn template_range(&self) -> Option<TextRange> {
        let start = self
            .syntax()
            .children_with_tokens()
            .filter_map(|x| x.into_token())
            .find(|tok| tok.kind() == BACKTICK)?;
        Some(TextRange::new(
            start.text_range().start(),
            self.syntax().text_range().end(),
        ))
    }
}

impl JsRegexLiteralExpression {
    pub fn pattern(&self) -> SyntaxResult<String> {
        let token = self.value_token()?;
        let text_trimmed = token.text_trimmed();

        // SAFETY: a valid regex literal must have a end slash
        let end_slash_pos = text_trimmed.rfind('/').unwrap();

        Ok(String::from(&text_trimmed[1..end_slash_pos]))
    }
}

impl JsAnyExpression {
    /// Try to extract non `JsParenthesizedExpression` from `JsAnyExpression`
    pub fn omit_parentheses(self) -> JsAnyExpression {
        let first = self
            .as_js_parenthesized_expression()
            .and_then(|expression| expression.expression().ok());

        iter::successors(first, |expression| {
            let parenthesized = expression.as_js_parenthesized_expression()?;
            parenthesized.expression().ok()
        })
        .last()
        .unwrap_or(self)
    }

    pub fn precedence(&self) -> SyntaxResult<OperatorPrecedence> {
        let precedence = match self {
            JsAnyExpression::JsSequenceExpression(_) => OperatorPrecedence::Comma,
            JsAnyExpression::JsYieldExpression(_) => OperatorPrecedence::Yield,
            JsAnyExpression::JsConditionalExpression(_) => OperatorPrecedence::Conditional,
            JsAnyExpression::JsAssignmentExpression(_) => OperatorPrecedence::Assignment,
            JsAnyExpression::JsInExpression(_)
            | JsAnyExpression::JsInstanceofExpression(_)
            | JsAnyExpression::TsAsExpression(_) => OperatorPrecedence::Relational,
            JsAnyExpression::JsLogicalExpression(expression) => expression.operator()?.precedence(),
            JsAnyExpression::JsBinaryExpression(expression) => expression.operator()?.precedence(),
            JsAnyExpression::TsTypeAssertionExpression(_)
            | JsAnyExpression::TsNonNullAssertionExpression(_)
            | JsAnyExpression::JsUnaryExpression(_)
            | JsAnyExpression::JsAwaitExpression(_) => OperatorPrecedence::Unary,
            JsAnyExpression::JsPostUpdateExpression(_)
            | JsAnyExpression::JsPreUpdateExpression(_) => OperatorPrecedence::Update,
            JsAnyExpression::JsCallExpression(_)
            | JsAnyExpression::JsImportCallExpression(_)
            | JsAnyExpression::JsSuperExpression(_) => OperatorPrecedence::LeftHandSide,

            JsAnyExpression::JsNewExpression(expression) => {
                if expression.arguments().is_none() {
                    OperatorPrecedence::NewWithoutArguments
                } else {
                    OperatorPrecedence::LeftHandSide
                }
            }
            JsAnyExpression::JsComputedMemberExpression(_)
            | JsAnyExpression::JsStaticMemberExpression(_)
            | JsAnyExpression::ImportMeta(_)
            | JsAnyExpression::TsInstantiationExpression(_)
            | JsAnyExpression::NewTarget(_) => OperatorPrecedence::Member,

            JsAnyExpression::JsThisExpression(_)
            | JsAnyExpression::JsAnyLiteralExpression(_)
            | JsAnyExpression::JsArrayExpression(_)
            | JsAnyExpression::JsArrowFunctionExpression(_)
            | JsAnyExpression::JsClassExpression(_)
            | JsAnyExpression::JsFunctionExpression(_)
            | JsAnyExpression::JsIdentifierExpression(_)
            | JsAnyExpression::JsObjectExpression(_)
            | JsAnyExpression::JsxTagExpression(_) => OperatorPrecedence::Primary,

            JsAnyExpression::JsTemplate(template) => {
                if template.tag().is_some() {
                    OperatorPrecedence::Member
                } else {
                    OperatorPrecedence::Primary
                }
            }

            JsAnyExpression::JsUnknownExpression(_) => OperatorPrecedence::lowest(),
            JsAnyExpression::JsParenthesizedExpression(_) => OperatorPrecedence::highest(),
        };

        Ok(precedence)
    }
}

impl JsIdentifierExpression {
    pub fn is_undefined(&self) -> SyntaxResult<bool> {
        Ok(self.name()?.value_token()?.text_trimmed() == "undefined")
    }
}

impl JsAnyLiteralExpression {
    pub fn value_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            JsAnyLiteralExpression::JsBigIntLiteralExpression(expression) => {
                expression.value_token()
            }
            JsAnyLiteralExpression::JsBooleanLiteralExpression(expression) => {
                expression.value_token()
            }
            JsAnyLiteralExpression::JsNullLiteralExpression(expression) => expression.value_token(),
            JsAnyLiteralExpression::JsNumberLiteralExpression(expression) => {
                expression.value_token()
            }
            JsAnyLiteralExpression::JsRegexLiteralExpression(expression) => {
                expression.value_token()
            }
            JsAnyLiteralExpression::JsStringLiteralExpression(expression) => {
                expression.value_token()
            }
        }
    }
}

impl JsStaticMemberExpression {
    /// Returns `true` if this is an optional member access
    ///
    /// ```javascript
    /// a.b -> false,
    /// a?.b -> true
    /// a?.[b][c][d].e -> false
    /// ```
    pub fn is_optional(&self) -> bool {
        self.operator_token()
            .map_or(false, |token| token.kind() == JsSyntaxKind::QUESTIONDOT)
    }

    /// Returns true if this member has an optional token or any member expression on the left side.
    ///
    /// ```javascript
    /// a.b -> false
    /// a?.b-> true
    /// a?.[b][c][d].e -> true
    /// ```
    pub fn is_optional_chain(&self) -> bool {
        is_optional_chain(self.clone().into())
    }
}

impl JsComputedMemberExpression {
    /// Returns `true` if this is an optional member access
    ///
    /// ```javascript
    /// a[b] -> false,
    /// a?.[b] -> true
    /// a?.b.c.d[e] -> false
    /// ```
    pub fn is_optional(&self) -> bool {
        self.optional_chain_token().is_some()
    }

    /// Returns true if this member has an optional token or any member expression on the left side.
    ///
    /// ```javascript
    /// a[b] -> false
    /// a?.[b]-> true
    /// a?.b.c.d[e] -> true
    /// ```
    pub fn is_optional_chain(&self) -> bool {
        is_optional_chain(self.clone().into())
    }
}

impl JsCallExpression {
    /// Returns `true` if this is an optional member access
    ///
    /// ```javascript
    /// a() -> false,
    /// a?.() -> true
    /// a?.b() -> false
    /// ```
    pub fn is_optional(&self) -> bool {
        self.optional_chain_token().is_some()
    }

    /// Returns true if this member has an optional token or any member expression on the left side.
    ///
    /// ```javascript
    /// a() -> false
    /// a?.()-> true
    /// a?.b.c.d() -> true
    /// ```
    pub fn is_optional_chain(&self) -> bool {
        is_optional_chain(self.clone().into())
    }

    /// Get [JsAnyCallArgument] by it index inside the [JsCallExpression] argument list.  
    ///
    /// Each index inside "indices" should be unique.  
    /// "indices" must be sorted.  
    ///
    /// Supports maximum of 16 indices to avoid stack overflow. Eeach argument will consume:
    ///
    /// - 8 bytes for the [Option<JsAnyCallArgument>] result;
    /// - 8 bytes for the [usize] argument.
    pub fn get_arguments_by_index<const N: usize>(
        &self,
        indices: [usize; N],
    ) -> [Option<JsAnyCallArgument>; N] {
        // assert there are no duplicates
        debug_assert!(HashSet::<_>::from_iter(indices).len() == N);
        debug_assert!({
            // is_sorted is unstable
            let mut sorted = indices;
            sorted.sort();
            indices == sorted
        });
        debug_assert!(N <= 16);

        const INIT: Option<JsAnyCallArgument> = None;
        let mut results = [INIT; N];

        let mut next = 0;

        for (i, arg) in self
            .arguments()
            .ok()
            .map(|x| x.args().into_iter())
            .into_iter()
            .flatten()
            .enumerate()
        {
            if i == indices[next] {
                results[next] = arg.ok();
                next += 1;
            }
        }

        results
    }

    /// Check if the callee is an identifier with given name
    pub fn has_callee(&self, matcher: impl FnOnce(JsReferenceIdentifier) -> bool) -> bool {
        self.callee()
            .map(|it| it.is_ident_deep(matcher))
            .unwrap_or(false)
    }
}

impl JsNewExpression {
    /// Check if the callee is an identifier with given name
    pub fn has_callee(&self, matcher: impl FnOnce(JsReferenceIdentifier) -> bool) -> bool {
        self.callee()
            .map(|it| it.is_ident_deep(matcher))
            .unwrap_or(false)
    }
}

impl JsAnyExpression {
    /// Check if expression is identifier with given name.
    pub fn is_ident(&self, matcher: impl FnOnce(JsReferenceIdentifier) -> bool) -> bool {
        self.as_js_identifier_expression()
            .and_then(|it| it.name().ok())
            .map(|it| matcher(it))
            .unwrap_or(false)
    }

    /// Check if expression is identifier with given name, optionally wrapped in parentheses.
    pub fn is_ident_deep(self, matcher: impl FnOnce(JsReferenceIdentifier) -> bool) -> bool {
        self.omit_parentheses().is_ident(matcher)
    }
}

impl JsAnyExpression {
    /// Check if the given expression is a static or computed member expression
    /// with given object name matcher and member name.
    pub fn is_member_access(
        &self,
        object: impl FnOnce(JsReferenceIdentifier) -> bool,
        member: &str,
    ) -> bool {
        let expr = self.clone().omit_parentheses();
        match expr {
            JsAnyExpression::JsStaticMemberExpression(e) => {
                e.has_object_name(object) && e.has_member_name(member)
            }
            JsAnyExpression::JsComputedMemberExpression(e) => {
                e.has_object_name(object) && e.has_member_name(member)
            }
            _ => false,
        }
    }
}

impl JsStaticMemberExpression {
    /// Check if the object in the expression has the given name, optionally with parentheses
    pub fn has_object_name(&self, name: impl FnOnce(JsReferenceIdentifier) -> bool) -> bool {
        self.object()
            .map(|it| it.is_ident_deep(name))
            .unwrap_or(false)
    }

    /// Check if member in the expression has the given name
    pub fn has_member_name(&self, name: &str) -> bool {
        self.member()
            .ok()
            .and_then(|it| {
                it.as_js_name()
                    .and_then(|it| it.value_token().ok())
                    .map(|it| it.text() == name)
            })
            .unwrap_or(false)
    }
}

impl JsComputedMemberExpression {
    /// Check if the object in the expression has the given name, optionally with parentheses
    pub fn has_object_name(&self, name: impl FnOnce(JsReferenceIdentifier) -> bool) -> bool {
        self.object()
            .map(|it| it.is_ident_deep(name))
            .unwrap_or(false)
    }

    /// Check if member in the expression is given string as string literal or non-interpolated template.
    pub fn has_member_name(&self, name: &str) -> bool {
        self.member()
            .ok()
            .and_then(|it| {
                it.as_string_or_no_substitution_template()
                    .map(|it| it == name)
            })
            .unwrap_or(false)
    }
}

impl JsAnyExpression {
    /// Return the expression is a string of given value if the given expression is
    /// 1. A string literal
    /// 2. A template literal with no substitutions
    pub fn is_string_or_no_substitution_template(&self, text: &str) -> bool {
        self.with_string_or_no_substitution_template(|it| it == text)
            .unwrap_or(false)
    }

    /// Return the string value if the given expression is
    /// 1. A string literal
    /// 2. A template literal with no substitutions
    pub fn as_string_or_no_substitution_template(&self) -> Option<String> {
        self.with_string_or_no_substitution_template(|it| it.to_string())
    }

    /// Call the given closure if the given expression is
    /// 1. A string literal
    /// 2. A template literal with no substitutions
    fn with_string_or_no_substitution_template<R>(&self, f: impl FnOnce(&str) -> R) -> Option<R> {
        match self {
            Self::JsTemplate(t) => {
                if t.tag().is_some() {
                    return None;
                }

                let mut elements = t.elements().into_iter();
                match (elements.next(), elements.next()) {
                    (Some(JsAnyTemplateElement::JsTemplateChunkElement(chunk)), None) => chunk
                        .template_chunk_token()
                        .ok()
                        .map(|it| f(it.text_trimmed())),
                    _ => None,
                }
            }
            Self::JsAnyLiteralExpression(JsAnyLiteralExpression::JsStringLiteralExpression(s)) => {
                s.inner_string_text().ok().map(|it| f(&it))
            }
            _ => None,
        }
    }
}

fn is_optional_chain(start: JsAnyExpression) -> bool {
    let mut current = Some(start);

    while let Some(node) = current {
        current = match node {
            JsAnyExpression::JsParenthesizedExpression(parenthesized) => {
                parenthesized.expression().ok()
            }

            JsAnyExpression::JsCallExpression(call) => {
                if call.is_optional() {
                    return true;
                }
                call.callee().ok()
            }

            JsAnyExpression::JsStaticMemberExpression(member) => {
                if member.is_optional() {
                    return true;
                }
                member.object().ok()
            }

            JsAnyExpression::JsComputedMemberExpression(member) => {
                if member.is_optional() {
                    return true;
                }
                member.object().ok()
            }
            _ => return false,
        }
    }

    false
}
