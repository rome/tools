use crate::parser::{expected_any, expected_node, ParserProgress, RecoveryResult, ToDiagnostic};
use crate::state::{EnterAmbientContext, ExportDefaultItem, ExportDefaultItemKind};
use crate::syntax::binding::{
    is_at_identifier_binding, is_nth_at_identifier_binding, parse_binding, parse_identifier_binding,
};
use crate::syntax::class::parse_class_export_default_declaration;
use crate::syntax::expr::{
    is_nth_at_expression, is_nth_at_reference_identifier, parse_assignment_expression_or_higher,
    parse_name, parse_reference_identifier, ExpressionContext,
};
use crate::syntax::function::parse_function_export_default_declaration;
use crate::syntax::js_parse_error::{
    duplicate_assertion_keys_error, expected_binding, expected_declaration, expected_export_clause,
    expected_export_name_specifier, expected_expression, expected_identifier,
    expected_literal_export_name, expected_module_source, expected_named_import,
    expected_named_import_specifier, expected_statement,
};
use crate::syntax::stmt::{parse_statement, semi, StatementContext, STMT_RECOVERY_SET};
use crate::syntax::typescript::ts_parse_error::ts_only_syntax_error;
use crate::syntax::typescript::{
    parse_ts_enum_declaration, parse_ts_import_equals_declaration_rest,
    parse_ts_interface_declaration,
};
use crate::syntax::util::{
    eat_contextual_keyword, expect_contextual_keyword, is_at_contextual_keyword,
    is_nth_at_contextual_keyword,
};
use crate::JsSyntaxFeature::TypeScript;
use crate::{
    Absent, CompletedMarker, Marker, ParseRecovery, ParseSeparatedList, ParsedSyntax, Parser,
    Present, SyntaxFeature,
};
use rome_js_syntax::JsSyntaxKind::*;
use rome_js_syntax::{JsSyntaxKind, T};
use rslint_errors::Span;
use std::collections::HashMap;
use std::ops::Range;

use super::auxiliary::{is_nth_at_declaration_clause, parse_declaration_clause};

///! Implements the parsing logic for ES Module syntax

// test module
// import a from "b";
// export { a };
// c();
// import { c } from "c";
pub(crate) fn parse_module_body(p: &mut Parser, m: Marker) -> CompletedMarker {
    parse_module_item_list(p, ModuleItemListParent::Module);

    m.complete(p, JS_MODULE)
}

pub(crate) enum ModuleItemListParent {
    Module,
    Block,
}

impl ModuleItemListParent {
    fn is_module(&self) -> bool {
        matches!(self, ModuleItemListParent::Module)
    }

    #[inline]
    fn is_at_list_end(&self, p: &Parser) -> bool {
        if p.at(EOF) {
            return true;
        }

        match self {
            ModuleItemListParent::Block => p.at(T!['}']),
            _ => false,
        }
    }
}

pub(crate) fn parse_module_item_list(p: &mut Parser, parent: ModuleItemListParent) {
    let list_marker = p.start();
    let mut progress = ParserProgress::default();

    let recovery_set = if parent.is_module() {
        STMT_RECOVERY_SET
    } else {
        // test_err ts module_closing_curly
        // declare module A {
        //  "name": "troublesome-lib",
        //  "typings": "lib/index.d.ts",
        //  "version": "0.0.1"
        // }

        // don't eat the closing `}` if inside a block
        STMT_RECOVERY_SET.union(token_set!(T!['}']))
    };

    while !parent.is_at_list_end(p) {
        progress.assert_progressing(p);

        let module_item = parse_module_item(p);

        let recovered = module_item.or_recover(
            p,
            &ParseRecovery::new(JS_UNKNOWN_STATEMENT, recovery_set),
            expected_statement,
        );

        if recovered.is_err() {
            break;
        }
    }

    list_marker.complete(p, JS_MODULE_ITEM_LIST);
}

fn parse_module_item(p: &mut Parser) -> ParsedSyntax {
    match p.cur() {
        T![import] if !token_set![T![.], T!['(']].contains(p.nth(1)) => {
            parse_import_or_import_equals_declaration(p)
        }
        T![export] => parse_export(p),
        _ => parse_statement(p, StatementContext::StatementList),
    }
}

pub(crate) fn parse_import_or_import_equals_declaration(p: &mut Parser) -> ParsedSyntax {
    if !p.at(T![import]) {
        return Absent;
    }

    let start = p.cur_tok().start();
    let import = p.start();
    p.bump(T![import]);

    debug_assert!(p.state.name_map.is_empty());
    p.state.duplicate_binding_parent = Some("import");

    let statement = if is_at_identifier_binding(p) && (p.nth_at(1, T![=]) || p.nth_at(2, T![=])) {
        let import_equals = parse_ts_import_equals_declaration_rest(p, import, start);
        TypeScript.exclusive_syntax(p, import_equals, |p, decl| {
            ts_only_syntax_error(p, "'import =' declarations", decl.range(p))
        })
    } else {
        // test_err import_err
        // import;
        // import *;
        // import * as c, { a, b } from "c";
        // import { aa + bb, dd } from "c";
        // import { ab, ac } from "c";
        // import { default } from "c";
        // import { "a" } from "c";
        // import { as x } from "c";
        // import 4 from "c";
        // import y from 4;
        parse_import_clause(p).or_add_diagnostic(p, |p, range| {
            expected_any(
                &["default import", "namespace import", "named import"],
                range,
            )
            .to_diagnostic(p)
        });

        let end = p.cur_tok().start();

        semi(p, start..end);
        Present(import.complete(p, JS_IMPORT))
    };

    p.state.duplicate_binding_parent = None;
    p.state.name_map.clear();

    statement
}

