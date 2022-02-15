//! Statements, these include `if`, `while`, `for`, `;`, and more.
//!
//! See the [ECMAScript spec](https://www.ecma-international.org/ecma-262/5.1/#sec-12).

use super::binding::*;
use super::class::is_at_ts_abstract_class_declaration;
use super::expr::parse_expression;
use super::typescript::*;
use crate::parser::{expected_token, ParseNodeList, ParsedSyntax, ParserProgress};
use crate::parser::{RecoveryError, RecoveryResult};
use crate::state::{
    BreakableKind, ChangeParserState, EnableStrictMode, EnableStrictModeSnapshot, EnterBreakable,
    LabelledItem, StrictMode as StrictModeState, WithLabel,
};
use crate::syntax::assignment::expression_to_assignment_pattern;
use crate::syntax::class::{parse_class_declaration, parse_initializer_clause};
use crate::syntax::expr::{
    is_at_expression, is_at_identifier, is_nth_at_identifier,
    parse_assignment_expression_or_higher, parse_expression_or_recover_to_next_statement,
    parse_identifier, ExpressionContext,
};
use crate::syntax::function::{is_at_async_function, parse_function_declaration, LineBreak};
use crate::syntax::js_parse_error;
use crate::syntax::js_parse_error::{expected_binding, expected_statement, ts_only_syntax_error};
use crate::syntax::module::{parse_export, parse_import};
use crate::syntax::util::{is_at_contextual_keyword, is_nth_at_contextual_keyword};
use crate::JsSyntaxFeature::{StrictMode, TypeScript};
use crate::ParsedSyntax::{Absent, Present};
use crate::SyntaxFeature;
use crate::{JsSyntaxKind::*, *};
use rome_rowan::SyntaxKind;
use rslint_errors::Span;

pub const STMT_RECOVERY_SET: TokenSet = token_set![
    L_CURLY,
    VAR_KW,
    FUNCTION_KW,
    IF_KW,
    FOR_KW,
    DO_KW,
    WHILE_KW,
    CONTINUE_KW,
    BREAK_KW,
    RETURN_KW,
    WITH_KW,
    SWITCH_KW,
    THROW_KW,
    TRY_KW,
    DEBUGGER_KW,
    FUNCTION_KW,
    CLASS_KW,
    IMPORT_KW,
    EXPORT_KW,
    T![;]
];

/// Consume an explicit semicolon, or try to automatically insert one,
/// or add an error to the parser if there was none and it could not be inserted
// test semicolons
// let foo = bar;
// let foo2 = b;
// let foo3;
// let foo4
// let foo5
// function foo6() { return true }
pub fn semi(p: &mut Parser, err_range: Range<usize>) -> bool {
    // test_err semicolons_err
    // let foo = bar throw foo

    if !optional_semi(p) {
        let err = p
            .err_builder(
                "Expected a semicolon or an implicit semicolon after a statement, but found none",
            )
            .primary(
                p.cur_tok().range(),
                "An explicit or implicit semicolon is expected here...",
            )
            .secondary(err_range, "...Which is required to end this statement");

        p.error(err);
        false
    } else {
        true
    }
}

/// Eats a semicolon if present but doesn't add an error none is present and the automatic
/// semicolon insertion rule does not apply.
///
/// Returns false if neither a semicolon was present and the current position doesn't allow an automatic
/// semicolon insertion.
pub(crate) fn optional_semi(p: &mut Parser) -> bool {
    if p.eat(T![;]) {
        return true;
    }

    is_semi(p, 0)
}

