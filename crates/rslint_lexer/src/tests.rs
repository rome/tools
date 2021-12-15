#![cfg(test)]
#![allow(unused_mut, unused_variables, unused_assignments)]

use crate::Lexer;
use quickcheck_macros::quickcheck;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

// Assert the result of lexing a piece of source code,
// and make sure the tokens yielded are fully lossless and the source can be reconstructed from only the tokens
macro_rules! assert_lex {
    ($src:expr, $($kind:ident:$len:expr $(,)?)*) => {{
        let mut lexer = Lexer::from_str($src, 0);
        let mut tokens = lexer.collect::<Vec<_>>();
        let mut idx = 0;
        let mut tok_idx = 0;

        let mut new_str = String::with_capacity($src.len());
        // remove eof
        tokens.pop();

        $(
            assert_eq!(
                tokens[idx].0.kind,
                rslint_syntax::SyntaxKind::$kind,
                "expected token kind {}, but found {:?}",
                stringify!($kind),
                tokens[idx].0.kind,
            );

            assert_eq!(
                tokens[idx].0.len,
                $len,
                "expected token length of {}, but found {} for token {:?}",
                $len,
                tokens[idx].0.len,
                tokens[idx].0.kind,
            );

            new_str.push_str($src.get(tok_idx..(tok_idx + tokens[idx].0.len)).unwrap());
            tok_idx += tokens[idx].0.len;

            idx += 1;
        )*

        assert_eq!($src, new_str, "Failed to reconstruct input");
        assert_eq!(idx, tokens.len());
    }};
}

// This is for testing if the lexer is truly lossless
// It parses random strings and puts them back together with the produced tokens and compares
#[quickcheck]
fn losslessness(string: String) -> bool {
	// using an mpsc channel allows us to spawn a thread and spawn the lexer there, then if
	// it takes more than 2 seconds we panic because it is 100% infinite recursion
	let cloned = string.clone();
	let (sender, receiver) = channel();
	thread::spawn(move || {
		sender
			.send(Lexer::from_str(&cloned, 0).map(|x| x.0).collect::<Vec<_>>())
			.expect("Could not send tokens to receiver");
	});
	let tokens = receiver
		.recv_timeout(Duration::from_secs(2))
		.unwrap_or_else(|_| {
			panic!(
				"Lexer is infinitely recursing with this code: ->{}<-",
				string
			)
		});

	let mut new_str = String::with_capacity(string.len());
	let mut idx = 0;

	for token in tokens {
		new_str.push_str(string.get(idx..(idx + token.len)).unwrap());
		idx += token.len;
	}

	string == new_str
}

#[test]
fn empty() {
	assert_lex! {
		"",
	}
}

#[test]
fn identifier() {
	assert_lex! {
		"Abcdefg",
		IDENT:7
	}
}

#[test]
fn punctuators() {
	assert_lex! {
		"!%%&()*+,-.:;<=>?[]^{}|~",
		BANG:1,
		PERCENT:1,
		PERCENT:1,
		AMP:1,
		L_PAREN:1,
		R_PAREN:1,
		STAR:1,
		PLUS:1,
		COMMA:1,
		MINUS:1,
		DOT:1,
		COLON:1,
		SEMICOLON:1,
		LTEQ:2,
		R_ANGLE:1,
		QUESTION:1,
		L_BRACK:1,
		R_BRACK:1,
		CARET:1,
		L_CURLY:1,
		R_CURLY:1,
		PIPE:1,
		TILDE:1,
	}
}

#[test]
fn consecutive_punctuators() {
	assert_lex! {
		"&&&&^^^||",
		AMP2:2,
		AMP2:2,
		CARET:1,
		CARET:1,
		CARET:1,
		PIPE2:2,
	}
}

#[test]
fn unicode_whitespace() {
	assert_lex! {
		" \u{00a0}\u{1680}\u{2000}\u{2001}\u{2002}\u{2003}\u{2004}\u{2005}\u{2006}\u{2007}\u{2008}\u{2009}\u{200A}\u{202F}\u{205F}\u{3000}",
		WHITESPACE:48
	}
}

