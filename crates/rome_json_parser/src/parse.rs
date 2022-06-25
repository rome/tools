use rome_json_syntax::{JsonSyntaxKind, T};
use rome_parse::ParseDiagnostic;

use crate::{
    event::Event,
    parse_error::{expected_any, expected_node},
    parser::CompletedMarker,
    token_source::Trivia,
    Parser,
};
pub fn parse_common(text: &str, file_id: usize) -> (Vec<Event>, Vec<ParseDiagnostic>, Vec<Trivia>) {
    let mut parser = Parser::new(text, file_id);
    parse_root(&mut parser);
    let (events, trivia, errors) = parser.finish();

    (events, errors, trivia)
}

fn parse_root(p: &mut Parser) -> CompletedMarker {
    let marker = p.start();
    parse_value(p);
    match p.cur() {
        JsonSyntaxKind::EOF => marker.complete(p, JsonSyntaxKind::JSON_ROOT),
        _ => {
            p.error(expected_node("EOF", p.cur_range()));
            while !p.at(JsonSyntaxKind::EOF) {
                p.bump_any();
            }
            marker.complete(p, JsonSyntaxKind::JSON_UNKNOWN)
        }
    }
}

fn parse_value(p: &mut Parser) -> CompletedMarker {
    let marker = p.start();
    match p.cur() {
        JsonSyntaxKind::EOF => {
            expected_any(
                &["{", "[", "number", "string", "null", "true", "false"],
                p.cur_range(),
            );
        }
        JsonSyntaxKind::COLON => {
            // TODO: Recover
            expected_any(
                &["{", "[", "number", "string", "null", "true", "false"],
                p.cur_range(),
            );
        }
        JsonSyntaxKind::COMMA => {
            // TODO: Recover
            expected_any(
                &["{", "[", "number", "string", "null", "true", "false"],
                p.cur_range(),
            );
        }
        JsonSyntaxKind::L_CURLY => {
            parse_object(p);
        }
        JsonSyntaxKind::R_CURLY => {
            // TODO: Recover
            expected_any(
                &["{", "[", "number", "string", "null", "true", "false"],
                p.cur_range(),
            );
        }
        JsonSyntaxKind::L_BRACK => {
            parse_array(p);
        }
        JsonSyntaxKind::R_BRACK => {
            // TODO: Recover
            expected_any(
                &["{", "[", "number", "string", "null", "true", "false"],
                p.cur_range(),
            );
        }
        JsonSyntaxKind::NULL_KW => {
            parse_null(p);
        }
        JsonSyntaxKind::TRUE_KW | JsonSyntaxKind::FALSE_KW => {
            parse_boolean(p);
        }
        JsonSyntaxKind::JSON_STRING_LITERAL => {
            parse_string(p);
        }
        JsonSyntaxKind::JSON_NUMBER_LITERAL => {
            parse_number(p);
        }
        JsonSyntaxKind::ERROR_TOKEN => {
            // TODO: Recover
            expected_any(
                &["{", "[", "number", "string", "null", "true", "false"],
                p.cur_range(),
            );
        }
        _ => unreachable!(),
    }
    marker.complete(p, JsonSyntaxKind::JSON_VALUE)
}

fn parse_number(p: &mut Parser) -> CompletedMarker {
    let marker = p.start();
    p.bump(JsonSyntaxKind::JSON_NUMBER_LITERAL);
    marker.complete(p, JsonSyntaxKind::JSON_NUMBER)
}

fn parse_string(p: &mut Parser) -> CompletedMarker {
    let marker = p.start();
    p.bump(JsonSyntaxKind::JSON_STRING_LITERAL);
    marker.complete(p, JsonSyntaxKind::JSON_STRING)
}

fn parse_boolean(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(T![true]) || p.at(T![false]));
    let marker = p.start();
    p.bump_any();
    marker.complete(p, JsonSyntaxKind::JSON_BOOLEAN)
}

fn parse_null(p: &mut Parser) -> CompletedMarker {
    let marker = p.start();
    p.bump(T![null]);
    marker.complete(p, JsonSyntaxKind::JSON_NULL)
}

fn parse_object(p: &mut Parser) -> CompletedMarker {
    todo!()
}

fn parse_array(p: &mut Parser) -> CompletedMarker {
    todo!()
}