pub(super) fn is_semi(p: &Parser, offset: usize) -> bool {
    p.nth_at(offset, T![;])
        || p.nth_at(offset, EOF)
        || p.nth_at(offset, T!['}'])
        || p.has_linebreak_before_n(offset)
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum StatementContext {
    If,
    Label,
    Do,
    While,
    With,
    For,
    // Block, Switch consequence, etc.
    StatementList,
}

impl StatementContext {
    pub(crate) fn is_single_statement(&self) -> bool {
        !matches!(self, StatementContext::StatementList)
    }

    pub(crate) fn is_statement_list(&self) -> bool {
        matches!(self, StatementContext::StatementList)
    }
}

/// A generic statement such as a block, if, while, with, etc
///
/// Error handling and recovering happens inside this function, so the
/// caller has to pass a recovery set.
///
/// If not passed, [STMT_RECOVERY_SET] will be used as recovery set
pub(crate) fn parse_statement(p: &mut Parser, context: StatementContext) -> ParsedSyntax {
    match p.cur() {
        // test_err import_decl_not_top_level
        // {
        //  import foo from "bar";
        // }

        // make sure we dont try parsing import.meta or import() as declarations
        T![import] if !token_set![T![.], T!['(']].contains(p.nth(1)) => {
            let mut import = parse_import(p).unwrap();
            import.change_kind(p, JS_UNKNOWN_STATEMENT);

            if p.syntax.file_kind == FileKind::Script {
                let err = p
                    .err_builder("Illegal use of an import declaration outside of a module")
                    .primary(import.range(p), "not allowed inside scripts");

                p.error(err);
            } else {
                let err = p
                    .err_builder("Illegal use of an import declaration not at the top level")
                    .primary(import.range(p), "move this declaration to the top level");

                p.error(err);
            }
            Present(import)
        }
        // test_err export_decl_not_top_level
        // {
        //  export { pain } from "life";
        // }
        T![export] => parse_export(p).map(|mut export| {
            if !p.is_module() && !p.typescript() {
                let err = p
                    .err_builder("Illegal use of an export declaration outside of a module")
                    .primary(export.range(p), "not allowed inside scripts");

                p.error(err);
            } else {
                let err = p
                    .err_builder("Illegal use of an import declaration not at the top level")
                    .primary(export.range(p), "move this declaration to the top level");

                p.error(err);
            }
            export.change_kind(p, JS_UNKNOWN_STATEMENT);
            export
        }),
        T![;] => parse_empty_statement(p),
        T!['{'] => parse_block_stmt(p),
        T![if] => parse_if_statement(p),
        T![with] => parse_with_statement(p),
        T![while] => parse_while_statement(p),
        T![const] | T![enum] if is_at_ts_enum_declaration(p) => {
            // test_err enum_in_js
            // enum A {}
            TypeScript.parse_exclusive_syntax(p, parse_ts_enum_declaration, |p, declaration| {
                ts_only_syntax_error(p, "'enum's", declaration.range(p).as_range())
            })
        }
        T![var] => parse_variable_statement(p, context),
        T![const] => parse_variable_statement(p, context),
        T![for] => parse_for_statement(p),
        T![do] => parse_do_statement(p),
        T![switch] => parse_switch_statement(p),
        T![try] => parse_try_statement(p),
        T![return] => parse_return_statement(p),
        T![break] => parse_break_statement(p),
        T![continue] => parse_continue_statement(p),
        T![throw] => parse_throw_statement(p),
        T![debugger] => parse_debugger_statement(p),
        // function and async function
        T![function] => parse_function_declaration(p, context),
        T![ident] if is_at_async_function(p, LineBreak::DoCheck) => {
            parse_function_declaration(p, context)
        }
        // class and abstract class
        T![class] => parse_class_declaration(p, context),
        T![ident] if is_at_ts_abstract_class_declaration(p, LineBreak::DoCheck) => {
            let abstract_class = parse_class_declaration(p, context);

            // test_err abstract_class_in_js
            // abstract class A {}
            if !p.typescript() {
                abstract_class.map(|mut abstract_class| {
                    p.error(p.err_builder(
                        "`abstract` classes can only be declared in TypeScript files",
                    ));
                    abstract_class.change_to_unknown(p);
                    abstract_class
                })
            } else {
                abstract_class
            }
        }
        T![ident] | T![await] | T![yield] | T![enum] if p.nth_at(1, T![:]) => {
            parse_labeled_statement(p, context)
        }
        T![ident] if is_nth_at_let_variable_statement(p, 0) => {
            // test_err let_newline_in_async_function
            // async function f() {
            //   let
            //   await 0;
            // }

            // test let_asi_rule
            // // SCRIPT
            // let // NO ASI
            // x = 1;
            // for await (var x of []) let // ASI
            // x = 1;

            // test_err let_array_with_new_line
            // // SCRIPT
            // L: let
            // [a] = 0;
            if p.nth_at(1, T!['[']) || context.is_statement_list() || !p.has_linebreak_before_n(1) {
                parse_variable_statement(p, context)
            } else {
                parse_expression_statement(p)
            }
        }
        // contextual keywords that expect to be followed by another token on the same line
        T![ident] if !p.has_linebreak_before_n(1) => {
            if is_at_async_function(p, LineBreak::DoNotCheck) {
                parse_function_declaration(p, context)
            } else if is_at_contextual_keyword(p, "type") && is_nth_at_identifier(p, 1) {
                // test ts ts_type_variable
                // let type;
                // type = getFlowTypeInConstructor(symbol, getDeclaringConstructor(symbol)!);
                TypeScript.parse_exclusive_syntax(
                    p,
                    parse_ts_type_alias_declaration,
                    |p, type_alias| {
                        ts_only_syntax_error(p, "type alias", type_alias.range(p).as_range())
                    },
                )
            } else if is_at_ts_interface_declaration(p) {
                TypeScript.parse_exclusive_syntax(
                    p,
                    parse_ts_interface_declaration,
                    |p, interface| {
                        ts_only_syntax_error(p, "interface", interface.range(p).as_range())
                    },
                )
            } else if is_at_ts_declare_statement(p) {
                let declare_range = p.cur_tok().range();
                TypeScript.parse_exclusive_syntax(p, parse_ts_declare_statement, |p, _| {
                    p.err_builder("The 'declare' modifier can only be used in TypeScript files.")
                        .primary(declare_range, "")
                })
            } else if is_at_any_ts_namespace_declaration(p) {
                let name = p.cur_tok().range();
                TypeScript.parse_exclusive_syntax(
                    p,
                    parse_any_ts_namespace_declaration_statement,
                    |p, declaration| {
                        ts_only_syntax_error(
                            p,
                            p.source(name.as_text_range()),
                            declaration.range(p).as_range(),
                        )
                    },
                )
            } else {
                parse_expression_statement(p)
            }
        }
        _ if is_at_expression(p) => parse_expression_statement(p),
        _ => Absent,
    }
}

// test labeled_statement
// label1: 1
// label1: 1
// label2: 2
//
// test_err double_label
// label1: {
//   label2: {
//     label1: {}
//   }
// }
//
// test labelled_function_declaration
// // SCRIPT
// label1: function a() {}
//
// test_err labelled_function_declaration_strict_mode
// label1: function a() {}
fn parse_labeled_statement(p: &mut Parser, context: StatementContext) -> ParsedSyntax {
    parse_identifier(p, JS_LABELED_STATEMENT).map(|identifier| {
		fn parse_body(p: &mut Parser, context: StatementContext) -> ParsedSyntax {
			if is_at_identifier(p) && p.nth_at(1, T![:]) && StrictMode.is_unsupported(p) {
				// Re-use the parent context to catch `if (true) label1: label2: function A() {}
				parse_labeled_statement(p, context)
			} else {
				parse_statement(p, StatementContext::Label)
			}
		}

		p.bump(T![:]);

		let identifier_range = identifier.range(p);
		let is_valid_identifier = !identifier.kind().is_unknown();
		let labelled_statement = identifier.undo_completion(p);
		let label = p.source(identifier_range);

		let body = match p.state.get_labelled_item(label) {
			None => {
				let labelled_item = match p.cur() {
					T![for] | T![do] | T![while] => LabelledItem::Iteration(identifier_range.as_range()),
					_ => LabelledItem::Other(identifier_range.as_range())
				};
				let change = WithLabel(String::from(label), labelled_item);
				p.with_state(change, |p| parse_body(p, context))
			},
			Some(label_item) if is_valid_identifier => {
				let err = p
					.err_builder("Duplicate statement labels are not allowed")
					.secondary(
						label_item.range().to_owned(),
						&format!("`{}` is first used as a label here", label),
					)
					.primary(
						identifier_range,
						&format!("a second use of `{}` here is not allowed", label),
					);

				p.error(err);
				parse_body(p, context)
			},
			Some(_) => {
				// Don't add another error, the identifier is already invalid
				parse_body(p, context)
			}
		};

        match body.or_add_diagnostic(p, expected_statement) {
            Some(mut body) if context.is_single_statement() && body.kind() == JS_FUNCTION_DECLARATION => {
                // test_err labelled_function_decl_in_single_statement_context
                // if (true) label1: label2: function a() {}
                p.error(p.err_builder("Labelled function declarations are only allowed at top-level or inside a block").primary(body.range(p), "Wrap the labelled statement in a block statement"));
                body.change_to_unknown(p);
            },
            // test labelled_statement_in_single_statement_context
            // if (true) label1: var a = 10;
            _ => {}
        }

        labelled_statement.complete(p, JS_LABELED_STATEMENT)
    })
}

// test ts_keyword_assignments
// declare = 1;
// abstract = 2;
// namespace = 3;
// type = 4;
// module = 5;
// global = 6;
//
// test ts_keywords_assignments_script
// // SCRIPT
// interface = 1;
// private = 2;
// protected = 3;
// public = 4;
// implements = 5;
fn parse_expression_statement(p: &mut Parser) -> ParsedSyntax {
    let start = p.cur_tok().start();

    let expr =
        parse_expression_or_recover_to_next_statement(p, false, ExpressionContext::default());

    if let Ok(expr) = expr {
        let m = expr.precede(p);
        semi(p, start..p.cur_tok().end());
        Present(m.complete(p, JS_EXPRESSION_STATEMENT))
    } else {
        Absent
    }
}

// test debugger_stmt
// debugger;

// test_err debugger_stmt
// function foo() {
//   debugger {
//     var something = "lorem";
//   }
// }

/// A debugger statement such as `debugger;`
fn parse_debugger_statement(p: &mut Parser) -> ParsedSyntax {
    if !p.at(T![debugger]) {
        return Absent;
    }
    let m = p.start();
    let range = p.cur_tok().range();
    p.bump_any(); // debugger keyword
    semi(p, range);
    Present(m.complete(p, JS_DEBUGGER_STATEMENT))
}

/// A throw statement such as `throw new Error("uh oh");`
// test throw_stmt
// throw new Error("foo");
// throw "foo"
fn parse_throw_statement(p: &mut Parser) -> ParsedSyntax {
    // test_err throw_stmt_err
    // throw
    // new Error("oh no :(")
    // throw;
    if !p.at(T![throw]) {
        return Absent;
    }
    let m = p.start();
    let start = p.cur_tok().start();
    p.bump_any(); // throw keyword
    if p.has_linebreak_before_n(0) {
        let mut err = p
            .err_builder(
                "Linebreaks between a throw statement and the error to be thrown are not allowed",
            )
            .primary(p.cur_tok().range(), "A linebreak is not allowed here");

        if is_at_expression(p) {
            err = err.secondary(p.cur_tok().range(), "Help: did you mean to throw this?");
        }

        p.error(err);
    } else {
        parse_expression_or_recover_to_next_statement(p, false, ExpressionContext::default()).ok();
    }

    semi(p, start..p.cur_tok().end());
    Present(m.complete(p, JS_THROW_STATEMENT))
}

// test break_stmt
// while (true) {
//   break;
//   foo: {
//    break foo;
//   }
// }

// test_err break_stmt
// function foo() { break; }
// while (true) {
//   break foo;
// }

/// A break statement with an optional label such as `break a;`
fn parse_break_statement(p: &mut Parser) -> ParsedSyntax {
    if !p.at(T![break]) {
        return Absent;
    }
    let m = p.start();
    let start = p.cur_tok().range();
    p.bump_any(); // break keyword

    let error = if !p.has_linebreak_before_n(0) && p.at(T![ident]) {
        let label_token = p.cur_tok();
        let label_name = p.token_src(label_token);

        let error = match p.state.get_labelled_item(label_name) {
            Some(_) => None,
            None => Some(
                p.err_builder(&format!(
                    "Use of undefined statement label `{}`",
                    label_name
                ))
                .primary(
                    label_token.range(),
                    "This label is used, but it is never defined",
                ),
            ),
        };

        p.bump_any();
        error
    } else if !p.state.break_allowed() {
        Some(p.err_builder("A `break` statement can only be used within an enclosing iteration or switch statement.")
			.primary(start.clone(), ""))
    } else {
        None
    };

    semi(p, start.start..p.cur_tok().end());

    if let Some(error) = error {
        p.error(error);
        Present(m.complete(p, JS_UNKNOWN_STATEMENT))
    } else {
        Present(m.complete(p, JS_BREAK_STATEMENT))
    }
}

// test continue_stmt
// outer: while(true) {
// while (true) {
//   continue;
//     continue outer;
//    }
//   continue
// }

// test_err continue_stmt
// function foo() { continue; }
// while (true) {
//   continue foo;
// }
// foo: {
//   continue foo;
// }
/// A continue statement with an optional label such as `continue a;`
fn parse_continue_statement(p: &mut Parser) -> ParsedSyntax {
    if !p.at(T![continue]) {
        return Absent;
    }
    let m = p.start();
    let start = p.cur_tok().range();
    p.bump_any(); // continue keyword

    let error = if !p.has_linebreak_before_n(0) && p.at(T![ident]) {
        let label_token = p.cur_tok();
        let label_name = p.token_src(label_token);

        let error = match p.state.get_labelled_item(label_name) {
			Some(LabelledItem::Iteration(_)) => None,
			Some(LabelledItem::Other(range)) => {
				Some(p.err_builder("A `continue` statement can only jump to a label of an enclosing `for`, `while` or `do while` statement.")
					.primary(label_token.range(), "This label")
					.secondary(range.to_owned(), "points to non-iteration statement"))
			}
			None => {
				Some(p
					.err_builder(&format!(
						"Use of undefined statement label `{}`",
						label_name
					))
					.primary(
						label_token.range(),
						"This label is used, but it is never defined",
					))
			}
		};

        p.bump_any();

        error
    } else if !p.state.continue_allowed() {
        Some(
            p.err_builder(
                "A `continue` statement can only be used within an enclosing `for`, `while` or `do while` statement.",
            )
            .primary(start.clone(), ""),
        )
    } else {
        None
    };

    semi(p, start.start..p.cur_tok().end());

    if let Some(error) = error {
        p.error(error);
        Present(m.complete(p, JS_UNKNOWN_STATEMENT))
    } else {
        Present(m.complete(p, JS_CONTINUE_STATEMENT))
    }
}

// test return_stmt
// () => {
//   return;
//   return foo;
//   return
// }
/// A return statement with an optional value such as `return a;`
fn parse_return_statement(p: &mut Parser) -> ParsedSyntax {
    // test_err return_stmt_err
    // return;
    // return foo;
    if !p.at(T![return]) {
        return Absent;
    }
    let m = p.start();
    let start = p.cur_tok().start();
    p.bump_any(); // return keyword
    if !p.has_linebreak_before_n(0) {
        parse_expression(p, ExpressionContext::default()).ok();
    }

    semi(p, start..p.cur_tok().end());
    let mut complete = m.complete(p, JS_RETURN_STATEMENT);

    if !p.state.in_function() && !p.syntax.global_return {
        let err = p
            .err_builder("Illegal return statement outside of a function")
            .primary(complete.range(p), "");

        p.error(err);
        complete.change_kind(p, JS_UNKNOWN_STATEMENT);
    }
    Present(complete)
}

// test empty_stmt
// ;
/// An empty statement denoted by a single semicolon.
fn parse_empty_statement(p: &mut Parser) -> ParsedSyntax {
    if p.at(T![;]) {
        let m = p.start();
        p.bump_any(); // bump ;
        m.complete(p, JS_EMPTY_STATEMENT).into()
    } else {
        Absent
    }
}

// test block_stmt
// {}
// {{{{}}}}
// { foo = bar; }
/// A block statement consisting of statements wrapped in curly brackets.
pub(crate) fn parse_block_stmt(p: &mut Parser) -> ParsedSyntax {
    parse_block_impl(p, JS_BLOCK_STATEMENT)
}

/// A block wrapped in curly brackets. Can either be a function body or a block statement.
pub(super) fn parse_block_impl(p: &mut Parser, block_kind: JsSyntaxKind) -> ParsedSyntax {
    if !p.at(T!['{']) {
        return Absent;
    }

    let m = p.start();
    p.bump(T!['{']);

    let strict_snapshot = if block_kind == JS_FUNCTION_BODY {
        directives(p)
    } else {
        None
    };

    parse_statements(p, true);

    p.expect(T!['}']);

    if let Some(strict_snapshot) = strict_snapshot {
        EnableStrictMode::restore(&mut p.state, strict_snapshot);
    }

    Present(m.complete(p, block_kind))
}

#[derive(Default)]
struct DirectivesList {
    strict_snapshot: Option<EnableStrictModeSnapshot>,
}

impl DirectivesList {
    fn is_at_directives(&self, p: &mut Parser) -> bool {
        if !p.at(JS_STRING_LITERAL) {
            false
        } else {
            let next = p.nth(1);

            matches!(next, T![;] | EOF | T!['}']) || p.has_linebreak_before_n(1)
        }
    }
}

impl ParseNodeList for DirectivesList {
    fn parse_element(&mut self, p: &mut Parser) -> ParsedSyntax {
        let directive_token = p.cur_tok();

        let directive = p.start();
        // bump string token
        p.bump_any();

        // eat semicolon if present, correct termination guaranteed by is_directive
        p.eat(JsSyntaxKind::SEMICOLON);

        let completed_marker = directive.complete(p, JS_DIRECTIVE);

        let directive_text = p.token_src(directive_token);

        if directive_text == "\"use strict\"" || directive_text == "'use strict'" {
            if let Some(strict) = p.state.strict() {
                let mut err = p.warning_builder("Redundant strict mode declaration");

                match strict {
                    StrictModeState::Explicit(prev_range) => {
                        err = err.secondary(prev_range, "strict mode is previous declared here");
                    }
                    StrictModeState::Module => {
                        err = err.footer_note("modules are always strict mode");
                    }
                    StrictModeState::Class(prev_range) => {
                        err = err.secondary(prev_range, "class bodies are always strict mode");
                    }
                }

                err = err.primary(directive_token.range(), "this declaration is redundant");
                p.error(err);
            } else if self.strict_snapshot.is_none() {
                self.strict_snapshot = Some(
                    EnableStrictMode(StrictModeState::Explicit(directive_token.range()))
                        .apply(&mut p.state),
                );
            }
        }

        Present(completed_marker)
    }

    fn is_at_list_end(&mut self, p: &mut Parser) -> bool {
        !self.is_at_directives(p)
    }

    fn recover(&mut self, _p: &mut Parser, _parsed_element: ParsedSyntax) -> RecoveryResult {
        // directives don't need proper error recovery
        Err(RecoveryError::AlreadyRecovered)
    }

    fn list_kind() -> JsSyntaxKind {
        JS_DIRECTIVE_LIST
    }
}

// test directives
// // SCRIPT
// "use new"
// let a = 10;
// "use strict"; // not a directive
// function test() {
//   'use strict';
//   let b = 10;
//   'use strict'; // not a directive
// }
// (function () {
//   "use strict";
//   let c = 10;
//   "use strict"; // not a directive
// });
// let b = () => {
//   "use strict";
//   let e = 10;
//   "use strict";  // not a directive
// }
// {
//   "use strict"; // not a directive
// }
//
// test_err directives_err
// // SCRIPT
// function test() {
//   "use strict";
//   function inner_a() {
//     "use strict";
//   }
//   function inner_b() {
//     function inner_inner() {
//       "use strict";
//     }
//   }
// }
#[must_use]
pub(crate) fn directives(p: &mut Parser) -> Option<EnableStrictModeSnapshot> {
    let mut list = DirectivesList::default();
    list.parse_list(p);
    list.strict_snapshot
}

/// Top level items or items inside of a block statement, this also handles module items so we can
/// easily recover from erroneous module declarations in scripts
pub(crate) fn parse_statements(p: &mut Parser, stop_on_r_curly: bool) {
    let list_start = p.start();
    let mut progress = ParserProgress::default();

    while !p.at(EOF) {
        progress.assert_progressing(p);
        if stop_on_r_curly && p.at(T!['}']) {
            break;
        }

        if parse_statement(p, StatementContext::StatementList)
            .or_recover(
                p,
                &ParseRecovery::new(JS_UNKNOWN_STATEMENT, STMT_RECOVERY_SET),
                expected_statement,
            )
            .is_err()
        {
            break;
        }
    }

    list_start.complete(p, JS_STATEMENT_LIST);
}

/// An expression wrapped in parentheses such as `()`
/// Returns `true` if the closing parentheses is present
fn parenthesized_expression(p: &mut Parser) -> bool {
    let has_l_paren = p.expect(T!['(']);

    parse_expression(
        p,
        ExpressionContext::default().and_object_expression_allowed(has_l_paren),
    )
    .or_add_diagnostic(p, js_parse_error::expected_expression);

    p.expect(T![')'])
}

/// An if statement such as `if (foo) { bar(); }`
// test if_stmt
// if (true) {} else {}
// if (true) {}
// if (true) false
// if (bar) {} else if (true) {} else {}
fn parse_if_statement(p: &mut Parser) -> ParsedSyntax {
    // test_err if_stmt_err
    // if (true) else {}
    // if (true) else
    // if else {}
    // if () {} else {}
    // if (true)}}}} {}
    if !p.at(T![if]) {
        return Absent;
    }

    let m = p.start();
    p.bump_any(); // bump if

    // (test)
    parenthesized_expression(p);

    // body
    parse_statement(p, StatementContext::If).or_add_diagnostic(p, expected_statement);

    // else clause
    if p.at(T![else]) {
        let else_clause = p.start();
        p.bump_any(); // bump else
        parse_statement(p, StatementContext::If).or_add_diagnostic(p, expected_statement);
        else_clause.complete(p, JS_ELSE_CLAUSE);
    }

    Present(m.complete(p, JS_IF_STATEMENT))
}

// test with_statement
// // SCRIPT
// function f(x, o) {
//   with (o) {
//     console.log(x);
//   }
// }
/// A with statement such as `with (foo) something()`
fn parse_with_statement(p: &mut Parser) -> ParsedSyntax {
    if !p.at(T![with]) {
        return Absent;
    }

    let m = p.start();
    p.bump_any(); // with
    parenthesized_expression(p);

    parse_statement(p, StatementContext::With).or_add_diagnostic(p, expected_statement);

    let with_stmt = m.complete(p, JS_WITH_STATEMENT);

    // or SloppyMode.exclusive_syntax(...) but this reads better with the error message, saying that
    // it's only forbidden in strict mode
    StrictMode.excluding_syntax(p, with_stmt, |p, marker| {
        p.err_builder("`with` statements are not allowed in strict mode")
            .primary(marker.range(p), "")
    })
}

/// A while statement such as `while(true) { do_something() }`
// test while_stmt
// while (true) {}
// while (5) {}
fn parse_while_statement(p: &mut Parser) -> ParsedSyntax {
    // test_err while_stmt_err
    // while true {}
    // while {}
    // while (true {}
    // while true) }
    if !p.at(T![while]) {
        return Absent;
    }
    let m = p.start();
    p.bump_any(); // while
    parenthesized_expression(p);

    p.with_state(EnterBreakable(BreakableKind::Iteration), |p| {
        parse_statement(p, StatementContext::While)
    })
    .or_add_diagnostic(p, expected_statement);

    Present(m.complete(p, JS_WHILE_STATEMENT))
}

pub(crate) fn is_nth_at_variable_declarations(p: &Parser, n: usize) -> bool {
    match p.nth(n) {
        T![var] | T![const] => true,
        T![ident] if is_nth_at_let_variable_statement(p, n) => true,
        _ => false,
    }
}

pub(crate) fn is_nth_at_let_variable_statement(p: &Parser, n: usize) -> bool {
    is_nth_at_contextual_keyword(p, n, "let")
        && matches!(
            p.nth(n + 1),
            T!['{'] | T!['['] | T![ident] | T![yield] | T![await] | T![enum]
        )
}

/// A var, const, or let declaration statement such as `var a = 5, b;` or `let {a, b} = foo;`
// test var_decl
// var a = 5;
// let { foo, bar } = 5;
// let bar2, foo2;
// const b = 5;
// const { foo5: [bar11], baz6 } = {};
// let foo6 = "lorem", bar7 = "ipsum", third8 = "value", fourth = 6;
// var q, w, e, r, t;
//
// test_err variable_declaration_statement_err
// let a, { b } = { a: 10 }
// const c = 1, { d } = { a: 10 }
// const e;
// let [f];
// const { g };
pub(crate) fn parse_variable_statement(p: &mut Parser, context: StatementContext) -> ParsedSyntax {
    // test_err var_decl_err
    // var a =;
    // const b = 5 let c = 5;
    let start = p.cur_tok().start();
    let is_var = p.at(T![var]);

    parse_variable_declaration(p, VariableDeclarationParent::VariableStatement).map(|declaration| {
        let m = declaration.precede(p);
        semi(p, start..p.cur_tok().start());

        let mut statement = m.complete(p, JS_VARIABLE_STATEMENT);

        if !is_var && context.is_single_statement() {
            // test hoisted_declaration_in_single_statement_context
            // if (true) var a;
            //
            // test_err lexical_declaration_in_single_statement_context
            // if (true) let a;
            // while (true) const b = 5;
            p.error(
                p.err_builder("Lexical declaration cannot appear in a single-statement context")
                    .primary(
                        statement.range(p),
                        "Wrap this declaration in a block statement",
                    ),
            );
            statement.change_to_unknown(p);
        }

        statement
    })
}

pub(super) fn parse_variable_declaration(
    p: &mut Parser,
    declaration_context: VariableDeclarationParent,
) -> ParsedSyntax {
    let m = p.start();
    if eat_variable_declaration(p, declaration_context).is_some() {
        Present(m.complete(p, JS_VARIABLE_DECLARATION))
    } else {
        m.abandon(p);
        Absent
    }
}

/// What's the parent node of the variable declaration
#[repr(u8)]
#[derive(Clone, Debug, Copy, Eq, PartialEq)]
pub(super) enum VariableDeclarationParent {
    /// Declaration inside a `for...of` or `for...in` or `for (;;)` loop
    For,

    /// Declaration as part of a variable statement (`let a`, `const b`, or `var c`).
    VariableStatement,

    /// Declaration as part of another statement, like `export let ...` or `declare let a`
    Clause,
}

/// Parses a variable declaration that consist of a variable kind (`let`, `const` or `var` and a list
/// of variable declarators).
/// Returns a tuple where
/// * the first element is the marker to the not yet completed list
/// * the second element is the range of all variable declarations except the first one. Is [None] if
///   there's only one declaration.
fn eat_variable_declaration(
    p: &mut Parser,
    declaration_parent: VariableDeclarationParent,
) -> Option<(CompletedMarker, Option<Range<usize>>)> {
    let mut context = VariableDeclaratorContext::new(declaration_parent);

    match p.cur() {
        T![var] => p.bump_any(),
        T![const] => {
            context.is_const = Some(p.cur_tok().range());
            p.bump_any()
        }
        T![ident] if p.cur_src() == "let" => {
            // let is a valid identifier name that's why the returns an ident for let.
            // remap it here because we know from the context that this is the let keyword.
            p.bump_remap(T![let]);
            context.is_let = true;
        }
        _ => {
            return None;
        }
    }

    let mut variable_declarator_list = VariableDeclaratorList {
        declarator_context: context,
        remaining_declarator_range: None,
    };

    debug_assert!(p.state.name_map.is_empty());
    let list = variable_declarator_list.parse_list(p);

    p.state.name_map.clear();
    Some((list, variable_declarator_list.remaining_declarator_range))
}

struct VariableDeclaratorList {
    declarator_context: VariableDeclaratorContext,
    // Range of the declarators succeeding the first declarator
    // None until this hits the second declarator
    remaining_declarator_range: Option<Range<usize>>,
}

impl ParseSeparatedList for VariableDeclaratorList {
    fn parse_element(&mut self, p: &mut Parser) -> ParsedSyntax {
        parse_variable_declarator(p, &self.declarator_context).map(|declarator| {
            if self.declarator_context.is_first {
                self.declarator_context.is_first = false;
            } else if let Some(range) = self.remaining_declarator_range.as_mut() {
                range.end = declarator.range(p).as_range().end;
            } else {
                self.remaining_declarator_range = Some(declarator.range(p).as_range());
            }
            declarator
        })
    }

    fn is_at_list_end(&mut self, p: &mut Parser) -> bool {
        if self.declarator_context.is_first {
            false
        } else {
            !p.at(T![,])
        }
    }

    fn recover(&mut self, p: &mut Parser, parsed_element: ParsedSyntax) -> RecoveryResult {
        parsed_element.or_recover(
            p,
            &ParseRecovery::new(JS_UNKNOWN, STMT_RECOVERY_SET.union(token_set!(T![,])))
                .enable_recovery_on_line_break(),
            expected_binding,
        )
    }

    fn list_kind() -> JsSyntaxKind {
        JS_VARIABLE_DECLARATOR_LIST
    }

    fn separating_element_kind(&mut self) -> JsSyntaxKind {
        T![,]
    }
}

struct VariableDeclaratorContext {
    /// The range of the `const` keyword if this is `const` variable declaration.
    is_const: Option<Range<usize>>,
    /// `true` if this is a let variable declaration
    is_let: bool,
    /// Is this the first declaration in the declaration list (a first, b second in `let a, b`)
    is_first: bool,
    /// What's the parent of the variable declaration
    parent: VariableDeclarationParent,
}

impl VariableDeclaratorContext {
    fn new(parent: VariableDeclarationParent) -> Self {
        Self {
            parent,
            is_const: None,
            is_let: false,
            is_first: true,
        }
    }

    fn duplicate_binding_parent_name(&self) -> Option<&'static str> {
        if self.is_const.is_some() {
            Some("const")
        } else if self.is_let {
            Some("let")
        } else {
            None
        }
    }
}