#[test]
fn unicode_whitespace_ident_part() {
	assert_lex! {
		"Abcd\u{2006}",
		IDENT:4,
		WHITESPACE:3 // length is in bytes
	}
}

#[test]
fn all_whitespace() {
	assert_lex! {
		"
		",
		WHITESPACE:3
	}
}

#[test]
fn empty_string() {
	assert_lex! {
		r#""""#,
		JS_STRING_LITERAL:2
	}

	assert_lex! {
		"''",
		JS_STRING_LITERAL:2
	}
}

#[test]
fn template_literals() {
	assert_lex! {
		"`abcdefg` `abc",
		BACKTICK:1,
		TEMPLATE_CHUNK:7,
		BACKTICK:1,
		WHITESPACE:1,
		BACKTICK:1,
		TEMPLATE_CHUNK:3,
	}

	assert_lex! {
		"`${a} a`",
		BACKTICK:1,
		DOLLAR_CURLY:2,
		IDENT:1,
		R_CURLY:1,
		TEMPLATE_CHUNK:2,
		BACKTICK:1
	}

	assert_lex! {
		"`${a} b ${b}`",
		BACKTICK:1,
		DOLLAR_CURLY:2,
		IDENT:1,
		R_CURLY:1,
		TEMPLATE_CHUNK:3,
		DOLLAR_CURLY:2,
		IDENT:1,
		R_CURLY:1,
		BACKTICK:1
	}
}

#[test]
fn simple_string() {
	assert_lex! {
		r#"'abcdefghijklmnopqrstuvwxyz123456789\'10🦀'"#,
		JS_STRING_LITERAL:45
	}

	assert_lex! {
		r#""abcdefghijklmnopqrstuvwxyz123456789\"10🦀""#,
		JS_STRING_LITERAL:45
	}
}

#[test]
fn string_unicode_escape_invalid() {
	assert_lex! {
		r#""abcd\u21""#,
		ERROR_TOKEN:10
	}

	assert_lex! {
		r#"'abcd\u21'"#,
		ERROR_TOKEN:10
	}
}

#[test]
fn string_unicode_escape_valid() {
	assert_lex! {
		r#""abcd\u2000a""#,
		JS_STRING_LITERAL:13
	}

	assert_lex! {
		r#"'abcd\u2000a'"#,
		JS_STRING_LITERAL:13
	}
}

#[test]
fn string_unicode_escape_valid_resolving_to_endquote() {
	assert_lex! {
		r#""abcd\u0022a""#,
		JS_STRING_LITERAL:13
	}

	assert_lex! {
		r#"'abcd\u0027a'"#,
		JS_STRING_LITERAL:13
	}
}

#[test]
fn string_hex_escape_invalid() {
	assert_lex! {
		r#""abcd \xZ0 \xGH""#,
		ERROR_TOKEN:16
	}

	assert_lex! {
		r#"'abcd \xZ0 \xGH'"#,
		ERROR_TOKEN:16
	}
}

#[test]
fn string_hex_escape_valid() {
	assert_lex! {
		r#""abcd \x00 \xAB""#,
		JS_STRING_LITERAL:16
	}

	assert_lex! {
		r#"'abcd \x00 \xAB'"#,
		JS_STRING_LITERAL:16
	}
}

#[test]
fn unterminated_string() {
	assert_lex! {
		r#""abcd"#,
		ERROR_TOKEN:5
	}

	assert_lex! {
		r#"'abcd"#,
		ERROR_TOKEN:5
	}
}

#[test]
fn string_all_escapes() {
	assert_lex! {
		r#""\x\u2004\u20\ux\xNN""#,
		ERROR_TOKEN:21
	}

	assert_lex! {
		r#"'\x\u2004\u20\ux\xNN'"#,
		ERROR_TOKEN:21
	}
}

#[test]
fn complex_string_1() {
	assert_lex! {
		r#" _this += "str'n\u200bg";"#,
		WHITESPACE:1,
		IDENT:5,
		WHITESPACE:1,
		PLUSEQ:2,
		WHITESPACE:1,
		JS_STRING_LITERAL:14,
		SEMICOLON:1
	}

	assert_lex! {
		r#" _this += 'str"n\u200bg';"#,
		WHITESPACE:1,
		IDENT:5,
		WHITESPACE:1,
		PLUSEQ:2,
		WHITESPACE:1,
		JS_STRING_LITERAL:14,
		SEMICOLON:1
	}
}