// test import_default_clause
// import foo from "test";
fn parse_import_clause(p: &mut Parser) -> ParsedSyntax {
    if p.at(JS_STRING_LITERAL) {
        return parse_import_bare_clause(p);
    }

    let pos = p.token_pos();
    let m = p.start();

    // test ts ts_import_clause_types
    // import type from "./mod"; // not a type
    // import type foo from "./mod";
    // import type * as foo2 from "./mod";
    // import type { foo3 } from "mod";
    let is_typed = is_at_contextual_keyword(p, "type")
        && (matches!(p.nth(1), T![*] | T!['{'])
            || (is_nth_at_identifier_binding(p, 1) && !is_nth_at_contextual_keyword(p, 1, "from")));

    if is_typed {
        expect_contextual_keyword(p, "type", T![type]);
    }

    let clause = match p.cur() {
        T![*] => parse_import_namespace_clause_rest(p, m),
        T!['{'] => parse_import_named_clause_rest(p, m),
        _ if is_at_identifier_binding(p) => {
            parse_identifier_binding(p).unwrap();
            parse_import_default_or_named_clause_rest(p, m, is_typed)
        }
        _ => {
            // SAFETY: Safe because the parser only eats the "type" keyword if it's followed by
            // either a *, {, or binding
            debug_assert_eq!(pos, p.token_pos());
            m.abandon(p);
            return Absent;
        }
    };

    if is_typed {
        TypeScript.exclusive_syntax(p, clause, |p, clause| {
            ts_only_syntax_error(p, "'import type'", clause.range(p))
        })
    } else {
        Present(clause)
    }
}

/// Parses the rest of an import named or default clause.
/// Rest meaning, everything after `type binding`
fn parse_import_default_or_named_clause_rest(
    p: &mut Parser,
    m: Marker,
    is_typed: bool,
) -> CompletedMarker {
    match p.cur() {
        T![,] | T!['{'] => {
            p.expect(T![,]);

            let default_specifier = m.complete(p, JS_DEFAULT_IMPORT_SPECIFIER);
            let default_start = default_specifier.range(p).start();

            let named_clause = default_specifier.precede(p);

            parse_named_import(p).or_add_diagnostic(p, expected_named_import);

            if is_typed {
                let end = p
                    .tokens
                    .last_tok()
                    .map(|t| t.end())
                    .unwrap_or_else(|| p.cur_tok().start());

                // test_err ts ts_typed_default_import_with_named
                // import type A, { B, C } from './a';
                p.error(p.err_builder("A type-only import can specify a default import or named bindings, but not both.")
                    .primary(default_start.into()..end, ""))
            }

            expect_contextual_keyword(p, "from", T![from]);
            parse_module_source(p).or_add_diagnostic(p, expected_module_source);
            parse_import_assertion(p).ok();

            named_clause.complete(p, JS_IMPORT_NAMED_CLAUSE)
        }
        _ => {
            expect_contextual_keyword(p, "from", T![from]);
            parse_module_source(p).or_add_diagnostic(p, expected_module_source);
            parse_import_assertion(p).ok();

            m.complete(p, JS_IMPORT_DEFAULT_CLAUSE)
        }
    }
}

// test import_bare_clause
// import "test";
// import "no_semicolon"
fn parse_import_bare_clause(p: &mut Parser) -> ParsedSyntax {
    parse_module_source(p).map(|module_source| {
        let m = module_source.precede(p);
        parse_import_assertion(p).ok();
        m.complete(p, JS_IMPORT_BARE_CLAUSE)
    })
}

// test import_decl
// import * as foo from "bla";
fn parse_import_namespace_clause_rest(p: &mut Parser, m: Marker) -> CompletedMarker {
    p.expect(T![*]);

    expect_contextual_keyword(p, "as", T![as]);
    parse_binding(p).or_add_diagnostic(p, expected_binding);
    expect_contextual_keyword(p, "from", T![from]);
    parse_module_source(p).or_add_diagnostic(p, expected_module_source);
    parse_import_assertion(p).ok();

    m.complete(p, JS_IMPORT_NAMESPACE_CLAUSE)
}

// test import_named_clause
// import {} from "a";
// import { a, b, c, } from "b";
// import e, { f } from "b";
// import g, * as lorem from "c";
// import { f as x, default as w, "a-b-c" as y } from "b";
fn parse_import_named_clause_rest(p: &mut Parser, m: Marker) -> CompletedMarker {
    parse_default_import_specifier(p).ok();
    parse_named_import(p).or_add_diagnostic(p, expected_named_import);
    expect_contextual_keyword(p, "from", T![from]);
    parse_module_source(p).or_add_diagnostic(p, expected_module_source);
    parse_import_assertion(p).ok();

    m.complete(p, JS_IMPORT_NAMED_CLAUSE)
}

fn parse_default_import_specifier(p: &mut Parser) -> ParsedSyntax {
    parse_binding(p).map(|binding| {
        let m = binding.precede(p);
        p.expect(T![,]);
        m.complete(p, JS_DEFAULT_IMPORT_SPECIFIER)
    })
}

fn parse_named_import(p: &mut Parser) -> ParsedSyntax {
    match p.cur() {
        T![*] => parse_namespace_import_specifier(p),
        _ => parse_named_import_specifier_list(p),
    }
}

