pub mod jsx_parse_errors;

use rslint_lexer::{JsSyntaxKind, T};

use crate::{
    parser::RecoveryResult, Checkpoint, Marker, ParseNodeList, ParseRecovery, ParsedSyntax, Parser,
};

use self::jsx_parse_errors::jsx_expected_attribute;

use super::expr::ExpressionContext;

// Constraints function to be inside a checkpointed parser
// allowing them advancing and abandoning the parser.
struct CheckpointedParser<'a, 'b> {
    parser: &'a mut Parser<'b>,
    checkpoint: Checkpoint,
}

impl<'a, 'b> CheckpointedParser<'a, 'b> {
    pub fn new(p: &'a mut Parser<'b>) -> CheckpointedParser<'a, 'b> {
        let checkpoint = p.checkpoint();
        Self {
            parser: p,
            checkpoint,
        }
    }

    pub fn rewind(self) -> &'a mut Parser<'b> {
        self.parser.rewind(self.checkpoint);
        self.parser
    }
}

impl<'a, 'b> std::ops::Deref for CheckpointedParser<'a, 'b> {
    type Target = Parser<'b>;

    fn deref(&self) -> &Self::Target {
        self.parser
    }
}

impl<'a, 'b> std::ops::DerefMut for CheckpointedParser<'a, 'b> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.parser
    }
}

// It is impossible to lookahead and guarantee that we are at a JSX expression,
// so this function will checkpoint and rewind the parser on failures.
pub(super) fn maybe_parse_jsx_expression(p: &mut Parser) -> ParsedSyntax {
    if !p.at(T![<]) {
        return ParsedSyntax::Absent;
    }

    let mut p = CheckpointedParser::new(p);
    let syntax = parse_jsx_expression(&mut p);

    if syntax.is_absent() {
        p.rewind();
    }

    syntax
}

// test jsx jsx_element_on_return
// function f() {
//     return <div></div>
// }

// test jsx jsx_element_on_arrow_function
// const f = () => <div></div>;
// const f = () => (<div></div>);

// test jsx jsx_element_as_statements
// <div />
fn parse_jsx_expression(p: &mut CheckpointedParser<'_, '_>) -> ParsedSyntax {
    parse_jsx_element(p).map(|element| {
        let m = element.precede(p);
        m.complete(p, JsSyntaxKind::JSX_ELEMENT_EXPRESSION)
    })
}

// test jsx jsx_element_open_close
// function f() {
//     return <div></div>
// }

// test jsx jsx_element_self_close
// function f() {
//     return <div />
// }
fn parse_jsx_element(p: &mut Parser) -> ParsedSyntax {
    let m = p.start();
    match parse_jsx_element_head(p, m) {
        ParsedSyntax::Present(opening_marker)
            if opening_marker.kind() == JsSyntaxKind::JSX_OPENING_ELEMENT =>
        {
            let element_marker = opening_marker.precede(p);
            let closing_marker = parse_jsx_closing_element(p);
            if closing_marker.is_absent() {
                element_marker.abandon(p);
                ParsedSyntax::Absent
            } else {
                ParsedSyntax::Present(element_marker.complete(p, JsSyntaxKind::JSX_ELEMENT))
            }
        }
        ParsedSyntax::Present(self_closing_marker)
            if self_closing_marker.kind() == JsSyntaxKind::JSX_SELF_CLOSING_ELEMENT =>
        {
            ParsedSyntax::Present(self_closing_marker)
        }
        ParsedSyntax::Absent => ParsedSyntax::Absent,
        _ => unreachable!("Unexpected present node returned"),
    }
}

// <a ...> or <a ... />
// ^          ^
fn parse_jsx_element_head(p: &mut Parser, m: Marker) -> ParsedSyntax {
    if !p.eat(T![<]) {
        return ParsedSyntax::Absent;
    }

    let name = parse_jsx_any_element_name(p);
    if name.is_absent() {
        m.abandon(p);
        return ParsedSyntax::Absent;
    }

    JsxAttributeList.parse_list(p);

    let kind = if p.at(T![/]) && p.nth_at(1, T![>]) {
        p.bump_multiple(2, JsSyntaxKind::SLASH_R_ANGLE);
        JsSyntaxKind::JSX_SELF_CLOSING_ELEMENT
    } else if p.eat(T![>]) {
        JsSyntaxKind::JSX_OPENING_ELEMENT
    } else {
        m.abandon(p);
        return ParsedSyntax::Absent;
    };

    ParsedSyntax::Present(m.complete(p, kind))
}