// test scoped_declarations
// let a = {
//   test() {
//     let a = "inner";
//   }
// };
//
// A single declarator, either `ident` or `ident = assign_expr`
fn parse_variable_declarator(p: &mut Parser, context: &VariableDeclaratorContext) -> ParsedSyntax {
    p.state.duplicate_binding_parent = context.duplicate_binding_parent_name();
    let id = parse_binding_pattern(p, ExpressionContext::default());
    p.state.duplicate_binding_parent = None;

    id.map(|id| {
        let m = id.precede(p);

        let ts_annotation = TypeScript.parse_exclusive_syntax(p, parse_ts_variable_annotation,
            |p, annotation| {
                let name = match annotation.kind() {
                    TS_TYPE_ANNOTATION => "type annotation",
                    TS_DEFINITE_VARIABLE_ANNOTATION => "definite assertion assignments",
                    _ => unreachable!(),
                };

                ts_only_syntax_error(p, name, annotation.range(p).as_range())
            })
            .ok();

        let last_name_map = std::mem::take(&mut p.state.name_map);
        let duplicate_binding_parent = p.state.duplicate_binding_parent.take();

        let mut initializer = parse_initializer_clause(
            p,
            ExpressionContext::default()
                .and_include_in(context.parent != VariableDeclarationParent::For),
        )
        .ok();

        if let (Some(initializer), Some(ts_annotation)) =
            (initializer.as_mut(), ts_annotation.as_ref())
        {
            if ts_annotation.kind() == TS_DEFINITE_VARIABLE_ANNOTATION {
                // test_err ts_definite_variable_with_initializer
                // let a!: string = "test";
                p.error(
                    p
                        .err_builder("Declarations with initializers cannot also have definite assignment assertions.")
                        .primary(initializer.range(p), "")
                        .secondary(ts_annotation.range(p), "")
                );
                initializer.change_to_unknown(p);
            }
        }

        p.state.name_map = last_name_map;
        p.state.duplicate_binding_parent = duplicate_binding_parent;

        // Heuristic to determine if we're in a for of or for in loop. This may be off if
        // the user uses a for of/in with multiple declarations but this isn't allowed anyway.
        let is_in_for_loop = context.parent == VariableDeclarationParent::For && context.is_first;
        let is_in_for_of = is_in_for_loop && p.cur_src() == "of";
        let is_in_for_in = is_in_for_loop && p.at(T![in]);

        if is_in_for_of || is_in_for_in {
            if p.typescript() {
                if let Some(mut ts_annotation) = ts_annotation {
                    let err = p
                        .err_builder("`for` statement declarators cannot have a type annotation")
                        .primary(ts_annotation.range(p), "");

                    p.error(err);
                    ts_annotation.change_to_unknown(p);
                }
            }
            if let Some(initializer) = initializer {
                // Initializers are disallowed for `for..in` and `for..of`,
                // except for `for(var ... in ...)` in loose mode

                // test for_in_initializer_loose_mode
                // // SCRIPT
                // for (var i = 0 in []) {}

                // test_err for_in_and_of_initializer_loose_mode
                // // SCRIPT
                // for (let i = 0 in []) {}
                // for (const i = 0 in []) {}
                // for (var i = 0 of []) {}
                // for (let i = 0 of []) {}
                // for (const i = 0 of []) {}

                // test_err for_in_and_of_initializer_strict_mode
                // for (var i = 0 in []) {}
                // for (let i = 0 in []) {}
                // for (const i = 0 in []) {}
                // for (var i = 0 of []) {}
                // for (let i = 0 of []) {}
                // for (const i = 0 of []) {}

                let is_strict = StrictMode.is_supported(p);
                let is_var = !context.is_let && context.is_const.is_none();

                if is_strict || !is_in_for_in || !is_var {
                    let err = p
                        .err_builder(if is_in_for_in {
                            "`for..in` statement declarators cannot have an initializer expression"
                        } else {
                            "`for..of` statement declarators cannot have an initializer expression"
                        })
                        .primary(initializer.range(p), "");

                    p.error(err);
                }
            }
        } else if initializer.is_none()
            && !p.state.in_ambient_context()
            && matches!(
                id.kind(),
                JS_ARRAY_BINDING_PATTERN | JS_OBJECT_BINDING_PATTERN
            )
        {
            let err = p
                .err_builder("Object and Array patterns require initializers")
                .primary(
                    id.range(p),
                    "this pattern is declared, but it is not given an initialized value",
                );

            p.error(err);
        } else if initializer.is_none() && context.is_const.is_some() && !p.state.in_ambient_context() {
            let err = p
                .err_builder("Const var declarations must have an initialized value")
                .primary(id.range(p), "this variable needs to be initialized");

            p.error(err);
        }

        m.complete(p, JS_VARIABLE_DECLARATOR)
    })
}

