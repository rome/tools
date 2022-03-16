//! Extensions for things which are not easily generated in ast expr nodes
use crate::numbers::{parse_js_big_int, parse_js_number};
use crate::{
    AstNode, AstSeparatedList, JsArrayExpression, JsArrayHole, JsAssignmentExpression,
    JsBigIntLiteralExpression, JsBinaryExpression, JsLiteralMemberName, JsLogicalExpression,
    JsNumberLiteralExpression, JsObjectExpression, JsStringLiteralExpression, JsTemplate,
    JsUnaryExpression, SyntaxResult, SyntaxToken,
};
use crate::{JsPreUpdateExpression, JsSyntaxKind::*};
use num_bigint::BigInt;
use rome_rowan::{NodeOrToken, SyntaxText, TextRange, TextSize};
use std::cmp::Ordering;

impl JsLiteralMemberName {
    /// Returns the name of the member as a syntax text
    ///
    /// ## Examples
    ///
    /// Getting the name of a static member containing a string literal
    ///
    /// ```
    /// use rome_js_syntax::{JsSyntaxKind, JsLanguage, SyntaxNode, SyntaxNodeExt, SyntaxTreeBuilder, JsLiteralMemberName};
    ///
    /// let node: SyntaxNode = SyntaxTreeBuilder::wrap_with_node(JsSyntaxKind::JS_LITERAL_MEMBER_NAME, |builder| {
    ///   builder.token(JsSyntaxKind::JS_STRING_LITERAL, "\"abcd\"");
    /// });
    ///
    /// let static_member_name = node.to::<JsLiteralMemberName>();
    ///
    /// assert_eq!("abcd", static_member_name.name().unwrap());
    /// ```
    ///
    /// Getting the name of a static member containing a number literal
    ///
    /// ```
    /// use rome_js_syntax::{JsSyntaxKind, JsLanguage, SyntaxNode, SyntaxNodeExt, SyntaxTreeBuilder, JsLiteralMemberName};
    ///
    /// let node: SyntaxNode = SyntaxTreeBuilder::wrap_with_node(JsSyntaxKind::JS_LITERAL_MEMBER_NAME, |builder| {
    ///   builder.token(JsSyntaxKind::JS_NUMBER_LITERAL, "5");
    /// });
    ///
    /// let static_member_name = node.to::<JsLiteralMemberName>();
    ///
    /// assert_eq!("5", static_member_name.name().unwrap());
    /// ```
    ///
    /// Getting the name of a static member containing an identifier
    ///
    /// ```
    /// use rome_js_syntax::{JsSyntaxKind, JsLanguage, SyntaxNode, SyntaxNodeExt, SyntaxTreeBuilder, JsLiteralMemberName};
    ///
    /// let node: SyntaxNode = SyntaxTreeBuilder::wrap_with_node(JsSyntaxKind::JS_LITERAL_MEMBER_NAME, |builder| {
    ///   builder.token(JsSyntaxKind::IDENT, "abcd");
    /// });
    ///
    /// let static_member_name = node.to::<JsLiteralMemberName>();
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
pub enum JsBinaryOperation {
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

impl JsBinaryOperation {
    pub fn is_bit_wise_operator(&self) -> bool {
        matches!(
            self,
            JsBinaryOperation::LeftShift
                | JsBinaryOperation::RightShift
                | JsBinaryOperation::UnsignedRightShift
                | JsBinaryOperation::BitwiseAnd
                | JsBinaryOperation::BitwiseOr
                | JsBinaryOperation::BitwiseXor
        )
    }

    pub fn is_plus_or_minus_operator(&self) -> bool {
        matches!(self, JsBinaryOperation::Plus | JsBinaryOperation::Minus)
    }

    pub fn is_times_or_div_operator(&self) -> bool {
        matches!(
            self,
            JsBinaryOperation::Divide | JsBinaryOperation::Times | JsBinaryOperation::Remainder
        )
    }

    pub fn is_exponent_operator(&self) -> bool {
        matches!(self, JsBinaryOperation::Exponent)
    }

    pub fn is_comparison_operator(&self) -> bool {
        matches!(
            self,
            JsBinaryOperation::LessThan
                | JsBinaryOperation::GreaterThan
                | JsBinaryOperation::LessThanOrEqual
                | JsBinaryOperation::GreaterThanOrEqual
                | JsBinaryOperation::Equality
                | JsBinaryOperation::StrictEquality
                | JsBinaryOperation::Inequality
                | JsBinaryOperation::StrictInequality
        )
    }

    // The numbers returned by this function are arbitrary, the most important thing
    // is that, given the current implementation, they should be ordered from bigger (top) to smaller (bottom)
    pub fn get_precedence(&self) -> u8 {
        if self.is_bit_wise_operator() {
            5
        } else if self.is_times_or_div_operator() {
            4
        } else if self.is_plus_or_minus_operator() {
            3
        } else if self.is_comparison_operator() {
            2
        } else {
            1
        }
    }