fn parse_namespace_import_specifier(p: &mut Parser) -> ParsedSyntax {
    if !p.at(T![*]) {
        return Absent;
    }

    let m = p.start();
    p.bump_any();
    expect_contextual_keyword(p, "as", T![as]);
    parse_binding(p).or_add_diagnostic(p, expected_binding);

    Present(m.complete(p, JS_NAMESPACE_IMPORT_SPECIFIER))
}

fn parse_named_import_specifier_list(p: &mut Parser) -> ParsedSyntax {
    if !p.at(T!['{']) {
        return Absent;
    }

    let m = p.start();
    p.bump(T!['{']);
    NamedImportSpecifierList.parse_list(p);
    p.expect(T!['}']);

    Present(m.complete(p, JS_NAMED_IMPORT_SPECIFIERS))
}

struct NamedImportSpecifierList;

impl ParseSeparatedList for NamedImportSpecifierList {
    fn parse_element(&mut self, p: &mut Parser) -> ParsedSyntax {
        parse_any_named_import_specifier(p)
    }

    fn is_at_list_end(&mut self, p: &mut Parser) -> bool {
        p.at(T!['}'])
    }

    fn recover(&mut self, p: &mut Parser, parsed_element: ParsedSyntax) -> RecoveryResult {
        parsed_element.or_recover(
            p,
            &ParseRecovery::new(
                JS_UNKNOWN_NAMED_IMPORT_SPECIFIER,
                STMT_RECOVERY_SET.union(token_set![T![,], T!['}'], T![;]]),
            )
            .enable_recovery_on_line_break(),
            expected_named_import_specifier,
        )
    }

    fn list_kind() -> JsSyntaxKind {
        JS_NAMED_IMPORT_SPECIFIER_LIST
    }

    fn separating_element_kind(&mut self) -> JsSyntaxKind {
        T![,]
    }

    fn allow_trailing_separating_element(&self) -> bool {
        true
    }
}

// test ts ts_named_import_specifier_with_type
// import { type, type as } from "./mod";
// import { type as other } from "./mod"
// import { type as as } from "./mod";
// import { type as as as } from "./mod"
// import { type "test-abcd" as test } from "./mod";
//
// test_err ts ts_named_import_specifier_error
// import { default } from "./mod";
// import { type default } from "./mod";
// import { "literal-name" } from "./mod";
// import { type "literal-name" } from "./mod";
fn parse_any_named_import_specifier(p: &mut Parser) -> ParsedSyntax {
    if !is_nth_at_literal_export_name(p, 0) {
        // covers `type` and `as` too
        return Absent;
    }

    let m = p.start();

    let metadata = specifier_metadata(
        p,
        is_nth_at_literal_export_name,
        is_nth_at_identifier_binding,
    );

    if metadata.is_type {
        expect_contextual_keyword(p, "type", T![type]);
    }

    let specifier = if metadata.has_alias || p.at(JS_STRING_LITERAL) || p.cur().is_keyword() {
        if metadata.is_local_name_missing {
            // test_err import_as_identifier_err
            // import { as c } from "test";
            p.error(expected_literal_export_name(
                p,
                p.cur_tok().start()..p.cur_tok().start(),
            ));
        } else {
            // test import_as_as_as_identifier
            // import { as as as } from "test";
            parse_literal_export_name(p).or_add_diagnostic(p, expected_literal_export_name);
        }

        expect_contextual_keyword(p, "as", T![as]);
        parse_binding(p).or_add_diagnostic(p, expected_binding);
        m.complete(p, JS_NAMED_IMPORT_SPECIFIER)
    } else {
        // test import_as_identifier
        // import { as } from "test";
        parse_binding(p).or_add_diagnostic(p, expected_identifier);
        m.complete(p, JS_SHORTHAND_NAMED_IMPORT_SPECIFIER)
    };

    if metadata.is_type {
        TypeScript.exclusive_syntax(p, specifier, |p, specifier| {
            ts_only_syntax_error(p, "'import { type x ident }'", specifier.range(p))
        })
    } else {
        Present(specifier)
    }
}

// test import_assertion
// import "x" assert { type: "json" }
// import "foo" assert { "type": "json" };
// import foo from "foo.json" assert { type: "json" };
// import {test} from "foo.json" assert { for: "for" }
// import foo_json from "foo.json" assert { type: "json", hasOwnProperty: "true" };
// import "x" assert
// { type: "json" }

// test_err import_assertion_err
// import "foo" assert { type, "json" };
// import "bar" \u{61}ssert { type: "json" };
// import { foo } assert { type: "json" };
// import "lorem"
// assert { type: "json" }
// import foo2 from "foo.json" assert { "type": "json", type: "html", "type": "js" };
// import "x" assert;
// import ipsum from "ipsum.json" assert { type: "json", lazy: true, startAtLine: 1 };
// import { a } from "a.json" assert
fn parse_import_assertion(p: &mut Parser) -> ParsedSyntax {
    if !p.at(T![ident]) || p.cur_src() != "assert" || p.has_linebreak_before_n(0) {
        return Absent;
    }

    let m = p.start();
    p.bump_remap(T![assert]);
    p.expect(T!['{']);

    ImportAssertionList::default().parse_list(p);

    p.expect(T!['}']);

    Present(m.complete(p, JS_IMPORT_ASSERTION))
}

#[derive(Default)]
struct ImportAssertionList {
    assertion_keys: HashMap<String, Range<usize>>,
}

impl ParseSeparatedList for ImportAssertionList {
    fn parse_element(&mut self, p: &mut Parser) -> ParsedSyntax {
        parse_import_assertion_entry(p, &mut self.assertion_keys)
    }

    fn is_at_list_end(&mut self, p: &mut Parser) -> bool {
        p.at(T!['}'])
    }