// test_err js_type_variable_annotation
// let a: string, b!: number;
//
// test_err ts ts_variable_annotation_err
// let a!;
//
// test ts ts_type_variable_annotation
// let a: string = "test", b!: number;
fn parse_ts_variable_annotation(p: &mut Parser) -> ParsedSyntax {
    if !p.at(T![!]) {
        return parse_ts_type_annotation(p);
    }

    let m = p.start();
    p.bump(T![!]);

    parse_ts_type_annotation(p).or_add_diagnostic(p, |_, _| expected_token(T![:]));

    Present(m.complete(p, TS_DEFINITE_VARIABLE_ANNOTATION))
}

// A do.. while statement, such as `do {} while (true)`

// test do_while_statement
// do console.log("test"); while(true)
// do {
//   console.log("test")
// } while (true);
// let a = 1;
// do
// do {
//   a = a + 1
// } while(a < 5)
// while (a < 100)
//
// test do_while_stmt
// do { } while (true)
// do { throw Error("foo") } while (true)
// do { break; } while (true)
fn parse_do_statement(p: &mut Parser) -> ParsedSyntax {
    // test_err do_while_stmt_err
    // do while (true)
    // do while ()
    // do while true

    // test_err do_while_no_continue_break
    // do { } break (continue)
    // do { } continue (break)
    if !p.at(T![do]) {
        return Absent;
    }
    let m = p.start();
    let start = p.cur_tok().start();
    p.bump_any(); // do keyword

    p.with_state(EnterBreakable(BreakableKind::Iteration), |p| {
        parse_statement(p, StatementContext::Do)
    })
    .or_add_diagnostic(p, expected_statement);

    p.expect(T![while]);

    // test do-while-asi
    // do do do ; while (x) while (x) while (x) x = 39;
    // do do ; while (x); while (x) x = 39
    if parenthesized_expression(p) {
        optional_semi(p);
    } else {
        let end_range = p.cur_tok().end();
        semi(p, start..end_range);
    }
    Present(m.complete(p, JS_DO_WHILE_STATEMENT))
}