    pub fn compare_precedence(&self, other: &Self) -> Ordering {
        let self_precedence = self.get_precedence();
        let other_precedence = other.get_precedence();

        self_precedence.cmp(&other_precedence)
    }
}

impl JsBinaryExpression {
    pub fn operator_kind(&self) -> SyntaxResult<JsBinaryOperation> {
        let kind = match self.operator()?.kind() {
            T![<] => JsBinaryOperation::LessThan,
            T![>] => JsBinaryOperation::GreaterThan,
            T![<=] => JsBinaryOperation::LessThanOrEqual,
            T![>=] => JsBinaryOperation::GreaterThanOrEqual,
            T![==] => JsBinaryOperation::Equality,
            T![===] => JsBinaryOperation::StrictEquality,
            T![!=] => JsBinaryOperation::Inequality,
            T![!==] => JsBinaryOperation::StrictInequality,
            T![+] => JsBinaryOperation::Plus,
            T![-] => JsBinaryOperation::Minus,
            T![*] => JsBinaryOperation::Times,
            T![/] => JsBinaryOperation::Divide,
            T![%] => JsBinaryOperation::Remainder,
            T![**] => JsBinaryOperation::Exponent,
            T![<<] => JsBinaryOperation::LeftShift,
            T![>>] => JsBinaryOperation::RightShift,
            T![>>>] => JsBinaryOperation::UnsignedRightShift,
            T![&] => JsBinaryOperation::BitwiseAnd,
            T![|] => JsBinaryOperation::BitwiseOr,
            T![^] => JsBinaryOperation::BitwiseXor,
            _ => unreachable!(),
        };

        Ok(kind)
    }
    /// Whether this is a comparison operation, such as `>`, `<`, `==`, `!=`, `===`, etc.
    pub fn is_comparison_operator(&self) -> bool {
        matches!(
            self.operator().map(|t| t.kind()),
            Ok(T![>] | T![<] | T![>=] | T![<=] | T![==] | T![===] | T![!=] | T![!==])
        )
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum JsLogicalOperation {
    /// `??`
    NullishCoalescing,
    /// `||`
    LogicalOr,
    /// `&&`
    LogicalAnd,
}

impl JsLogicalExpression {
    pub fn operator_kind(&self) -> SyntaxResult<JsLogicalOperation> {
        let kind = match self.operator()?.kind() {
            T![&&] => JsLogicalOperation::LogicalAnd,
            T![||] => JsLogicalOperation::LogicalOr,
            T![??] => JsLogicalOperation::NullishCoalescing,
            _ => unreachable!(),
        };

        Ok(kind)
    }
}

impl JsArrayHole {
    pub fn hole_token(&self) -> Option<SyntaxToken> {
        None
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum JsUnaryOperation {
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

impl JsUnaryExpression {
    pub fn operation(&self) -> SyntaxResult<JsUnaryOperation> {
        let operator = self.operator()?;

        Ok(match operator.kind() {
            T![+] => JsUnaryOperation::Plus,
            T![-] => JsUnaryOperation::Minus,
            T![~] => JsUnaryOperation::BitwiseNot,
            T![!] => JsUnaryOperation::LogicalNot,
            T![typeof] => JsUnaryOperation::Typeof,
            T![void] => JsUnaryOperation::Void,
            T![delete] => JsUnaryOperation::Delete,
            _ => unreachable!(),
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum JsPreUpdateOperation {
    /// `++`
    Increment,
    /// `--`
    Decrement,
}

impl JsPreUpdateExpression {
    pub fn operation(&self) -> SyntaxResult<JsPreUpdateOperation> {
        let operator = self.operator()?;

        Ok(match operator.kind() {
            T![++] => JsPreUpdateOperation::Increment,
            T![--] => JsPreUpdateOperation::Decrement,
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
}

impl JsNumberLiteralExpression {
    pub fn as_number(&self) -> Option<f64> {
        parse_js_number(self.value_token().unwrap().text())
    }
}

impl JsBigIntLiteralExpression {
    pub fn as_number(&self) -> Option<BigInt> {
        parse_js_big_int(self.value_token().ok()?.text())
    }
}

impl JsStringLiteralExpression {
    /// Get the inner text of a string not including the quotes
    pub fn inner_string_text(&self) -> SyntaxText {
        let start = self.syntax().text_range().start() + TextSize::from(1);
        let end_char = self
            .syntax()
            .text()
            .char_at(self.syntax().text().len() - TextSize::from(1))
            .unwrap();
        let end = if end_char == '"' || end_char == '\'' {
            self.syntax().text_range().end() - TextSize::from(1)
        } else {
            self.syntax().text_range().end()
        };

        let offset = self.syntax().text_range().start();

        self.syntax()
            .text()
            .slice(TextRange::new(start - offset, end - offset))
    }
}

impl JsTemplate {
    /// The string chunks of the template. aka:
    /// `foo ${bar} foo` breaks down into:
    /// `QUASIS ELEMENT{EXPR} QUASIS`
    pub fn quasis(&self) -> impl Iterator<Item = SyntaxToken> {
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