#[test]
fn unterminated_string_length() {
	assert_lex! {
		"'abc",
		ERROR_TOKEN:4
	}
}

#[test]
fn unterminated_string_with_escape_len() {
	assert_lex! {
		"'abc\\",
		ERROR_TOKEN:5
	}

	assert_lex! {
		r#"'abc\x"#,
		ERROR_TOKEN:6
	}

	assert_lex! {
		r#"'abc\x4"#,
		ERROR_TOKEN:7
	}

	assert_lex! {
		r#"'abc\x45"#,
		ERROR_TOKEN:8
	}

	assert_lex! {
		r#"'abc\u"#,
		ERROR_TOKEN:6
	}

	assert_lex! {
		r#"'abc\u20"#,
		ERROR_TOKEN:8
	}
}

#[test]
fn dollarsign_underscore_idents() {
	assert_lex! {
		"$a",
		IDENT:2
	}
}

#[test]
fn labels_a() {
	assert_lex! {
		"await",
		AWAIT_KW:5
	}

	assert_lex! {
		"awaited",
		IDENT:7
	}
}

#[test]
fn labels_b() {
	assert_lex! {
		"break",
		BREAK_KW:5
	}

	assert_lex! {
		"breaking speed records",
		IDENT:8,
		WHITESPACE:1,
		IDENT:5,
		WHITESPACE:1,
		IDENT:7
	}
}

#[test]
fn labels_c() {
	assert_lex! {
		"continue, const, class, catch, case",
		CONTINUE_KW:8,
		COMMA:1,
		WHITESPACE:1,
		CONST_KW:5,
		COMMA:1,
		WHITESPACE:1,
		CLASS_KW:5,
		COMMA:1,
		WHITESPACE:1,
		CATCH_KW:5,
		COMMA:1,
		WHITESPACE:1,
		CASE_KW:4
	}

	assert_lex! {
		"classy crabs",
		IDENT:6,
		WHITESPACE:1,
		IDENT:5
	}
}

#[test]
fn labels_d() {
	assert_lex! {
		"debugger default delete do",
		DEBUGGER_KW:8,
		WHITESPACE:1,
		DEFAULT_KW:7,
		WHITESPACE:1,
		DELETE_KW:6,
		WHITESPACE:1,
		DO_KW:2
	}

	assert_lex! {
		"derive doot d",
		IDENT:6,
		WHITESPACE:1,
		IDENT:4,
		WHITESPACE:1,
		IDENT:1
	}
}

#[test]
fn labels_e() {
	assert_lex! {
		"else enum export extends",
		ELSE_KW:4,
		WHITESPACE:1,
		ENUM_KW:4,
		WHITESPACE:1,
		EXPORT_KW:6,
		WHITESPACE:1,
		EXTENDS_KW:7
	}

	assert_lex! {
		"e exports elsey",
		IDENT:1,
		WHITESPACE:1,
		IDENT:7,
		WHITESPACE:1,
		IDENT:5
	}
}

#[test]
fn labels_f() {
	assert_lex! {
		"finally for function",
		FINALLY_KW:7,
		WHITESPACE:1,
		FOR_KW:3,
		WHITESPACE:1,
		FUNCTION_KW:8
	}

	assert_lex! {
		"finally, foreign food!",
		FINALLY_KW:7,
		COMMA:1,
		WHITESPACE:1,
		IDENT:7,
		WHITESPACE:1,
		IDENT:4,
		BANG:1
	}
}

#[test]
fn labels_i() {
	assert_lex! {
		"i in instanceof if import",
		IDENT:1,
		WHITESPACE:1,
		IN_KW: 2,
		WHITESPACE:1,
		INSTANCEOF_KW:10,
		WHITESPACE:1,
		IF_KW:2,
		WHITESPACE:1,
		IMPORT_KW:6
	}

	assert_lex! {
		"icecream is interesting, innit?",
		IDENT:8,
		WHITESPACE:1,
		IDENT:2,
		WHITESPACE:1,
		IDENT:11,
		COMMA:1,
		WHITESPACE:1,
		IDENT:5,
		QUESTION:1
	}
}