/// Parses the header of a for statement into the current node and returns whatever it is a for in/of or "regular" for statement
fn parse_for_head(p: &mut Parser, has_l_paren: bool, is_for_await: bool) -> JsSyntaxKind {
    // for (;...
    if p.at(T![;]) {
        parse_normal_for_head(p);
        return JS_FOR_STATEMENT;
    }

    // `for (let...` | `for (const...` | `for (var...`

    if p.at(T![const]) || p.at(T![var]) || is_nth_at_let_variable_statement(p, 0) {
        let m = p.start();

        let (declarations, additional_declarations) =
            eat_variable_declaration(p, VariableDeclarationParent::For).unwrap();

        let is_in = p.at(T![in]);
        let is_of = p.cur_src() == "of";

        if is_in || is_of {
            // remove the intermediate list node created by parse variable declarations that is not needed
            // for a ForInOrOfInitializer where the variable declaration is a direct child.
            declarations.undo_completion(p).abandon(p);

            if let Some(additional_declarations_range) = additional_declarations {
                p.error(
                    p.err_builder(&format!(
                        "Only a single declaration is allowed in a `for...{}` statement.",
                        if is_of { "of" } else { "in" },
                    ))
                    .primary(additional_declarations_range, "additional declarations"),
                );
            }

            m.complete(p, JS_FOR_VARIABLE_DECLARATION);

            parse_for_of_or_in_head(p)
        } else {
            m.complete(p, JS_VARIABLE_DECLARATION);
            parse_normal_for_head(p);
            JS_FOR_STATEMENT
        }
    } else {
        // for (some_expression`
        let checkpoint = p.checkpoint();

        let starts_with_async_of = p.cur_src() == "async" && p.nth_src(1) == "of";
        let init_expr = parse_expression(
            p,
            ExpressionContext::default()
                .and_include_in(false)
                .and_object_expression_allowed(has_l_paren),
        );

        if p.at(T![in]) || p.cur_src() == "of" {
            // for (assignment_pattern in ...
            if let Present(assignment_expr) = init_expr {
                let mut assignment =
                    expression_to_assignment_pattern(p, assignment_expr, checkpoint);

                if p.typescript()
                    && p.at(T![in])
                    && matches!(
                        assignment.kind(),
                        JS_ARRAY_ASSIGNMENT_PATTERN | JS_OBJECT_ASSIGNMENT_PATTERN
                    )
                {
                    let err = p.err_builder("the left hand side of a `for..in` statement cannot be a destructuring pattern")
						.primary(assignment.range(p), "");
                    p.error(err);
                    assignment.change_to_unknown(p);
                } else if p.cur_src() == "of" && !is_for_await && starts_with_async_of {
                    //  for ( [lookahead ∉ { let, async of }] LeftHandSideExpression[?Yield, ?Await] of AssignmentExpression[+In, ?Yield, ?Await] ) Statement[?Yield, ?Await, ?Return]
                    // [+Await] for await ( [lookahead ≠ let] LeftHandSideExpression[?Yield, ?Await] of AssignmentExpression[+In, ?Yield, ?Await] ) Statement[?Yield, ?Await, ?Return]

                    // test for_await_async_identifier
                    // let async;
                    // async function fn() {
                    //   for await (async of [7]);
                    // }

                    // test_err for_of_async_identifier
                    // let async;
                    // for (async of [1]) ;
                    p.error(
                        p.err_builder(
                            "The left-hand side of a `for...of` statement may not be `async`",
                        )
                        .primary(assignment.range(p), ""),
                    );
                    assignment.change_to_unknown(p);
                }
            }

            return parse_for_of_or_in_head(p);
        }

        init_expr.or_add_diagnostic(p, js_parse_error::expected_expression);

        parse_normal_for_head(p);
        JS_FOR_STATEMENT
    }
}

