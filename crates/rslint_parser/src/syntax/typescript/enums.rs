use crate::parser::{RecoveryResult, ToDiagnostic};
use crate::syntax::class::parse_initializer_clause;
use crate::syntax::expr::parse_name;
use crate::syntax::expr::ExpressionContext;
use crate::syntax::js_parse_error;

use crate::syntax::object::parse_object_member_name;
use crate::{JsSyntaxKind::*, *};

/// An individual enum member
fn parse_enum_member(p: &mut Parser) -> ParsedSyntax {
    let member = p.start();
    let _ = parse_object_member_name(p);
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

// test ts typescript_enum
// enum A {}
// enum B { a, b, c }
// const enum C { A = 1, B = A * 2, ["A"] = 3, }
pub fn ts_enum(p: &mut Parser) -> CompletedMarker {
    let m = p.start();

    p.eat(T![const]);
    p.expect(T![enum]);
    parse_name(p).or_add_diagnostic(p, js_parse_error::expected_identifier);
    p.expect(T!['{']);

    EnumMembersList.parse_list(p);

    p.expect(T!['}']);
    m.complete(p, TS_ENUM)
}