    fn recover(&mut self, p: &mut Parser, parsed_element: ParsedSyntax) -> RecoveryResult {
        parsed_element.or_recover(
            p,
            &ParseRecovery::new(
                JS_UNKNOWN_IMPORT_ASSERTION_ENTRY,
                STMT_RECOVERY_SET.union(token_set![T![,], T!['}']]),
            )
            .enable_recovery_on_line_break(),
            |p, range| expected_node("import assertion entry", range).to_diagnostic(p),
        )
    }

    fn list_kind() -> JsSyntaxKind {
        JS_IMPORT_ASSERTION_ENTRY_LIST
    }

    fn separating_element_kind(&mut self) -> JsSyntaxKind {
        T![,]
    }

    fn allow_trailing_separating_element(&self) -> bool {
        true
    }
}

fn parse_import_assertion_entry(
    p: &mut Parser,
    seen_assertion_keys: &mut HashMap<String, Range<usize>>,
) -> ParsedSyntax {
    let m = p.start();
    let key_range = p.cur_tok().range();

    let key = match p.cur() {
        JS_STRING_LITERAL => Some(p.cur_src().trim_matches(&['\'', '"'][..])),
        T![ident] => Some(p.cur_src()),
        t if t.is_keyword() => Some(p.cur_src()),
        _ => None,
    }
    .map(String::from);

    match p.cur() {
        JS_STRING_LITERAL | T![ident] => {
            p.bump_any();
        }
        t if t.is_keyword() => {
            p.bump_remap(T![ident]);
        }
        T![:] => {
            p.error(
                expected_any(&["identifier", "string literal"], p.cur_tok().range())
                    .to_diagnostic(p),
            );
        }
        _ => {
            m.abandon(p);
            return Absent;
        }
    };

    let mut valid = true;

    if let Some(key) = key {
        if let Some(first_use) = seen_assertion_keys.get(&key) {
            p.error(duplicate_assertion_keys_error(
                p,
                &key,
                first_use.to_owned(),
                key_range,
            ));
            valid = false;
        } else {
            seen_assertion_keys.insert(key, key_range);
        }
    };

    p.expect(T![:]);
    p.expect(JS_STRING_LITERAL);

    let mut entry = m.complete(p, JS_IMPORT_ASSERTION_ENTRY);

    if !valid {
        entry.change_to_unknown(p);
    }

    Present(entry)
}

// test_err export_err
// export
//
// test_err ts_export_syntax_in_js
// let a, b, c;
// export type { a };
// export { type b };
// export { type c as cc };
// export type { d } from "./d";
// export { type e } from "./e";
// export { type e as ee } from "./e";

// test_err export_huge_function_in_script
// // SCRIPT
// export function A () { return "Kinsmen hot Moria tea serves. Sticky camp spell covering forged they're Oakenshield vines. Admirable relatives march regained wheel Ere eternally on rest parts unhappy? Leave hundreds market's Argonath answered avail grieve doing goodness! Wrong miserable well-wishers wander stood immediately neither Agreed goat poison holes fire? Nobody tosses a Dwarf. Brigands Bilbo Baggins prisoner stinker birthday injuries. Kili's loosened shy spiders till. Gandalf's death was not in vain. Nor would he have you give up hope. Bread kindly ghost Beorn's jelly. Andûril two-faced bitterness biding seemed says drinking splendor feed light unnoticed one! Carven nearest Eärendil fireworks former. Mattress smelling wandering teaching appear taste wise Mithril uprooted winter forebearers wheel. Let's beside Proudfoots succumbed! Excuse Anárion stolen helpless nudge study shown holding form? Changes point Snowbourn material side outer highest eaves flash-flame relic descendant lurking. Thousand death Agreed oppose whole? Glóin head's hurts feasting fight shiny legacy. Thror's broken odds suffice believe well-protected? Rightfully manners begged Maggot's fairer. Unheard-of grog shields sad wondering gardener killed gone Galadriel! Pan Frodo fingers spreads magic parting amount interest idly naked. It's some form of Elvish. I can't read it. Silverwork Wraiths riddled enchantment apple anywhere."; }
pub(super) fn parse_export(p: &mut Parser) -> ParsedSyntax {
    if !p.at(T![export]) {
        return Absent;
    }

    let stmt_start = p.cur_tok().start();
    let m = p.start();
    p.bump(T![export]);

    let clause = if is_at_contextual_keyword(p, "type") && p.nth_at(1, T!['{']) {
        parse_export_named_or_named_from_clause(p)
    } else if is_nth_at_declaration_clause(p, 0) {
        // test export_class_clause
        // export class A {}
        // export class A extends B {}

        // test export_function_clause
        // export function test(a, b) {}
        // export function* test2(a, b) {}
        // export async function test3(a, b, ) {}

        // test ts ts_export_enum_declaration
        // export enum A { X, Y }
        // export const enum B { X, Y }

        // test ts ts_export_interface_declaration
        // export interface A {}

        parse_declaration_clause(p, stmt_start)
    } else {
        match p.cur() {
            T!['{'] => parse_export_named_or_named_from_clause(p),
            T![default] => parse_export_default_clause(p),
            T![*] => parse_export_from_clause(p),
            T![=] => TypeScript.parse_exclusive_syntax(
                p,
                parse_ts_export_assignment_clause,
                |p, clause| ts_only_syntax_error(p, "'export ='", clause.range(p)),
            ),
            T![ident] if is_at_contextual_keyword(p, "from") => parse_export_from_clause(p),
            T![ident]
                if is_at_contextual_keyword(p, "as")
                    && is_nth_at_contextual_keyword(p, 1, "namespace") =>
            {
                TypeScript.parse_exclusive_syntax(
                    p,
                    parse_ts_export_namespace_clause,
                    |p, clause| ts_only_syntax_error(p, "'export as namespace'", clause.range(p)),
                )
            }
            T![ident] if is_at_contextual_keyword(p, "declare") && !p.has_linebreak_before_n(1) => {
                TypeScript.parse_exclusive_syntax(
                    p,
                    |p| parse_ts_export_declare_clause(p, stmt_start),
                    |p, clause| ts_only_syntax_error(p, "'export declare'", clause.range(p)),
                )
            }
            _ if is_nth_at_contextual_keyword(p, 1, "from") => parse_export_from_clause(p),
            _ => Absent,
        }
    };

    clause.or_add_diagnostic(p, expected_export_clause);

    Present(m.complete(p, JS_EXPORT))
}