/// Parses the parenthesized part of a non for in or for of statement
/// Expects to be positioned right after the initializer
fn parse_normal_for_head(p: &mut Parser) {
    p.expect(T![;]);

    if !p.at(T![;]) {
        parse_expression(p, ExpressionContext::default())
            .or_add_diagnostic(p, js_parse_error::expected_expression);
    }

    p.expect(T![;]);

    if !p.at(T![')']) {
        parse_expression(p, ExpressionContext::default())
            .or_add_diagnostic(p, js_parse_error::expected_expression);
    }
}

/// Expects to be positioned right before the of or in keyword
fn parse_for_of_or_in_head(p: &mut Parser) -> JsSyntaxKind {
    let is_in = p.at(T![in]);

    if is_in {
        p.bump_any();
        parse_expression(p, ExpressionContext::default())
            .or_add_diagnostic(p, js_parse_error::expected_expression);

        JS_FOR_IN_STATEMENT
    } else {
        p.bump_remap(T![of]);

        parse_assignment_expression_or_higher(p, ExpressionContext::default())
            .or_add_diagnostic(p, js_parse_error::expected_expression_assignment);

        JS_FOR_OF_STATEMENT
    }
}

/// Either a traditional for statement or a for.. in statement
// test for_stmt
// for (let i = 5; i < 10; i++) {}
// for (let { foo, bar } of {}) {}
// for (foo in {}) {}
// for (;;) {}
// for (let foo of []) {}
// for (let i = 5, j = 6; i < j; ++j) {}
// for await (let a of []) {}
fn parse_for_statement(p: &mut Parser) -> ParsedSyntax {
    // test_err for_stmt_err
    // for ;; {}
    // for let i = 5; i < 10; i++ {}
    // for let i = 5; i < 10; ++i {}
    // for (in []) {}
    // for (let i, j = 6 of []) {}
    // for await (let a in []) {}
    // for await (let i = 0; i < 10; ++i) {}
    // for (let [a];;) {}
    if !p.at(T![for]) {
        return Absent;
    }

    let m = p.start();
    p.bump_any(); // for keyword

    let mut await_range = None;
    if p.at(T![await]) {
        await_range = Some(p.cur_tok().range());
        p.bump_any();
    }

    let has_l_paren = p.expect(T!['(']);
    let kind = parse_for_head(p, has_l_paren, await_range.is_some());
    p.expect(T![')']);

    p.with_state(EnterBreakable(BreakableKind::Iteration), |p| {
        parse_statement(p, StatementContext::For)
    })
    .or_add_diagnostic(p, expected_statement);

    let mut completed = m.complete(p, kind);

    if kind != JS_FOR_OF_STATEMENT {
        if let Some(await_range) = await_range {
            p.error(
                p.err_builder("await can only be used in conjunction with `for...of` statements")
                    .primary(await_range, "Remove the await here")
                    .secondary(
                        completed.range(p),
                        "or convert this to a `for...of` statement",
                    ),
            );
            completed.change_kind(p, JS_UNKNOWN_STATEMENT)
        }
    }

    Present(completed)
}

