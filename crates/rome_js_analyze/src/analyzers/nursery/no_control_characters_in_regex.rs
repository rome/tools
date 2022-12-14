use std::{iter::Peekable, str::Chars};

use rome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{JsNewExpression, JsRegexLiteralExpression};
use rome_rowan::{declare_node_union, AstNode};

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
  pub(crate) PossibleRegexExpression = JsNewExpression | JsRegexLiteralExpression
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

    if let Some(&c) = it.peek() {
        for _ in 1..=4 {
            digits.push(it.next()?);
        }

        let s: String = digits.into_iter().collect();
        let cp = i64::from_str_radix(s.as_str(), 16).ok()?;
        return Some((s, cp));
    }

    None
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
fn collect_control_characters(pattern: String, flags: String) -> Option<Vec<String>> {
    let mut control_characters: Vec<String> = Vec::new();

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
                            if flags.contains('u') {
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

impl Rule for NoControlCharactersInRegex {
    type Query = Ast<PossibleRegexExpression>;
    type State = Vec<String>;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let ele = JsRegexLiteralExpression::cast(node.syntax().clone()).unwrap();
        let pattern = ele.pattern().unwrap();
        collect_control_characters(pattern, ele.flags().unwrap())
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