#[test]
fn labels_n() {
	assert_lex! {
		"new",
		NEW_KW:3
	}

	assert_lex! {
		"newly n",
		IDENT:5,
		WHITESPACE:1,
		IDENT:1
	}
}

#[test]
fn labels_r() {
	assert_lex! {
		"return",
		RETURN_KW:6
	}

	assert_lex! {
		"returning",
		IDENT:9
	}
}

#[test]
fn labels_s() {
	assert_lex! {
		"switch super",
		SWITCH_KW:6,
		WHITESPACE:1,
		SUPER_KW:5
	}

	assert_lex! {
		"superb switching",
		IDENT:6,
		WHITESPACE:1,
		IDENT:9
	}
}

#[test]
fn labels_t() {
	assert_lex! {
		"this try throw typeof t",
		THIS_KW:4,
		WHITESPACE:1,
		TRY_KW:3,
		WHITESPACE:1,
		THROW_KW:5,
		WHITESPACE:1,
		TYPEOF_KW:6,
		WHITESPACE:1,
		IDENT:1
	}

	assert_lex! {
		"thistle throwing tea",
		IDENT:7,
		WHITESPACE:1,
		IDENT:8,
		WHITESPACE:1,
		IDENT:3
	}
}

#[test]
fn labels_v() {
	assert_lex! {
		"var void v",
		VAR_KW:3,
		WHITESPACE:1,
		VOID_KW:4,
		WHITESPACE:1,
		IDENT:1
	}

	assert_lex! {
		"variable voiding is bad",
		IDENT:8,
		WHITESPACE:1,
		IDENT:7,
		WHITESPACE:1,
		IDENT:2,
		WHITESPACE:1,
		IDENT:3
	}
}

#[test]
fn labels_w() {
	assert_lex! {
		"with while w",
		WITH_KW:4,
		WHITESPACE:1,
		WHILE_KW:5,
		WHITESPACE:1,
		IDENT:1
	}

	assert_lex! {
		"whiley withow",
		IDENT:6,
		WHITESPACE:1,
		IDENT:6
	}
}

#[test]
fn labels_y() {
	assert_lex! {
		"yield",
		YIELD_KW:5
	}

	assert_lex! {
		"yielding",
		IDENT:8
	}
}

#[test]
fn number_basic() {
	assert_lex! {
		"1",
		JS_NUMBER_LITERAL:1
	}

	assert_lex! {
		"123456 ",
		JS_NUMBER_LITERAL:6,
		WHITESPACE:1
	}

	assert_lex! {
		"90",
		JS_NUMBER_LITERAL:2
	}

	assert_lex! {
		".13",
		JS_NUMBER_LITERAL:3
	}
}

#[test]
fn number_basic_err() {
	assert_lex! {
		"2_?",
		JS_NUMBER_LITERAL:2, // numeric separator error
		QUESTION:1
	}

	assert_lex! {
		r#"25\u0046abcdef"#,
		ERROR_TOKEN:14
	}

	assert_lex! {
		r#"25\uFEFFb"#,
		JS_NUMBER_LITERAL:2,
		ERROR_TOKEN:6,
		IDENT:1
	}

	assert_lex! {
		r#".32\u0046abde"#,
		ERROR_TOKEN:13
	}
}

#[test]
fn number_complex() {
	assert_lex! {
		"3e-5 123e+56",
		JS_NUMBER_LITERAL:4,
		WHITESPACE:1,
		JS_NUMBER_LITERAL:7
	}

	assert_lex! {
		"3.14159e+1",
		JS_NUMBER_LITERAL:10
	}

	assert_lex! {
		".0e34",
		JS_NUMBER_LITERAL:5
	}
}

#[test]
fn dot_number_disambiguation() {
	assert_lex! {
		".e+5",
		DOT:1,
		IDENT:1,
		PLUS:1,
		JS_NUMBER_LITERAL:1
	}

	assert_lex! {
		".0e+5",
		JS_NUMBER_LITERAL:5
	}
}