struct SwitchCaseStatementList;

impl ParseNodeList for SwitchCaseStatementList {
    fn parse_element(&mut self, p: &mut Parser) -> ParsedSyntax {
        parse_statement(p, StatementContext::StatementList)
    }

    fn is_at_list_end(&mut self, p: &mut Parser) -> bool {
        p.at_ts(token_set![T![default], T![case], T!['}']])
    }

    fn recover(&mut self, p: &mut Parser, parsed_element: ParsedSyntax) -> parser::RecoveryResult {
        parsed_element.or_recover(
            p,
            &ParseRecovery::new(JS_UNKNOWN_STATEMENT, STMT_RECOVERY_SET),
            js_parse_error::expected_case,
        )
    }

    fn list_kind() -> JsSyntaxKind {
        JS_STATEMENT_LIST
    }
}

// We return the range in case its a default clause so we can report multiple default clauses in a better way
fn parse_switch_clause(
    p: &mut Parser,
    first_default: &mut Option<CompletedMarker>,
) -> ParsedSyntax {
    let m = p.start();
    match p.cur() {
        T![default] => {
            // in case we have two `default` expression, we mark the second one
            // as `JS_CASE_CLAUSE` where the the "default" keyword is an Unknown node
            let syntax_kind = if first_default.is_some() {
                let discriminant = p.start();
                p.bump_any(); // interpret `default` as the test of the case
                discriminant.complete(p, JS_UNKNOWN_EXPRESSION);
                JS_CASE_CLAUSE
            } else {
                p.bump_any();
                JS_DEFAULT_CLAUSE
            };

            p.expect(T![:]);
            SwitchCaseStatementList.parse_list(p);
            let default = m.complete(p, syntax_kind);
            if first_default.is_some() {
                let err = p
                    .err_builder(
                        "Multiple default clauses inside of a switch statement are not allowed",
                    )
                    .secondary(
                        first_default.unwrap().range(p),
                        "the first default clause is defined here",
                    )
                    .primary(default.range(p), "a second clause here is not allowed");

                p.error(err);
            }

            Present(default)
        }
        T![case] => {
            p.bump_any();
            parse_expression(p, ExpressionContext::default())
                .or_add_diagnostic(p, js_parse_error::expected_expression);
            p.expect(T![:]);

            SwitchCaseStatementList.parse_list(p);
            Present(m.complete(p, JS_CASE_CLAUSE))
        }
        _ => {
            m.abandon(p);
            Absent
        }
    }
}
#[derive(Default)]
struct SwitchCasesList {
    first_default: Option<CompletedMarker>,
}