// <a/>
// ^
fn parse_jsx_closing_element(p: &mut Parser) -> ParsedSyntax {
    if !p.at(T![<]) {
        return ParsedSyntax::Absent;
    }

    let m = p.start();

    if p.at(T![<]) && p.nth_at(1, T![/]) {
        p.bump_multiple(2, JsSyntaxKind::L_ANGLE_SLASH);
    } else {
        m.abandon(p);
        return ParsedSyntax::Absent;
    }

    let closing_name = parse_jsx_any_element_name(p);
    if closing_name.is_absent() {
        m.abandon(p);
        return ParsedSyntax::Absent;
    }

    if !p.eat(T![>]) {
        m.abandon(p);
        return ParsedSyntax::Absent;
    }

    ParsedSyntax::Present(m.complete(p, JsSyntaxKind::JSX_CLOSING_ELEMENT))
}

fn parse_jsx_any_element_name(p: &mut Parser) -> ParsedSyntax {
    if !p.eat(T![ident]) {
        return ParsedSyntax::Absent;
    }

    let m = p.start();
    p.bump(T![ident]);
    ParsedSyntax::Present(m.complete(p, JsSyntaxKind::JSX_REFERENCE_IDENTIFIER))
}

struct JsxAttributeList;

// test jsx jsx_element_attributes
// function f() {
//     return <div string_literal="a" expression={1} novalue el=<a/>></div>;
// }
impl ParseNodeList for JsxAttributeList {
    fn parse_element(&mut self, p: &mut Parser) -> ParsedSyntax {
        let m = p.start();

        let name = parse_jsx_attribute_name(p);
        if name.is_absent() {
            m.abandon(p);
            return ParsedSyntax::Absent;
        }

        let _ = parse_jsx_attribute_initializer_clause(p);

        ParsedSyntax::Present(m.complete(p, JsSyntaxKind::JSX_ATTRIBUTE))
    }

    fn is_at_list_end(&mut self, p: &mut Parser) -> bool {
        p.at(T![>]) || p.at(T![/])
    }

    fn recover(&mut self, p: &mut Parser, parsed_element: ParsedSyntax) -> RecoveryResult {
        parsed_element.or_recover(
            p,
            &ParseRecovery::new(
                JsSyntaxKind::JS_UNKNOWN_MEMBER,
                token_set![T![/], T![>], T![<], T!['{'], T!['}'], T![ident]],
            ),
            jsx_expected_attribute,
        )
    }

    fn list_kind() -> JsSyntaxKind {
        JsSyntaxKind::JSX_ATTRIBUTE_LIST
    }
}

fn parse_jsx_attribute_name(p: &mut Parser) -> ParsedSyntax {
    if !p.at(JsSyntaxKind::IDENT) {
        return ParsedSyntax::Absent;
    }

    let m = p.start();
    p.bump(JsSyntaxKind::IDENT);
    ParsedSyntax::Present(m.complete(p, JsSyntaxKind::JSX_NAME))
}

fn parse_jsx_attribute_initializer_clause(p: &mut Parser) -> ParsedSyntax {
    if p.at(T![=]) {
        ParsedSyntax::Absent
    }

    let m = p.start();

    p.bump(T![=]);
    let _ = parse_jsx_attribute_value(p);

    ParsedSyntax::Present(m.complete(p, JsSyntaxKind::JSX_ATTRIBUTE_INITIALIZER_CLAUSE))
}

fn parse_jsx_attribute_value(p: &mut Parser) -> ParsedSyntax {
    // Possible atribute values:
    // String Literal for constant values
    if p.at(JsSyntaxKind::JS_STRING_LITERAL) {
        let m = p.start();
        p.bump(JsSyntaxKind::JS_STRING_LITERAL);
        ParsedSyntax::Present(m.complete(p, JsSyntaxKind::JSX_STRING_LITERAL))
    }
    // expression values
    else if p.at(T!['{']) {
        let m = p.start();
        p.bump(T!['{']);
        let _ = super::expr::parse_expression(p, ExpressionContext::default());
        p.bump(T!['}']);
        ParsedSyntax::Present(m.complete(p, JsSyntaxKind::JSX_EXPRESSION_ATTRIBUTE_VALUE))
    }
    // JSX elements
    else if p.at(T![<]) {
        parse_jsx_element(p)
    } else {
        ParsedSyntax::Absent
    }
}
