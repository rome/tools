use crate::syntax::class::parse_class_declaration;
use crate::syntax::function::parse_function_declaration;
use crate::syntax::module::parse_import_or_import_equals_declaration;
use crate::syntax::stmt::{
    is_nth_at_variable_declarations, parse_variable_declaration, semi, StatementContext,
    VariableDeclarationParent,
};
use crate::syntax::typescript::{
    is_nth_at_any_ts_namespace_declaration, parse_any_ts_namespace_declaration_clause,
    parse_ts_enum_declaration, parse_ts_interface_declaration, parse_ts_type_alias_declaration,
};
use crate::{Absent, ParsedSyntax, Parser};
use rslint_syntax::JsSyntaxKind::JS_VARIABLE_DECLARATION_CLAUSE;
use rslint_syntax::T;

// test export_variable_clause
// export let a;
// export const b = 3;
// export var c, d, e = 3;
//
// test_err export_variable_clause_error
// export let a = ;
// export const b;
// export let d, c;
pub(crate) fn parse_variable_declaration_clause(p: &mut Parser) -> ParsedSyntax {
    let start = p.cur_tok().start();

    parse_variable_declaration(p, VariableDeclarationParent::Clause).map(|declaration| {
        let m = declaration.precede(p);
        semi(p, start..p.cur_tok().end());
        m.complete(p, JS_VARIABLE_DECLARATION_CLAUSE)
    })
}

pub(crate) fn is_nth_at_declaration_clause(p: &Parser, n: usize) -> bool {
    if matches!(
        p.nth(n),
        T![function] | T![const] | T![enum] | T![class] | T![import]
    ) {
        return true;
    }

    if is_nth_at_variable_declarations(p, n) {
        return true;
    }

    if p.has_linebreak_before_n(n + 1) {
        return false;
    }

    if p.nth_at(n, T![type]) || p.nth_at(n, T![interface]) {
        return true;
    }

    if p.nth_at(n, T![async]) && p.nth_at(n + 1, T![function]) {
        return true;
    }

    if is_nth_at_any_ts_namespace_declaration(p, n) {
        return true;
    }

    if p.nth_at(n, T![abstract]) && p.nth_at(n + 1, T![class]) {
        return true;
    }

    false
}

pub(crate) fn parse_declaration_clause(p: &mut Parser, stmt_start_pos: usize) -> ParsedSyntax {
    match p.cur() {
        T![function] => parse_function_declaration(p, StatementContext::StatementList),
        T![class] => parse_class_declaration(p, StatementContext::StatementList),
        T![abstract] => parse_class_declaration(p, StatementContext::StatementList),
        T![const] => {
            if p.nth_at(1, T![enum]) {
                parse_ts_enum_declaration(p)
            } else {
                // test ts ts_ambient_const_variable_statement
                // declare const a, b, c, d = "test";
                parse_variable_declaration_clause(p)
            }
        }
        // test ts ts_ambient_var_statement
        // declare var a, b, c;
        T![var] => parse_variable_declaration_clause(p),
        T![enum] => {
            // test ts ts_ambient_enum_statement
            // declare enum A { X, Y, Z }
            // declare const enum B { X, Y, Z }
            parse_ts_enum_declaration(p)
        }
        T![import] => parse_import_or_import_equals_declaration(p),
        T![async] => parse_function_declaration(p, StatementContext::StatementList),
        T![type] => {
            // test ts ts_declare_type_alias
            // declare type A = string;
            // declare type B = string | number & { a: string, b: number }
            parse_ts_type_alias_declaration(p)
        }
        T![interface] => {
            // test ts ts_ambient_interface
            // declare interface A { b: string, c: number }
            parse_ts_interface_declaration(p)
        }
        T![let] => {
            // test ts ts_ambient_let_variable_statement
            // declare let a, b, c, d;
            parse_variable_declaration_clause(p)
        }
        T![namespace] | T![global] | T![module] => {
            parse_any_ts_namespace_declaration_clause(p, stmt_start_pos)
        }

        _ => Absent,
    }
}