impl ParseNodeList for SwitchCasesList {
    fn parse_element(&mut self, p: &mut Parser) -> ParsedSyntax {
        let clause = parse_switch_clause(p, &mut self.first_default);

        if let Present(marker) = clause {
            if marker.kind() == JS_DEFAULT_CLAUSE && self.first_default == None {
                self.first_default = Some(marker);
            }
        }

        clause
    }

    fn is_at_list_end(&mut self, p: &mut Parser) -> bool {
        p.at(T!['}'])
    }

    fn recover(&mut self, p: &mut Parser, parsed_element: ParsedSyntax) -> RecoveryResult {
        if let Present(marker) = parsed_element {
            Ok(marker)
        } else {
            let m = p.start();
            let statements = p.start();

            let recovered_element = parsed_element.or_recover(
                p,
                &ParseRecovery::new(
                    JS_UNKNOWN_STATEMENT,
                    token_set![T![default], T![case], T!['}']],
                )
                .enable_recovery_on_line_break(),
                js_parse_error::expected_case_or_default,
            );

            match recovered_element {
                Ok(marker) => {
                    statements.complete(p, JS_STATEMENT_LIST);
                    m.complete(p, JS_CASE_CLAUSE);
                    Ok(marker)
                }
                Err(err) => {
                    statements.abandon(p);
                    m.abandon(p);
                    Err(err)
                }
            }
        }
    }

    fn list_kind() -> JsSyntaxKind {
        JS_SWITCH_CASE_LIST
    }
}

/// A switch statement such as
///
/// ```js
/// switch (a) {
///     case foo:
///         bar();
/// }
/// ```
// test switch_stmt
// switch (foo) {
//  case bar:
//  default:
// }
fn parse_switch_statement(p: &mut Parser) -> ParsedSyntax {
    // test_err switch_stmt_err
    // switch foo {}
    // switch {}
    // switch { var i = 0 }
    // switch { var i = 0; case "bar": {} }
    // switch (foo) {
    //   default: {}
    //   default: {}
    // }
    // switch (foo) { case : }

    if !p.at(T![switch]) {
        return Absent;
    }
    let m = p.start();
    p.bump_any(); // switch keyword
    parenthesized_expression(p);
    p.expect(T!['{']);

    p.with_state(EnterBreakable(BreakableKind::Switch), |p| {
        SwitchCasesList::default().parse_list(p)
    });

    p.expect(T!['}']);
    Present(m.complete(p, JS_SWITCH_STATEMENT))
}

fn parse_catch_clause(p: &mut Parser) -> ParsedSyntax {
    if !p.at(T![catch]) {
        return Absent;
    }

    let m = p.start();
    p.bump_any(); // bump catch

    parse_catch_declaration(p).ok();
    parse_block_stmt(p).or_add_diagnostic(p, js_parse_error::expected_block_statement);

    Present(m.complete(p, JS_CATCH_CLAUSE))
}

fn parse_catch_declaration(p: &mut Parser) -> ParsedSyntax {
    if !p.at(T!['(']) {
        return Absent;
    }

    let declaration_marker = p.start();

    p.bump_any(); // bump (
    parse_binding_pattern(p, ExpressionContext::default()).or_add_diagnostic(p, expected_binding);

    let type_annotation = parse_ts_type_parameters(p);
    JsSyntaxFeature::TypeScript
        .exclusive_syntax(p, type_annotation, |p, annotation| {
            ts_only_syntax_error(p, "type annotation", annotation.range(p).as_range())
        })
        .ok();

    p.expect(T![')']);

    Present(declaration_marker.complete(p, JS_CATCH_DECLARATION))
}

/// A try statement such as
///
/// ```js
/// try {
///     something();
/// } catch (a) {
///
/// }
/// ```
// test try_stmt
// try {} catch {}
// try {} catch (e) {}
// try {} catch {} finally {}
// try {} catch (e) {} finally {}
// try {} finally {}
pub fn parse_try_statement(p: &mut Parser) -> ParsedSyntax {
    // TODO: recover from `try catch` and `try finally`. The issue is block_items
    // will cause infinite recursion because parsing a stmt would not consume the catch token
    // and block_items would not exit, and if we exited on any error that would greatly limit
    // block_items error recovery

    if !p.at(T![try]) {
        return Absent;
    }

    let m = p.start();
    p.bump_any(); // eat try

    parse_block_stmt(p).or_add_diagnostic(p, js_parse_error::expected_block_statement);

    let catch = parse_catch_clause(p);

    if p.at(T![finally]) {
        catch.ok();

        let finalizer = p.start();
        p.bump_any();
        parse_block_stmt(p).or_add_diagnostic(p, js_parse_error::expected_block_statement);
        finalizer.complete(p, JS_FINALLY_CLAUSE);
        Present(m.complete(p, JS_TRY_FINALLY_STATEMENT))
    } else {
        catch.or_add_diagnostic(p, js_parse_error::expected_catch_clause);
        Present(m.complete(p, JS_TRY_STATEMENT))
    }
}
