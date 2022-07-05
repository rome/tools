use std::ops::Range;

use rome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, Rule, RuleCategory, RuleDiagnostic,
};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_syntax::{JsAnyStatement, JsSyntaxToken};
use rome_rowan::{AstNode, AstNodeExt, TriviaPiece};

use crate::JsRuleAction;

declare_rule! {
    /// Enforces the use of `@ts-expect-error` instead of `@ts-ignore` to get notified when a suppression is no longer necessary.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// // @ts-ignore
    /// let foo: boolean = 1;
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// // @ts-ignore: Blah blah blah
    /// let foo: boolean = 1;
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// /* @ts-ignore */
    /// let foo: boolean = 1;
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// /** @ts-ignore */
    /// let foo: boolean = 1;
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// /**
    ///  ** @ts-ignore */
    /// let foo: boolean = 1;
    /// ```
    /// ### Valid
    ///
    /// ```ts
    /// // @ts-expect-error
    /// let foo: boolean = 1;
    /// // @ts-expect-error: Blah blah blah
    /// let foo: boolean = 1;
    /// /* @ts-expect-error */
    /// let foo: boolean = 1;
    /// /** @ts-expect-error */
    /// let foo: boolean = 1;
    /// /**
    /// * @ts-expect-error */
    /// let foo: boolean = 1;
    /// /**
    /// ** @ts-expect-error */
    /// let foo: boolean = 1;
    /// ```
    pub(crate) UseTsExpectError = "useTsExpectError"
}

const TS_EXPECT_ERROR_SUPPRESSION: &str = "@ts-expect-error";

impl Rule for UseTsExpectError {
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = Ast<JsAnyStatement>;
    /// 
    type State = Vec<(usize, Vec<Range<usize>>)>;
    type Signals = Option<Self::State>;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let Ast(n) = ctx.query();
        n.syntax().first_token().and_then(|token| {
            let mut ts_ignore_index_vec = vec![];
            token
                .leading_trivia()
                .pieces()
                .enumerate()
                .for_each(|(i, item)| match item.kind() {
                    rome_js_syntax::TriviaPieceKind::SingleLineComment => {
                        let original = item.text();
                        // I use this `strip_prefix` just because of clippy not happy
                        let finalized: &str =
                            if let Some(stripped) = original.strip_prefix("/**") {
                                stripped
                            } else if original.starts_with("//") || original.starts_with("/*") {
                                &original[2..]
                            } else {
                                original
                            }
                            .trim_start();
                        let offset = original.len() - finalized.len();

                        if finalized.starts_with("@ts-ignore") {
                            ts_ignore_index_vec.push((
                                i,
                                // 10 is the length of `@ts-ignore`
                                vec![offset..offset + 10],
                            ));
                        }
                    }
                    rome_js_syntax::TriviaPieceKind::MultiLineComment => {
                        let original = item.text();
                        let mut multiline_ts_ignore_index_vec = vec![];
                        let mut offset = 2;
                        original
                            .trim_start_matches("/*")
                            .split('\n')
                            .enumerate()
                            .for_each(|(i, line)| {
                                //Add the new line offset
                                offset += if i == 0 { 0 } else { 1 };
                                let finalized =
                                    line.trim_start().trim_start_matches('*').trim_start();
                                offset += line.len() - finalized.len();
                                if finalized.starts_with("@ts-ignore") {
                                    multiline_ts_ignore_index_vec.push(offset..offset + 10);
                                }
                                offset += finalized.len();
                            });
                        assert_eq!(offset, original.len());

                        if !multiline_ts_ignore_index_vec.is_empty() {
                            ts_ignore_index_vec.push((i, multiline_ts_ignore_index_vec));
                        }
                    }
                    _ => {}
                });
            if !ts_ignore_index_vec.is_empty() {
                Some(ts_ignore_index_vec)
            } else {
                None
            }
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let Ast(node) = ctx.query();

        Some(RuleDiagnostic::warning(
            node.range(),
            markup! {
                "Prefer @ts-expect-error to get notified when suppression is no longer necessary."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let Ast(node) = ctx.query();
        let root = ctx.root();
        let mut ignore_cursor = 0;
        let first_token = node.syntax().first_token()?;
        // Clone trailing trivia and replace leading trivia
        let mut next_leading_trivia = vec![];
        let mut next_trailing_trivia = vec![];
        let mut next_leading_trivia_string = String::new();
        let mut next_trailing_trivia_string = String::new();
        first_token
            .leading_trivia()
            .pieces()
            .enumerate()
            .for_each(|(i, item)| {
                let text = item.text();
                if ignore_cursor < state.len() && state[ignore_cursor].0 == i {
                    let ts_ignore_range_list = &state[ignore_cursor].1;
                    let mut normalized_string = String::with_capacity(text.len());
                    // Slicing string from start of the comment to the first start of `@ts-ignore`
                    normalized_string.push_str(&text[0..ts_ignore_range_list[0].start]);
                    let mut previous_end = ts_ignore_range_list[0].start;
                    // Copy string between two `@ts-ignore` and replace it with `@ts-expect-error`
                    for &Range { start, end } in ts_ignore_range_list.iter() {
                        normalized_string.push_str(&text[previous_end..start]);
                        normalized_string.push_str(TS_EXPECT_ERROR_SUPPRESSION);
                        previous_end = end;
                    }
                    // Copy the rest of the string after the last `@ts-ignore`
                    normalized_string.push_str(&text[previous_end..]);
                    next_leading_trivia.push(TriviaPiece::new(
                        item.kind(),
                        normalized_string.len() as u32,
                    ));
                    next_leading_trivia_string.push_str(&normalized_string);
                    ignore_cursor += 1;
                } else {
                    next_leading_trivia.push(TriviaPiece::new(item.kind(), text.len() as u32));
                    next_leading_trivia_string.push_str(text);
                }
            });
        // Copy trailing trivia
        first_token.trailing_trivia().pieces().for_each(|item| {
            next_trailing_trivia.push(TriviaPiece::new(item.kind(), item.text().len() as u32));
            next_trailing_trivia_string.push_str(item.text());
        });
        let next_first_token = JsSyntaxToken::new_detached(
            first_token.kind(),
            &format!(
                "{}{}{}",
                next_leading_trivia_string,
                first_token.text_trimmed(),
                next_trailing_trivia_string
            ),
            next_leading_trivia,
            next_trailing_trivia,
        );
        let root = root.replace_token_discard_trivia(first_token, next_first_token)?;
        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Use `@ts-expect-error` instead of `@ts-ignore`." }.to_owned(),
            root,
        })
    }
}