#[test]
fn binary_literals() {
	assert_lex! {
		"0b10101010, 0B10101010, 0b10101010n",
		JS_NUMBER_LITERAL:10,
		COMMA:1,
		WHITESPACE:1,
		JS_NUMBER_LITERAL:10,
		COMMA:1,
		WHITESPACE:1,
		JS_NUMBER_LITERAL:11
	}
}

#[test]
fn octal_literals() {
	assert_lex! {
		"0o01742242, 0B10101010, 0b10101010n",
		JS_NUMBER_LITERAL:10,
		COMMA:1,
		WHITESPACE:1,
		JS_NUMBER_LITERAL:10,
		COMMA:1,
		WHITESPACE:1,
		JS_NUMBER_LITERAL:11
	}
}

#[test]
fn bigint_literals() {
	assert_lex! {
		"0n 1743642n 1n",
		JS_NUMBER_LITERAL:2,
		WHITESPACE:1,
		JS_NUMBER_LITERAL:8,
		WHITESPACE:1,
		JS_NUMBER_LITERAL:2
	}
}

#[test]
fn shebang() {
	assert_lex! {
		"#! /bin/node",
		JS_SHEBANG:12
	}

	assert_lex! {
		"#!/bin/node\n",
		JS_SHEBANG:11,
		WHITESPACE:1
	}

	assert_lex! {
		"#!/usr/bin/env deno\u{2028}",
		JS_SHEBANG:19,
		WHITESPACE:3
	}

	assert_lex! {
		"#0",
		ERROR_TOKEN:1,
		JS_NUMBER_LITERAL:1
	}

	assert_lex! {
		"0#!/bin/deno",
		JS_NUMBER_LITERAL:1,
		HASH:1,
		BANG:1,
		JS_REGEX_LITERAL:9
	}
}

#[test]
fn single_line_comments() {
	assert_lex! {
		"//abc
	",
		COMMENT:5,
		WHITESPACE:2
	}

	assert_lex! {
		"//a",
		COMMENT:3
	}
}

#[test]
fn block_comment() {
	assert_lex! {
		"/*
		*/",
		COMMENT:7
	}

	assert_lex! {
		"/* */",
		COMMENT:5
	}

	assert_lex! {
		"/* *",
		COMMENT:4
	}
}

#[test]
fn regex() {
	assert_lex! {
		"var a = /aa/gim",
		VAR_KW:3,
		WHITESPACE:1,
		IDENT:1,
		WHITESPACE:1,
		EQ:1,
		WHITESPACE:1,
		JS_REGEX_LITERAL:7
	}
}

#[test]
fn division() {
	assert_lex! {
		"var a = 5 / 6",
		VAR_KW:3,
		WHITESPACE:1,
		IDENT:1,
		WHITESPACE:1,
		EQ:1,
		WHITESPACE:1,
		JS_NUMBER_LITERAL:1,
		WHITESPACE:1,
		SLASH:1,
		WHITESPACE:1,
		JS_NUMBER_LITERAL:1
	}
}

#[test]
fn template_escape() {
	assert_lex! {
		r"`foo \` bar`",
		BACKTICK:1,
		TEMPLATE_CHUNK:10,
		BACKTICK:1
	}
}

#[test]
fn fuzz_fail_1() {
	assert_lex! {
		"$\\u",
		IDENT:1,
		ERROR_TOKEN:2
	}
}

#[test]
fn fuzz_fail_2() {
	assert_lex! {
		"..",
		DOT:1,
		DOT:1
	}
}

#[test]
fn fuzz_fail_3() {
	assert_lex! {
		"0e",
		ERROR_TOKEN:2
	}
}

#[test]
fn fuzz_fail_4() {
	assert_lex! {
		"0o 0b 0x",
		ERROR_TOKEN:2,
		WHITESPACE:1,
		ERROR_TOKEN:2,
		WHITESPACE:1,
		ERROR_TOKEN:2
	}
}

#[test]
fn fuzz_fail_5() {
	assert_lex! {
		"//\u{2028}",
		COMMENT:2,
		WHITESPACE:3
	}
}

#[test]
fn fuzz_fail_6() {
	assert_lex! {
		"//\u{200a}",
		COMMENT:5
	}
}