fn parse_export_named_or_named_from_clause(p: &mut Parser) -> ParsedSyntax {
    let checkpoint = p.checkpoint();
    match parse_export_named_clause(p) {
        Present(_) if is_at_contextual_keyword(p, "from") => {
            p.rewind(checkpoint);
            parse_export_named_from_clause(p)
        }
        t => t,
    }
}

// test export_named_clause
// export { a };
// export { a as z, b as "y", c as default }
// export { as };
//
// test_err export_named_clause_err
// export { default as "b" };
// export { "a" as b };
// export { as b };
// export { a as 5 };
// export { a b c };
//
// test ts ts_export_type_named
// type A = string;
// export type { A };
//
// test_err ts ts_export_type
// export type
fn parse_export_named_clause(p: &mut Parser) -> ParsedSyntax {
    if !p.at(T!['{']) && !is_at_contextual_keyword(p, "type") {
        return Absent;
    }

    let start = p.cur_tok().range().start;
    let m = p.start();

    let has_type = eat_contextual_keyword(p, "type", T![type]);
    p.bump(T!['{']);
    ExportNamedSpecifierList.parse_list(p);
    p.expect(T!['}']);

    semi(p, start..p.cur_tok().range().start);

    let clause = m.complete(p, JS_EXPORT_NAMED_CLAUSE);

    if has_type {
        TypeScript.exclusive_syntax(p, clause, |p, clause| {
            ts_only_syntax_error(p, "'export type' declarations", clause.range(p))
        })
    } else {
        Present(clause)
    }
}

struct ExportNamedSpecifierList;

impl ParseSeparatedList for ExportNamedSpecifierList {
    fn parse_element(&mut self, p: &mut Parser) -> ParsedSyntax {
        parse_any_export_named_specifier(p)
    }

    fn is_at_list_end(&mut self, p: &mut Parser) -> bool {
        p.at(T!['}'])
    }

    fn recover(&mut self, p: &mut Parser, parsed_element: ParsedSyntax) -> RecoveryResult {
        parsed_element.or_recover(
            p,
            &ParseRecovery::new(
                JS_UNKNOWN,
                STMT_RECOVERY_SET.union(token_set![T![,], T!['}'], T![;]]),
            )
            .enable_recovery_on_line_break(),
            expected_export_name_specifier,
        )
    }

    fn list_kind() -> JsSyntaxKind {
        JS_EXPORT_NAMED_SPECIFIER_LIST
    }

    fn separating_element_kind(&mut self) -> JsSyntaxKind {
        T![,]
    }

    fn allow_trailing_separating_element(&self) -> bool {
        true
    }
}

fn parse_any_export_named_specifier(p: &mut Parser) -> ParsedSyntax {
    // covers `type` and `as` too
    if !is_nth_at_reference_identifier(p, 0) {
        return Absent;
    }

    let m = p.start();

    let metadata = specifier_metadata(
        p,
        is_nth_at_reference_identifier,
        is_nth_at_literal_export_name,
    );

    // test ts ts_export_named_type_specifier
    // export { type }
    // export { type type }
    // export { type as somethingElse }
    if metadata.is_type {
        expect_contextual_keyword(p, "type", T![type]);
    }

    // test_err export_as_identifier_err
    // export { as c }

    if metadata.is_local_name_missing {
        p.error(expected_identifier(
            p,
            p.cur_tok().start()..p.cur_tok().start(),
        ));
    } else {
        parse_reference_identifier(p).or_add_diagnostic(p, expected_identifier);
    }

    // test export_as_identifier
    // export { as };
    // export { as as as }
    //
    let specifier = if metadata.has_alias {
        expect_contextual_keyword(p, "as", T![as]);
        parse_literal_export_name(p).or_add_diagnostic(p, expected_literal_export_name);

        m.complete(p, JS_EXPORT_NAMED_SPECIFIER)
    } else {
        m.complete(p, JS_EXPORT_NAMED_SHORTHAND_SPECIFIER)
    };

    if metadata.is_type {
        TypeScript.exclusive_syntax(p, specifier, |p, specifier| {
            ts_only_syntax_error(p, "export { type ident }'", specifier.range(p))
        })
    } else {
        Present(specifier)
    }
}

#[derive(Default, Debug)]
struct SpecifierMetadata {
    // Is this a type export (`export { type test }`) or a regular value export (`export { test }`)
    is_type: bool,
    // Is this an aliased export (`export { t as test }`) or not
    has_alias: bool,
    // For error recovery in case the local name is missing: `export { as test }`
    is_local_name_missing: bool,
}

