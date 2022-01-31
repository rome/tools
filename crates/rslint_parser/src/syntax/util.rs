//! General utility functions for parsing and error checking.

use crate::Parser;
use rslint_syntax::{JsSyntaxKind, T};

/// See: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Operator_Precedence#table
#[allow(dead_code)]
#[derive(Debug, Eq, Ord, PartialOrd, PartialEq, Copy, Clone)]
pub(crate) enum OperatorPrecedence {
    Comma = 0,
    Yield = 1,
    Assignment = 2,
    Conditional = 3,
    Coalesce = 4,
    LogicalOr = 5,
    LogicalAnd = 6,
    BitwiseOr = 7,
    BitwiseXor = 8,
    BitwiseAnd = 9,
    Equality = 10,
    Relational = 11,
    Shift = 12,
    Additive = 13,
    Multiplicative = 14,
    Exponential = 15,
    Unary = 16,
    Update = 17,
    LeftHandSide = 18,
    Member = 19,
    Primary = 20,
}

impl OperatorPrecedence {
    /// Returns the operator with the lowest precedence
    pub fn lowest() -> Self {
        OperatorPrecedence::Comma
    }

    /// Returns the operator with the highest precedence
    #[allow(dead_code)]
    pub fn highest() -> Self {
        OperatorPrecedence::Primary
    }

    /// Returns `true` if this operator has right to left associativity
    pub fn is_right_to_left(&self) -> bool {
        matches!(
            self,
            OperatorPrecedence::Yield
                | OperatorPrecedence::Assignment
                | OperatorPrecedence::Conditional
                | OperatorPrecedence::Exponential
                | OperatorPrecedence::Update
        )
    }

    /// Returns the precedence for a binary operator token or `Err` if the token isn't a binary operator
    pub fn try_from_binary_operator(kind: JsSyntaxKind) -> Result<OperatorPrecedence, ()> {
        Ok(match kind {
            T![??] => OperatorPrecedence::Coalesce,
            T![||] => OperatorPrecedence::LogicalOr,
            T![&&] => OperatorPrecedence::LogicalAnd,
            T![|] => OperatorPrecedence::BitwiseOr,
            T![^] => OperatorPrecedence::BitwiseXor,
            T![&] => OperatorPrecedence::BitwiseAnd,
            T![==] | T![!=] | T![===] | T![!==] => OperatorPrecedence::Equality,
            T![<] | T![>] | T![<=] | T![>=] | T![instanceof] | T![in] => {
                OperatorPrecedence::Relational
            }
            T![<<] | T![>>] | T![>>>] => OperatorPrecedence::Shift,
            T![+] | T![-] => OperatorPrecedence::Additive,
            T![*] | T![/] | T![%] => OperatorPrecedence::Multiplicative,
            T![**] => OperatorPrecedence::Exponential,
            _ => return Err(()),
        })
    }
}

pub(crate) fn expect_keyword(p: &mut Parser, keyword_name: &str, kind: JsSyntaxKind) {
    if p.at(T![ident]) && p.cur_src() == keyword_name {
        p.bump_remap(kind);
    } else {
        let err = if p.cur() == JsSyntaxKind::EOF {
            p.err_builder(&format!(
                "expected `{}` but instead the file ends",
                keyword_name
            ))
            .primary(p.cur_tok().range(), "the file ends here")
        } else {
            p.err_builder(&format!(
                "expected `{}` but instead found `{}`",
                keyword_name,
                p.cur_src()
            ))
            .primary(p.cur_tok().range(), "unexpected")
        };

        p.error(err);
    }
}
