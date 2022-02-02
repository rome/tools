use crate::parser::{RecoveryResult, ToDiagnostic};
use crate::syntax::binding::parse_binding;
use crate::syntax::class::parse_initializer_clause;
use crate::syntax::expr::ExpressionContext;
use crate::syntax::js_parse_error;

use crate::{JsSyntaxKind::*, *};

use super::is_reserved_type_name;

pub(super) fn parse_literal_member_name(p: &mut Parser) -> ParsedSyntax {
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
fn parse_enum_member(p: &mut Parser) -> ParsedSyntax {
    let member = p.start();
    
    let _ = match p.cur() {
        T!['['] => syntax::object::parse_computed_member_name(p),
        _ => parse_literal_member_name(p),
    };

    let _ = parse_initializer_clause(p, ExpressionContext::default());
    Present(member.complete(p, TS_ENUM_MEMBER))
}

fn expected_enum_member(p: &Parser, range: Range<usize>) -> Diagnostic {
    parser::expected_any(&["identifier", "string literal", "computed name"], range).to_diagnostic(p)
}

struct EnumMembersList;

impl ParseSeparatedList for EnumMembersList {
    fn parse_element(&mut self, p: &mut Parser) -> ParsedSyntax {
        parse_enum_member(p)
    }

    fn is_at_list_end(&mut self, p: &mut Parser) -> bool {
        p.at(T!['}'])
    }

    fn recover(&mut self, p: &mut Parser, parsed_element: ParsedSyntax) -> RecoveryResult {
        parsed_element.or_recover(
            p,
            &ParseRecovery::new(JS_UNKNOWN_MEMBER, token_set![T![,], T!['}'], T![;], T![:]])
                .enable_recovery_on_line_break(),
            expected_enum_member,
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


pub(crate) fn is_reserved_enum_name(name: &str) -> bool {
    matches!(
        name,
        "string"
            | "null"
            | "number"
            | "object"
            | "any"
            | "unknown"
            | "boolean"
            | "bigint"
            | "symbol"
            | "void"
            | "never"
    )
}

fn parse_name(p: &mut Parser, enum_token_range: Range<usize>) {
    let id = if p.cur_src() == "{" {
        Absent
    } else {
        parse_binding(p)
    };

    match id {
        Present(id) => {
            let text = p.span_text(id.range(p));
            if is_reserved_enum_name(text) {
                let err = p
                    .err_builder(&format!(
                            "`{}` cannot be used as a enum name because it is already reserved as a enum",
                            text
                        ))
                    .primary(id.range(p), "");

                p.error(err);
            }
        }
        Absent => {
            let err = p
                .err_builder("enum declarations must have a name")
                .primary(enum_token_range.start..p.cur_tok().start(), "");
            p.error(err);
        }
    }
}

// test ts typescript_enum
// enum A {}
// enum B { a, b, c }
// const enum C { A = 1, B = A * 2, ["A"] = 3, }
pub fn ts_enum(p: &mut Parser) -> CompletedMarker {
    let m = p.start();

    p.eat(T![const]);

    let enum_token_range = p.cur_tok().range();
    p.expect(T![enum]);
    parse_name(p, enum_token_range);

    p.expect(T!['{']);

    EnumMembersList.parse_list(p);

    p.expect(T!['}']);
    m.complete(p, TS_ENUM)
}
