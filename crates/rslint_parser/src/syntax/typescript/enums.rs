use crate::parser::RecoveryResult;
use crate::syntax::binding::parse_binding;
use crate::syntax::class::parse_initializer_clause;
use crate::syntax::expr::ExpressionContext;

use super::ts_parse_error::expected_ts_enum_member;
use crate::syntax::stmt::STMT_RECOVERY_SET;
use crate::{JsSyntaxKind::*, *};

fn parse_literal_as_ts_enum_member(p: &mut Parser) -> ParsedSyntax {
    let m = p.start();
    match p.cur() {
        JS_STRING_LITERAL | T![ident] => {
            p.bump_any();
        }
        t if t.is_keyword() => {
            p.bump_remap(T![ident]);
        }
        JS_NUMBER_LITERAL => {
            let err = p
                .err_builder("An enum member cannot have a numeric name")
                .primary(p.cur_tok().range(), "");
            p.error(err);
            p.bump_any()
        }
        _ => {
            m.abandon(p);
            return Absent;
        }
    }
    Present(m.complete(p, JS_LITERAL_MEMBER_NAME))
}

/// An individual enum member
fn parse_ts_enum_member(p: &mut Parser) -> ParsedSyntax {
    let member = p.start();

    let name = match p.cur() {
        T!['['] => syntax::object::parse_computed_member_name(p),
        T![#] => {
            let err = p
                .err_builder("An `enum` member cannot be private")
                .primary(p.cur_tok().range(), "");
            p.error(err);
            syntax::class::parse_private_class_member_name(p).map(|mut x| {
                x.change_to_unknown(p);
                x
            })
        }
        _ => parse_literal_as_ts_enum_member(p),
    };

    if name.is_absent() {
        member.abandon(p);
        return Absent;
    }

    let _ = parse_initializer_clause(p, ExpressionContext::default());

    Present(member.complete(p, TS_ENUM_MEMBER))
}
struct TsEnumMembersList;

impl ParseSeparatedList for TsEnumMembersList {
    fn parse_element(&mut self, p: &mut Parser) -> ParsedSyntax {
        parse_ts_enum_member(p)
    }

    fn is_at_list_end(&mut self, p: &mut Parser) -> bool {
        p.at(T!['}'])
    }

    fn recover(&mut self, p: &mut Parser, parsed_element: ParsedSyntax) -> RecoveryResult {
        parsed_element.or_recover(
            p,
            &ParseRecovery::new(
                JS_UNKNOWN_MEMBER,
                STMT_RECOVERY_SET.union(token_set![JsSyntaxKind::IDENT, T![,], T!['}']]),
            )
            .enable_recovery_on_line_break(),
            expected_ts_enum_member,
        )
    }

    fn list_kind() -> JsSyntaxKind {
        TS_ENUM_MEMBER_LIST
    }

    fn separating_element_kind(&mut self) -> JsSyntaxKind {
        T![,]
    }

    fn allow_trailing_separating_element(&self) -> bool {
        true
    }
}

#[inline(always)]
fn is_reserved_enum_name(name: &str) -> bool {
    super::is_reserved_type_name(name)
}

fn parse_ts_enum_id(p: &mut Parser, enum_token_range: Range<usize>) {
    match parse_binding(p) {
        Present(id) => {
            let text = p.span_text(id.range(p));
            if is_reserved_enum_name(text) {
                let err = p
                    .err_builder(&format!(
                        "`{}` cannot be used as a enum name because it is already reserved",
                        text
                    ))
                    .primary(id.range(p), "");

                p.error(err);
            }
        }
        // test_err ts enum_decl_no_id
        // enum {}
        // enum {A,B,C}
        Absent => {
            if p.nth_at(1, L_CURLY) {
                let range = p.cur_tok().range();

                let m = p.start();
                p.bump_remap(T![ident]);
                let _ = m.complete(p, JS_IDENTIFIER_BINDING);

                let err = p.err_builder("invalid `enum` name").primary(range, "");
                p.error(err);
            } else {
                let err = p
                    .err_builder("`enum` statements must have a name")
                    .primary(enum_token_range.start..p.cur_tok().start(), "");
                p.error(err);
            }
        }
    }
}

pub(crate) fn is_at_ts_enum_statement(p: &Parser, t: &JsSyntaxKind) -> bool {
    let is_ident1 = p.nth_at(1, JsSyntaxKind::IDENT);
    let is_l_curly1 = p.nth_at(1, JsSyntaxKind::L_CURLY);

    let is_ident2 = p.nth_at(2, JsSyntaxKind::IDENT);
    let is_l_curly2 = p.nth_at(2, JsSyntaxKind::L_CURLY);

    (*t == T![enum] && (is_ident1 || is_l_curly1))
        || (*t == T![const] && p.nth_at(1, T![enum]) && (is_ident2 || is_l_curly2))
}

// test ts typescript_enum
// enum A {}
// enum B { a, b, c }
// const enum C { A = 1, B = A * 2, ["A"] = 3, }
pub(crate) fn parse_ts_enum_statement(p: &mut Parser) -> ParsedSyntax {
    let m = p.start();

    p.eat(T![const]);

    let enum_token_range = p.cur_tok().range();
    if !p.expect(T![enum]) {
        m.abandon(p);
        return Absent;
    }

    parse_ts_enum_id(p, enum_token_range);

    p.expect(T!['{']);

    TsEnumMembersList.parse_list(p);

    p.expect(T!['}']);

    let mut res = m.complete(p, TS_ENUM_STATEMENT);
    res.err_if_not_ts(p, "enums can only be declared in TypeScript files");
    Present(res)
}
