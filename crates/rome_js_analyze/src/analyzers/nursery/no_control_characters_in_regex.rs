use std::{iter::Peekable, str::Chars};

use rome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{
    JsCallArguments, JsCallExpression, JsIdentifierExpression, JsLanguage, JsNewExpression,
    JsRegexLiteralExpression, JsStringLiteralExpression,
};
use rome_rowan::{declare_node_union, AstNode, AstSeparatedList, SyntaxNode};

use crate::utils::escape_string;

declare_rule! {
 /// Prevents from having control characters and some escape sequences that match control characters in regular expressions.
 ///
 /// ## Examples
 ///
 /// ### Invalid
 /// ```cjs,expect_diagnostic
 ///  var pattern1 = /\x00/;
 ///  var pattern2 = /\x0C/;
 ///  var pattern3 = /\x1F/;
 ///  var pattern4 = /\u000C/;
 ///  var pattern5 = /\u{C}/u;
 ///  var pattern6 = new RegExp("\x0C");
 ///  var pattern7 = new RegExp("\\x0C");
 /// ```
 ///
 /// ### Valid
 /// ```cjs,expect_diagnostic
 /// var pattern1 = /\x20/;
 /// var pattern2 = /\u0020/;
 /// var pattern3 = /\u{20}/u;
 /// var pattern4 = /\t/;
 /// var pattern5 = /\n/;
 /// var pattern6 = new RegExp("\x20");
 /// ```
 pub(crate) NoControlCharactersInRegex {
     version: "11.0.0",
     name: "noControlCharactersInRegex",
     recommended: true,
    }
}

declare_node_union! {
  pub(crate) PossibleRegexExpression = JsNewExpression | JsCallExpression |JsRegexLiteralExpression
}

fn get_code_point_from_hex_character(it: &mut Peekable<Chars>) -> Option<(String, i64)> {
    let mut digits = Vec::new();
    for _ in 1..=2 {
        digits.push(it.next()?);
    }
    let s: String = digits.into_iter().collect();
    let cp = i64::from_str_radix(s.as_str(), 16).ok()?;
    Some((s, cp))
}

fn get_code_point_from_escape_character(it: &mut Peekable<Chars>) -> Option<(String, i64)> {
    let mut digits = Vec::new();
    for _ in 1..=4 {
        if let Some(&c) = it.peek() {
            match c {
                '0'..='9' => digits.push(it.next()?),
                'a'..='f' => digits.push(it.next()?),
                'A'..='F' => digits.push(it.next()?),
                _ => {}
            }
        }
    }

    let s: String = digits.into_iter().collect();
    let cp = i64::from_str_radix(s.as_str(), 16).ok()?;
    Some((s, cp))
}

fn get_code_point_from_code_point_character(it: &mut Peekable<Chars>) -> Option<(String, i64)> {
    let mut digits = Vec::new();
    if let Some(&c) = it.peek() {
        if c == '{' {
            it.next();
            while let Some(&c) = it.peek() {
                match c {
                    '}' => {
                        it.next();
                        let s: String = digits.into_iter().collect();
                        let cp = i64::from_str_radix(s.as_str(), 16).ok()?;
                        return Some((format!("{{{}}}", s), cp));
                    }
                    _ => digits.push(it.next()?),
                }
            }
        }
    }

    None
}
fn collect_control_characters(pattern: String, flags: Option<String>) -> Option<Vec<String>> {
    let mut control_characters: Vec<String> = Vec::new();
    let is_unicode = flags.unwrap_or_default().contains("u");

    let mut it = pattern.chars().peekable();

    let check_control_character =
        |prefix: &str,
         it: &mut Peekable<Chars>,
         control_characters: &mut Vec<String>,
         get_code_point: fn(&mut Peekable<Chars>) -> Option<(String, i64)>| {
            if let Some((s, cp)) = get_code_point(it) {
                if cp >= 0 && cp < 32 {
                    control_characters.push(format!("{}{}", prefix, s));
                }
            }
        };
    while let Some(&c) = it.peek() {
        match c {
            '\\' => {
                it.next();
                if let Some(&c) = it.peek() {
                    match c {
                        'x' => {
                            it.next();
                            check_control_character(
                                "\\x",
                                &mut it,
                                &mut control_characters,
                                get_code_point_from_hex_character,
                            );
                        }
                        'u' => {
                            it.next();
                            if is_unicode {
                                check_control_character(
                                    "\\u",
                                    &mut it,
                                    &mut control_characters,
                                    get_code_point_from_code_point_character,
                                );
                            } else {
                                check_control_character(
                                    "\\u",
                                    &mut it,
                                    &mut control_characters,
                                    get_code_point_from_escape_character,
                                );
                            }
                        }
                        '\\' => {
                            it.next();
                        }
                        _ => {}
                    }
                }
            }
            _ => {
                it.next();
            }
        }
    }
    if control_characters.len() > 0 {
        Some(control_characters)
    } else {
        None
    }
}
fn collect_control_characters_from_expression(
    calee: &SyntaxNode<JsLanguage>,
    js_call_arguments: &JsCallArguments,
) -> Option<Vec<String>> {
    if let Some(js_identifier) = JsIdentifierExpression::cast_ref(calee) {
        if js_identifier.name().ok()?.has_name("RegExp") {
            let mut args = js_call_arguments.args().iter();

            let pattern = escape_string(
                args.next()
                    .and_then(|arg| arg.ok())
                    .and_then(|arg| JsStringLiteralExpression::cast_ref(arg.syntax()))
                    .and_then(|js_string_literal| js_string_literal.inner_string_text().ok())?
                    .to_string()
                    .as_str(),
            )
            .ok()?;

            let flags = args
                .next()
                .and_then(|arg| arg.ok())
                .and_then(|arg| JsStringLiteralExpression::cast_ref(arg.syntax()))
                .and_then(|js_string_literal| Some(js_string_literal.text()));

            return collect_control_characters(pattern, flags);
        }
    }
    None
}

impl Rule for NoControlCharactersInRegex {
    type Query = Ast<PossibleRegexExpression>;
    type State = Vec<String>;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        match node {
            PossibleRegexExpression::JsNewExpression(js_new_expression) => {
                return collect_control_characters_from_expression(
                    js_new_expression.callee().ok()?.syntax(),
                    &js_new_expression.arguments()?,
                );
            }
            PossibleRegexExpression::JsCallExpression(js_call_expression) => {
                return collect_control_characters_from_expression(
                    js_call_expression.callee().ok()?.syntax(),
                    &js_call_expression.arguments().ok()?,
                );
            }
            PossibleRegexExpression::JsRegexLiteralExpression(js_regex_literal_expression) => {
                return collect_control_characters(
                    js_regex_literal_expression.pattern().ok()?,
                    js_regex_literal_expression.flags().ok(),
                )
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diag = RuleDiagnostic::new(
            rule_category!(),
            ctx.query().range(),
            markup! {
                "Unexpected control character(s) in regular expression: "<Emphasis>{state.join(", ")}</Emphasis>""
            },
        );
        Some(diag)
    }
}