#[test]
fn unicode_ident_start_handling() {
	assert_lex! {
		"αβeta_tester",
		IDENT:14
	}
}

#[test]
fn unicode_ident_separated_by_unicode_whitespace() {
	assert_lex! {
		"β\u{FEFF}α",
		IDENT:2,
		WHITESPACE:3,
		IDENT:2
	}
}

#[test]
fn issue_30() {
	assert_lex! {
		"let foo = { α: true }",
		IDENT:3,
		WHITESPACE:1,
		IDENT:3,
		WHITESPACE:1,
		EQ:1,
		WHITESPACE:1,
		L_CURLY:1,
		WHITESPACE:1,
		IDENT:2,
		COLON:1,
		WHITESPACE:1,
		TRUE_KW:4,
		WHITESPACE:1,
		R_CURLY:1
	}
}

#[test]
fn fuzz_fail_7() {
	assert_lex! {
		"/\u{0}/ª\u{80}",
		JS_REGEX_LITERAL:5,
		ERROR_TOKEN:2
	}
}

#[test]
fn at_token() {
	assert_lex! {
		"@",
		AT:1
	}

	assert_lex! {
		"@foo",
		AT:1,
		IDENT:3
	}
}

#[test]
fn object_expr_getter() {
	assert_lex! {
		"({ get [foo]() {} })",
		L_PAREN:1
		L_CURLY:1
		WHITESPACE:1
		IDENT:3
		WHITESPACE:1
		L_BRACK:1
		IDENT:3
		R_BRACK:1
		L_PAREN:1
		R_PAREN:1
		WHITESPACE:1
		L_CURLY:1
		R_CURLY:1
		WHITESPACE:1
		R_CURLY:1
		R_PAREN:1
	}
}

#[test]
fn newline_space_must_be_two_tokens() {
	assert_lex! {
		"\n ",
		WHITESPACE:2
	}
	assert_lex! {
		" \n",
		WHITESPACE:1
		WHITESPACE:1
	}
	assert_lex! {
		" \n ",
		WHITESPACE:1
		WHITESPACE:2
	}

	assert_lex! {
		" a\n b \n ",
		WHITESPACE:1
		IDENT:1
		WHITESPACE:2
		IDENT:1
		WHITESPACE:1
		WHITESPACE:2
	}
	assert_lex! {
		"a //COMMENT \n /*COMMENT*/ b /*COM\nMENT*/",
		IDENT:1
		WHITESPACE:1
		COMMENT:10
		WHITESPACE:2
		COMMENT:11
		WHITESPACE:1
		IDENT:1
		WHITESPACE:1
		COMMENT:12
	}
	assert_lex! {
		"a //COMMENT \n /*COMMENT*/ b /*COM\nMENT*/",
		IDENT:1
		WHITESPACE:1
		COMMENT:10
		WHITESPACE:2
		COMMENT:11
		WHITESPACE:1
		IDENT:1
		WHITESPACE:1
		COMMENT:12
	}

	//Now with CR
	assert_lex! {
		"\r\n ",
		WHITESPACE:3
	}

	assert_lex! {
		" \r\n",
		WHITESPACE:1
		WHITESPACE:2
	}
	assert_lex! {
		" \r\n ",
		WHITESPACE:1
		WHITESPACE:3
	}

	assert_lex! {
		" a\r\n b \r\n ",
		WHITESPACE:1
		IDENT:1
		WHITESPACE:3
		IDENT:1
		WHITESPACE:1
		WHITESPACE:3
	}
	assert_lex! {
		"a //COMMENT \r\n /*COMMENT*/ b /*COM\r\nMENT*/",
		IDENT:1
		WHITESPACE:1
		COMMENT:10
		WHITESPACE:3
		COMMENT:11
		WHITESPACE:1
		IDENT:1
		WHITESPACE:1
		COMMENT:13
	}
	assert_lex! {
		"a //COMMENT \r\n /*COMMENT*/ b /*COM\r\nMENT*/",
		IDENT:1
		WHITESPACE:1
		COMMENT:10
		WHITESPACE:3
		COMMENT:11
		WHITESPACE:1
		IDENT:1
		WHITESPACE:1
		COMMENT:13
	}
}
