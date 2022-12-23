use std::{iter::Peekable, str::Chars};

use rome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{
    AnyJsExpression, JsCallArguments, JsCallExpression, JsNewExpression, JsRegexLiteralExpression,
    JsStringLiteralExpression,
};
use rome_rowan::{declare_node_union, AstNode, AstSeparatedList};

use crate::utils::escape_string;

declare_rule! {
 /// Prevents from having control characters and some escape sequences that match control characters in regular expressions.
 ///
 /// ## Examples
 ///
 /// ### Invalid
 /// ```cjs,expect_diagnostic
 ///  var pattern1 = /\x00/;
 /// ```
 /// ```cjs,expect_diagnostic
 ///  var pattern2 = /\x0C/;
 /// ```
 /// ```cjs,expect_diagnostic
 ///  var pattern3 = /\x1F/;
 /// ```
 /// ```cjs,expect_diagnostic
 ///  var pattern4 = /\u000C/;
 /// ```
 /// ```cjs,expect_diagnostic
 ///  var pattern5 = /\u{C}/u;
 /// ```
 /// ```cjs,expect_diagnostic
 ///  var pattern7 = new RegExp("\x0C");
 /// ```
 /// ```cjs,expect_diagnostic
 ///  var pattern7 = new RegExp("\\x0C");
 /// ```
 ///
 /// ### Valid
 /// ```cjs
 /// var pattern1 = /\x20/;
 /// var pattern2 = /\u0020/;
 /// var pattern3 = /\u{20}/u;
 /// var pattern4 = /\t/;
 /// var pattern5 = /\n/;
 /// var pattern6 = new RegExp("\x20");
 /// ```
 pub(crate) NoControlCharactersInRegex {
     version: "next",
     name: "noControlCharactersInRegex",
     recommended: true,
    }
}

declare_node_union! {
  pub(crate) PossibleRegexExpression = JsNewExpression | JsCallExpression |JsRegexLiteralExpression
}

fn get_code_point_from_hex_character(iter: &mut Peekable<Chars>) -> Option<(String, i64)> {
    let mut digits = String::new();
    for _ in 1..=2 {
        digits.push(iter.next()?);
    }
    let cp = i64::from_str_radix(digits.as_str(), 16).ok()?;
    Some((digits, cp))
}

fn get_code_point_from_escape_character(iter: &mut Peekable<Chars>) -> Option<(String, i64)> {
    let mut digits = String::new();
    for _ in 1..=4 {
        if let Some(&c) = iter.peek() {
            match c {
                '0'..='9' => digits.push(iter.next()?),
                'a'..='f' => digits.push(iter.next()?),
                'A'..='F' => digits.push(iter.next()?),
                _ => {}
            }
        }
    }

    let cp = i64::from_str_radix(digits.as_str(), 16).ok()?;
    Some((digits, cp))
}

fn get_code_point_from_code_point_character(iter: &mut Peekable<Chars>) -> Option<(String, i64)> {
    let mut digits = String::new();
    if let Some(&c) = iter.peek() {
        if c == '{' {
            iter.next();
            while let Some(&c) = iter.peek() {
                match c {
                    '}' => {
                        iter.next();
                        let cp = i64::from_str_radix(digits.as_str(), 16).ok()?;
                        return Some((format!("{{{}}}", digits), cp));
                    }
                    _ => digits.push(iter.next()?),
                }
            }
        }
    }

    None
}
fn add_control_character_to_vec(
    prefix: &str,
    iter: &mut Peekable<Chars>,
    control_characters: &mut Vec<String>,
    get_code_point: fn(&mut Peekable<Chars>) -> Option<(String, i64)>,
) {
    if let Some((s, cp)) = get_code_point(iter) {
        if (0..32).contains(&cp) {
            control_characters.push(format!("{}{}", prefix, s));
        }
    }
}
/// Collecting control character for regex, the following characters in regular expression patterns are considered as control characters:
/// Hexadecimal character escapes from \x00 to \x1F.
/// Unicode character escapes from \u0000 to \u001F.
/// Unicode code point escapes range from \u{0} to \u{1F}. The Unicode flag must be set as true in order for these Unicode code point escapes to work: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp/unicode.
/// Unescaped raw characters from U+0000 to U+001F.
fn collect_control_characters(pattern: String, flags: Option<String>) -> Option<Vec<String>> {
    let mut control_characters: Vec<String> = Vec::new();
    let is_unicode = flags.unwrap_or_default().contains('u');

    let mut iter = pattern.chars().peekable();

    while let Some(c) = iter.next() {
        if c == '\\' {
            if let Some(&c) = iter.peek() {
                match c {
                    'x' => {
                        iter.next();
                        add_control_character_to_vec(
                            "\\x",
                            &mut iter,
                            &mut control_characters,
                            get_code_point_from_hex_character,
                        );
                    }
                    'u' => {
                        iter.next();
                        if is_unicode {
                            add_control_character_to_vec(
                                "\\u",
                                &mut iter,
                                &mut control_characters,
                                get_code_point_from_code_point_character,
                            );
                        } else {
                            add_control_character_to_vec(
                                "\\u",
                                &mut iter,
                                &mut control_characters,
                                get_code_point_from_escape_character,
                            );
                        }
                    }
                    '\\' => {
                        iter.next();
                    }
                    _ => {}
                }
            }
        }
    }
    if !control_characters.is_empty() {
        Some(control_characters)
    } else {
        None
    }
}
fn collect_control_characters_from_expression(
    callee: &AnyJsExpression,
    js_call_arguments: &JsCallArguments,
) -> Option<Vec<String>> {
    if let AnyJsExpression::JsIdentifierExpression(js_identifier) = callee {
        if js_identifier.name().ok()?.has_name("RegExp") {
            let mut args = js_call_arguments.args().iter();
            let raw_pattern = args
                .next()
                .and_then(|arg| arg.ok())
                .and_then(|arg| JsStringLiteralExpression::cast_ref(arg.syntax()))
                .and_then(|js_string_literal| js_string_literal.inner_string_text().ok())?
                .to_string();

            let pattern = escape_string(&raw_pattern).map_or(raw_pattern, |p| p);

            let flags = args
                .next()
                .and_then(|arg| arg.ok())
                .and_then(|arg| JsStringLiteralExpression::cast_ref(arg.syntax()))
                .map(|js_string_literal| js_string_literal.text());

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
                collect_control_characters_from_expression(
                    &js_new_expression.callee().ok()?,
                    &js_new_expression.arguments()?,
                )
            }
            PossibleRegexExpression::JsCallExpression(js_call_expression) => {
                collect_control_characters_from_expression(
                    &js_call_expression.callee().ok()?,
                    &js_call_expression.arguments().ok()?,
                )
            }
            PossibleRegexExpression::JsRegexLiteralExpression(js_regex_literal_expression) => {
                collect_control_characters(
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
