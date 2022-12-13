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
fn get_code_point_from_hex_character(it: &mut Peekable<Chars>) -> Option<i64> {
    let mut digits = Vec::new();
    digits.push(it.next()?);
    digits.push(it.next()?);
    let s: String = digits.into_iter().collect();
    i64::from_str_radix(s.as_str(), 16).ok()
}

fn get_code_point_from_escape_character(it: &mut Peekable<Chars>) -> Option<i64> {
    let mut digits = Vec::new();

    if let Some(&c) = it.peek() {
        if c == '{' {
            it.next();
            while let Some(&c) = it.peek() {
                match c {
                    '}' => {
                        it.next();
                        let s: String = digits.into_iter().collect();
                        return i64::from_str_radix(s.as_str(), 16).ok();
                    }
                    _ => digits.push(it.next()?),
                }
            }
        } else {
            for _ in 1..=4 {
                digits.push(it.next()?);
            }

            let s: String = digits.into_iter().collect();
            return i64::from_str_radix(s.as_str(), 16).ok();
        }
    }

    None
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
                                            let cp = get_code_point_from_hex_character(&mut it)?;
                                            println!("{:?}", cp)
                                        }
                                        'u' => {
                                            it.next();
                                            // get number with {}
                                            let cp = get_code_point_from_escape_character(&mut it)?;
                                            println!("{}", cp);
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            'x' => {
                                it.next();
                                let cp = get_code_point_from_hex_character(&mut it)?;
                                println!("{:?}", cp)
                            }
                            'u' => {
                                it.next();
                                // get number with {}
                                let cp = get_code_point_from_escape_character(&mut it)?;
                                println!("{}", cp);
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
        None
    }
}
