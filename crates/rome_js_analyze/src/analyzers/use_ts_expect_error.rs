use rome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, Rule, RuleCategory, RuleDiagnostic,
};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_syntax::{JsAnyStatement, JsSyntaxToken};
use rome_rowan::{AstNode, AstNodeExt, TriviaPiece};

use crate::JsRuleAction;

declare_rule! {
    /// Enforce use `@ts-expect-error` instead of `@ts-ignore` to get notified when suppression is no longer necessary.
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

impl Rule for UseTsExpectError {
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = Ast<JsAnyStatement>;
    /// The `@ts-ignore` index of trailing trivia, and the corresponding replaced_string using `@ts-expect-error`
    type State = Vec<(usize, String)>;
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

                        if let Some(stripped) = finalized.strip_prefix("@ts-ignore") {
                            ts_ignore_index_vec.push((
                                i,
                                format!(
                                    "{}@ts-expect-error{}",
                                    &original[0..original.len() - finalized.len()],
                                    stripped
                                ),
                            ));
                        }
                    }
                    rome_js_syntax::TriviaPieceKind::MultiLineComment => {
                        let original = item.text();
                        //
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
                                    multiline_ts_ignore_index_vec.push(offset);
                                }
                                offset += finalized.len();
                            });
                        assert_eq!(offset, original.len());

                        if !multiline_ts_ignore_index_vec.is_empty() {
                            let len = multiline_ts_ignore_index_vec.len();
                            let mut replaced_string =
                                String::from(&original[0..multiline_ts_ignore_index_vec[0]]);
                            // Handle if multiline comment has multiple `@ts-ignore`
                            for window in multiline_ts_ignore_index_vec.windows(2) {
                                let first_index = window[0];
                                let second_index = window[1];
                                replaced_string.push_str("@ts-expect-error");
                                replaced_string.push_str(&original[first_index + 10..second_index]);
                            }
                            // Here we know that `multiline_ts-ignore_index_vec` have length greater equal to 1
                            // we need to concat the rest of length after the last `@ts-ignore`
                            replaced_string.push_str("@ts-expect-error");
                            replaced_string
                                .push_str(&original[multiline_ts_ignore_index_vec[len - 1] + 10..]);
                            ts_ignore_index_vec.push((i, replaced_string))
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
        let mut next_leading_trivia = vec![];
        let mut next_trailing_trivia = vec![];
        let mut next_leading_trivia_string = String::new();
        let mut next_trailing_trivia_string = String::new();
        first_token
            .leading_trivia()
            .pieces()
            .enumerate()
            .for_each(|(i, item)| {
                if ignore_cursor < state.len() && state[ignore_cursor].0 == i {
                    next_leading_trivia.push(TriviaPiece::new(
                        item.kind(),
                        state[ignore_cursor].1.len() as u32,
                    ));
                    next_leading_trivia_string.push_str(&state[ignore_cursor].1);
                    ignore_cursor += 1;
                } else {
                    next_leading_trivia
                        .push(TriviaPiece::new(item.kind(), item.text().len() as u32));
                    next_leading_trivia_string.push_str(item.text());
                }
            });
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