// test ts ts_export_type_specifier
// let as;
// let type;
// let a;
// export { type };
// export { type as as };
// export { as as as };
// export { type as as as }
// export { type type };
// export { type as };
// export { type a as aa };
fn specifier_metadata<LocalNamePred, AliasPred>(
    p: &Parser,
    is_nth_name: LocalNamePred,
    is_nth_alias: AliasPred,
) -> SpecifierMetadata
where
    LocalNamePred: Fn(&Parser, usize) -> bool,
    AliasPred: Fn(&Parser, usize) -> bool,
{
    let mut metadata = SpecifierMetadata::default();

    // This may be a typed import/export, but it could also be the name of the import/export:
    // ```ts
    // { type}              // name: `type`
    // { type type }        // name: `type`    type-export: `true`
    // { type as }          // name: `as`      type-export: `true`
    // { type as as }       // name: `type`    type-export: `false` (aliased to `as`)
    // { type as as as }    // name: `as`      type-export: `true`, aliased to `as`
    // ```
    if is_at_contextual_keyword(p, "type") {
        // `{ type identifier }`

        if is_nth_at_contextual_keyword(p, 1, "as") {
            // `{ type as ... }`

            if is_nth_at_contextual_keyword(p, 2, "as") {
                metadata.has_alias = true;
                // `{ type as as }`: Type can either be an identifier or the type keyword

                if is_nth_alias(p, 3) {
                    // `{ type as as x }` or `{ type as as "x"}`
                    metadata.is_type = true;
                }
            } else if is_nth_alias(p, 2) {
                // `{ type as x }` or `{ type as "x" }`
                metadata.has_alias = true;
            } else {
                // `{ type as }`
                metadata.is_type = true;
            }
        } else {
            // `{ type x }` or `{ type "x" }` or `{ type x as }`
            metadata.is_type = is_nth_name(p, 1);
            metadata.has_alias = is_nth_at_contextual_keyword(p, 2, "as");
        }
    } else if is_at_contextual_keyword(p, "as") && is_nth_alias(p, 1) {
        metadata.has_alias = true;

        // error recovery case in case someone typed "as x" but forgot the local name.
        // `{ as x }`
        metadata.is_local_name_missing = !is_nth_at_contextual_keyword(p, 1, "as");
    } else if is_nth_at_contextual_keyword(p, 1, "as") {
        // `{ x as ... }`
        metadata.has_alias = true;
    }

    metadata
}

// test export_from_clause
// export * from "a";
// export * as c from "b";
// export * as default from "b"
// export * from "mod" assert { type: "json" }
//
// test_err export_from_clause_err
// export *;
// export * from 5;
// export as from "test";
// export from "test";
fn parse_export_from_clause(p: &mut Parser) -> ParsedSyntax {
    if !p.at(T![*])
        && !is_at_contextual_keyword(p, "from")
        && !is_nth_at_contextual_keyword(p, 1, "from")
    {
        return Absent;
    }

    let start = p.cur_tok().range().start;
    let m = p.start();
    p.expect(T![*]);

    parse_export_as_clause(p).ok();
    expect_contextual_keyword(p, "from", T![from]);
    parse_module_source(p).or_add_diagnostic(p, expected_module_source);
    parse_import_assertion(p).ok();
    semi(p, start..p.cur_tok().range().end);

    Present(m.complete(p, JS_EXPORT_FROM_CLAUSE))
}

// test export_named_from_clause
// export { a, default } from "mod";
// export { a as z, b as "y", c as default } from "mod"
// export { as } from "mod";
// export { default as "b" } from "mod";
// export { "a" as b } from "mod";
// export { a } from "mod" assert { type: "json" }
//
// test_err export_named_from_clause_err
// export { as b } from "mod";
// export { a as 5 } from "mod";
// export { a b c } from "mod";
// export { 5 as b } from "mod";
//
// test ts ts_export_type_named_from
// export type { A } from "a";
fn parse_export_named_from_clause(p: &mut Parser) -> ParsedSyntax {
    if !p.at(T!['{']) && !is_at_contextual_keyword(p, "type") {
        return Absent;
    }

    let start = p.cur_tok().range().start;
    let m = p.start();

    let has_type = eat_contextual_keyword(p, "type", T![type]);

    p.bump(T!['{']);
    ExportNamedFromSpecifierList.parse_list(p);
    p.expect(T!['}']);

    expect_contextual_keyword(p, "from", T![from]);

    parse_module_source(p).or_add_diagnostic(p, expected_module_source);
    parse_import_assertion(p).ok();

    semi(p, start..p.cur_tok().range().start);

    let clause = m.complete(p, JS_EXPORT_NAMED_FROM_CLAUSE);

    if has_type {
        TypeScript.exclusive_syntax(p, clause, |p, clause| {
            ts_only_syntax_error(p, "'export type' declarations", clause.range(p))
        })
    } else {
        Present(clause)
    }
}

struct ExportNamedFromSpecifierList;

impl ParseSeparatedList for ExportNamedFromSpecifierList {
    fn parse_element(&mut self, p: &mut Parser) -> ParsedSyntax {
        parse_export_named_from_specifier(p)
    }

    fn is_at_list_end(&mut self, p: &mut Parser) -> bool {
        p.at(T!['}'])
    }

    fn recover(&mut self, p: &mut Parser, parsed_element: ParsedSyntax) -> RecoveryResult {
        parsed_element.or_recover(
            p,
            &ParseRecovery::new(
                JS_UNKNOWN,
                STMT_RECOVERY_SET.union(token_set![T![,], T!['}'], T![;]]),
            )
            .enable_recovery_on_line_break(),
            expected_literal_export_name,
        )
    }

