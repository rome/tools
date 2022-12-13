use std::{iter::Peekable, str::Chars};

use rome_analyze::{context::RuleContext, declare_rule, Ast, Rule};
use rome_js_syntax::{JsNewExpression, JsRegexLiteralExpression};
use rome_rowan::{declare_node_union, AstNode};

use crate::utils::escape;

declare_rule! {
 /// Promotes the use of awesome tricks
 ///
 /// ## Examples
 ///
 /// ### Invalid
 ///
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
        } else {
            for _ in 1..=4 {
                digits.push(it.next()?);
            }

            let s: String = digits.into_iter().collect();
            let cp = i64::from_str_radix(s.as_str(), 16).ok()?;
            return Some((s, cp));
        }
    }

    None
}

fn collect_control_characters(pattern: String) -> Option<Vec<String>> {
    let mut control_characters: Vec<String> = Vec::new();

    let mut it = pattern.chars().peekable();

    while let Some(&c) = it.peek() {
        match c {
            '\\' => {
                it.next();
                if let Some(&c) = it.peek() {
                    match c {
                        '\\' => {
                            it.next();
                            if let Some(&c) = it.peek() {
                                match c {
                                    'x' => {
                                        it.next();
                                        let (s, cp) = get_code_point_from_hex_character(&mut it)?;
                                        if cp >= 0 && cp < 32 {
                                            control_characters.push(format!("\\\\x{}", s));
                                        }
                                    }
                                    'u' => {
                                        it.next();
                                        let (s, cp) =
                                            get_code_point_from_escape_character(&mut it)?;
                                        if cp >= 0 && cp < 32 {
                                            control_characters.push(format!("\\\\u{}", s));
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                        'x' => {
                            it.next();
                            let (s, cp) = get_code_point_from_hex_character(&mut it)?;
                            if cp >= 0 && cp < 32 {
                                control_characters.push(format!("\\x{}", s));
                            }
                        }
                        'u' => {
                            it.next();
                            let (s, cp) = get_code_point_from_escape_character(&mut it)?;
                            if cp >= 0 && cp < 32 {
                                control_characters.push(format!("\\u{}", s));
                            }
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
    Some(control_characters)
}

impl Rule for NoControlCharactersInRegex {
    type Query = Ast<PossibleRegexExpression>;
    type State = String;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let ele = JsRegexLiteralExpression::cast(node.syntax().clone()).unwrap();
        let pattern = ele.pattern().unwrap();
        println!("patten : {}", pattern);
        let controls = collect_control_characters(pattern);
        if let Some(ctrls) = controls {
            for c in ctrls {
                println!("{}", c);
            }
        }

        None
    }
}