    fn list_kind() -> JsSyntaxKind {
        JS_EXPORT_NAMED_FROM_SPECIFIER_LIST
    }

    fn separating_element_kind(&mut self) -> JsSyntaxKind {
        T![,]
    }

    fn allow_trailing_separating_element(&self) -> bool {
        true
    }
}

// test ts ts_export_named_from_specifier_with_type
// export { type A } from "a"
// export { type } from "./type";
fn parse_export_named_from_specifier(p: &mut Parser) -> ParsedSyntax {
    // also covers the contextual keywords `type` and `as`
    if !is_nth_at_literal_export_name(p, 0) {
        return Absent;
    }

    let m = p.start();
    let metadata = specifier_metadata(
        p,
        is_nth_at_reference_identifier,
        is_nth_at_literal_export_name,
    );

    if metadata.is_type {
        expect_contextual_keyword(p, "type", T![type]);
    }

    if metadata.is_local_name_missing {
        p.error(expected_literal_export_name(
            p,
            p.cur_tok().start()..p.cur_tok().start(),
        ));
    } else {
        parse_literal_export_name(p).or_add_diagnostic(p, expected_literal_export_name);
    }

    if metadata.has_alias {
        parse_export_as_clause(p).ok();
    }

    let specifier = Present(m.complete(p, JS_EXPORT_NAMED_FROM_SPECIFIER));

    if metadata.is_type {
        TypeScript.exclusive_syntax(p, specifier, |p, specifier| {
            ts_only_syntax_error(p, "export { type ident }''", specifier.range(p))
        })
    } else {
        specifier
    }
}

fn parse_export_default_clause(p: &mut Parser) -> ParsedSyntax {
    if !p.at(T![default]) {
        return Absent;
    }

    let (clause, default_item_kind) = match p.nth(1) {
        T![class] => {
            parse_export_default_declaration_clause(p, ExportDefaultDeclarationKind::Class)
        }
        T![ident] if is_nth_at_contextual_keyword(p, 1, "abstract") && p.nth_at(2, T![class]) => {
            parse_export_default_declaration_clause(p, ExportDefaultDeclarationKind::Class)
        }
        T![function] => {
            parse_export_default_declaration_clause(p, ExportDefaultDeclarationKind::Function)
        }
        T![ident] if is_nth_at_contextual_keyword(p, 1, "async") && p.nth_at(2, T![function]) => {
            parse_export_default_declaration_clause(p, ExportDefaultDeclarationKind::Function)
        }
        T![ident]
            if is_nth_at_contextual_keyword(p, 1, "interface") && !p.has_linebreak_before_n(2) =>
        {
            parse_export_default_declaration_clause(p, ExportDefaultDeclarationKind::Interface)
        }
        T![enum] => parse_export_default_declaration_clause(p, ExportDefaultDeclarationKind::Enum),
        _ => (
            parse_export_default_expression_clause(p),
            ExportDefaultItemKind::Expression,
        ),
    };

    clause.map(|mut clause| {
        // test_err multiple_default_exports_err
        // export default (class {})
        // export default a + b;
        // export default (function a() {})
        if let Some(existing_default_item) = p.state.default_item.as_ref().filter(|_| p.is_module())
        {
            if existing_default_item.kind.is_overload()
                && (default_item_kind.is_overload() || default_item_kind.is_function_declaration())
            {
                // It's ok to have multiple overload declarations and an implementation.
                // This check won't catch if there are multiple implementations for the same overload
                // or if the overloads define different functions.
            } else {
                let err = p
                    .err_builder("Illegal duplicate default export declarations")
                    .secondary(
                        &existing_default_item.range.to_owned(),
                        "the module's default export is first defined here",
                    )
                    .primary(clause.range(p), "multiple default exports are erroneous");

                p.error(err);
                clause.change_kind(p, JsSyntaxKind::JS_UNKNOWN);
            }
        }
        // TypeScript supports multiple `export default interface` They all get merged together

        // test ts ts_export_default_multiple_interfaces
        // export default interface A { a: string; }
        // export default interface B { a: string }
        // export default function test() {}
        else if !default_item_kind.is_interface() {
            p.state.default_item = Some(ExportDefaultItem {
                range: clause.range(p).into(),
                kind: default_item_kind,
            });
        }

        clause
    })
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum ExportDefaultDeclarationKind {
    Function,
    Class,
    Interface,
    // Technically not supported but for better error handling
    Enum,
}

fn parse_export_default_declaration_clause(
    p: &mut Parser,
    kind: ExportDefaultDeclarationKind,
) -> (ParsedSyntax, ExportDefaultItemKind) {
    if !p.at(T![default]) {
        return (Absent, ExportDefaultItemKind::Unknown);
    }

    let m = p.start();
    p.bump(T![default]);

    let declaration = match kind {
        ExportDefaultDeclarationKind::Function => parse_function_export_default_declaration(p),
        ExportDefaultDeclarationKind::Class => parse_class_export_default_declaration(p),

        // test ts ts_export_default_interface
        // export default interface A { }
        ExportDefaultDeclarationKind::Interface => {
            TypeScript.parse_exclusive_syntax(p, parse_ts_interface_declaration, |p, interface| {
                ts_only_syntax_error(p, "interface", interface.range(p).as_range())
            })
        }
        ExportDefaultDeclarationKind::Enum => {
            // test_err ts ts_export_default_enum
            // export default enum A { X, Y, Z }
            parse_ts_enum_declaration(p).map(|enum_declaration| {
                p.error(p.err_builder("'export default' isn't allowed for 'enum's. Move the 'enum' declaration in its own statement and then export the enum's name.")
                    .primary(enum_declaration.range(p), "")
                );

                enum_declaration
            })
        }
    };

    let item_kind = match (kind, declaration.kind()) {
        (ExportDefaultDeclarationKind::Function, Some(TS_DECLARE_FUNCTION_DECLARATION)) => {
            ExportDefaultItemKind::FunctionOverload
        }
        (ExportDefaultDeclarationKind::Function, _) => ExportDefaultItemKind::FunctionDeclaration,
        (ExportDefaultDeclarationKind::Interface, _) => ExportDefaultItemKind::Interface,
        _ => ExportDefaultItemKind::Declaration,
    };

    declaration.or_add_diagnostic(p, |p, range| {
        if TypeScript.is_supported(p) {
            expected_any(
                &[
                    "class declaration",
                    "function declaration",
                    "interface declaration",
                ],
                range,
            )
        } else {
            expected_any(&["class declaration", "function declaration"], range)
        }
    });

    (
        Present(m.complete(p, JS_EXPORT_DEFAULT_DECLARATION_CLAUSE)),
        item_kind,
    )
}

// test export_default_expression_clause
// export default a;
//
// test_err export_default_expression_clause_err
// export default a, b;
fn parse_export_default_expression_clause(p: &mut Parser) -> ParsedSyntax {
    if !p.at(T![default]) && !is_nth_at_expression(p, 1) {
        return Absent;
    }

    let start = p.cur_tok().range().start;
    let m = p.start();
    p.expect(T![default]);

    parse_assignment_expression_or_higher(p, ExpressionContext::default())
        .or_add_diagnostic(p, expected_expression);

    semi(p, start..p.cur_tok().range().start);
    Present(m.complete(p, JS_EXPORT_DEFAULT_EXPRESSION_CLAUSE))
}

fn parse_export_as_clause(p: &mut Parser) -> ParsedSyntax {
    if p.cur_src() != "as" {
        return Absent;
    }

    let m = p.start();
    expect_contextual_keyword(p, "as", T![as]);

    parse_literal_export_name(p).or_add_diagnostic(p, expected_literal_export_name);

    Present(m.complete(p, JS_EXPORT_AS_CLAUSE))
}

// test ts ts_export_namespace_clause
// export function isPrime(x: number): boolean;
// export as namespace mathLib;
fn parse_ts_export_namespace_clause(p: &mut Parser) -> ParsedSyntax {
    if !is_at_contextual_keyword(p, "as") && !is_nth_at_contextual_keyword(p, 1, "namespace") {
        return Absent;
    }

    let m = p.start();
    let start_pos = p.cur_tok().start();
    expect_contextual_keyword(p, "as", T![as]);
    expect_contextual_keyword(p, "namespace", T![namespace]);
    parse_name(p).or_add_diagnostic(p, expected_identifier);
    semi(p, start_pos..p.cur_tok().end());

    Present(m.complete(p, TS_EXPORT_AS_NAMESPACE_CLAUSE))
}

// test ts ts_export_assignment_identifier
// declare const a: { b: string }
// export = a;
// export = class {}
// export = function () {}
// export = 4 + 3 + a;
//
// test ts ts_export_assignment_qualified_name
// declare const a: { b: string }
// export = a.b;
fn parse_ts_export_assignment_clause(p: &mut Parser) -> ParsedSyntax {
    if !p.at(T![=]) {
        return Absent;
    }

    let m = p.start();
    let start_pos = p.cur_tok().start();
    p.bump(T![=]);
    parse_assignment_expression_or_higher(p, ExpressionContext::default())
        .or_add_diagnostic(p, expected_expression);
    semi(p, start_pos..p.cur_tok().end());
    Present(m.complete(p, TS_EXPORT_ASSIGNMENT_CLAUSE))
}

// test ts ts_export_declare
// export declare const a: string;
// export declare interface A {}
// export declare enum B {}
// export declare type C = string;
// export declare class D {}
// export declare function e()
fn parse_ts_export_declare_clause(p: &mut Parser, stmt_start: usize) -> ParsedSyntax {
    if !is_at_contextual_keyword(p, "declare") {
        return Absent;
    }

    let m = p.start();
    expect_contextual_keyword(p, "declare", T![declare]);
    p.with_state(EnterAmbientContext, |p| {
        parse_declaration_clause(p, stmt_start).or_add_diagnostic(p, expected_declaration)
    });

    Present(m.complete(p, TS_EXPORT_DECLARE_CLAUSE))
}

fn is_nth_at_literal_export_name(p: &Parser, n: usize) -> bool {
    match p.nth(n) {
        JS_STRING_LITERAL | T![ident] => true,
        t if t.is_keyword() => true,
        _ => false,
    }
}

fn parse_literal_export_name(p: &mut Parser) -> ParsedSyntax {
    match p.cur() {
        JS_STRING_LITERAL | T![ident] => {
            let m = p.start();
            p.bump_any();
            Present(m.complete(p, JS_LITERAL_EXPORT_NAME))
        }
        t if t.is_keyword() => {
            let m = p.start();
            p.bump_remap(T![ident]);
            Present(m.complete(p, JS_LITERAL_EXPORT_NAME))
        }
        _ => Absent,
    }
}

pub(crate) fn parse_module_source(p: &mut Parser) -> ParsedSyntax {
    if !p.at(JS_STRING_LITERAL) {
        Absent
    } else {
        let m = p.start();
        p.bump_any();
        Present(m.complete(p, JS_MODULE_SOURCE))
    }
}
