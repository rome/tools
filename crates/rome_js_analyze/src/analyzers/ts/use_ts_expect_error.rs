use rome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, Rule, RuleCategory, RuleDiagnostic,
};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_syntax::{JsAnyStatement, JsSyntaxToken, TriviaPieceKind};
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
    pub(crate) UseTsExpectError  {
        version: "0.8.0",
        name: "useTsExpectError"
    }
}

const TS_EXPECT_ERROR_SUPPRESSION: &str = "@ts-expect-error";

impl Rule for UseTsExpectError {
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = Ast<JsAnyStatement>;
    type State = (usize, usize);
    type Signals = Option<Self::State>;

    /// Only two kinds of suppressions are supported:
    /// 1. For single line comment, the comment must start with `@ts-ignore` after trim_start.
    /// ```ts
    /// //                           @ts-ignore
    /// let a: string  = 3;
    /// ```
    /// 2. For multiline comment, the last line of the multiline comment must start with `@ts-ignore` after trim_start.
    /// ```ts
    /// /*
    ///
    ///  @ts-ignore                     */
    /// let a: string = 3;
    /// ```
    /// Even the suppression below is valid
    /// ```ts
    /// /*
    ///
    ///  @ts-ignoresomething                    */
    /// let a: string = 3;
    /// ```
    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let any_stmt = ctx.query();
        any_stmt.syntax().first_token().and_then(|token| {
            let mut ts_ignore_state = None;
            // Finding the last comment trivia of first token's leading trivia.
            match token
                .leading_trivia()
                .pieces()
                .enumerate()
                .rfind(|(_, piece)| {
                    matches!(
                        piece.kind(),
                        TriviaPieceKind::SingleLineComment | TriviaPieceKind::MultiLineComment
                    )
                }) {
                Some((i, piece)) => match piece.kind() {
                    TriviaPieceKind::SingleLineComment => {
                        let original_piece_text = piece.text();
                        let trimmed_text: &str =
                            if let Some(stripped) = original_piece_text.strip_prefix("//") {
                                stripped.trim_start()
                            } else if let Some(stripped) = original_piece_text.strip_prefix("/*") {
                                stripped.trim_start_matches('*').trim_start()
                            } else {
                                original_piece_text
                            };
                        if trimmed_text.starts_with("@ts-ignore") {
                            ts_ignore_state =
                                Some((i, original_piece_text.len() - trimmed_text.len()));
                        }
                    }
                    TriviaPieceKind::MultiLineComment => {
                        let original = piece.text();

                        let mut offset = 0;
                        let split_iterator = original.split('\n');
                        let line_count = split_iterator.clone().count();
                        split_iterator.enumerate().for_each(|(index, line)| {
                            // We use `\n` as our splitter,
                            // so we need to add a leading newline offset (1) when i greater than 0.
                            offset += if index == 0 { 0 } else { 1 };
                            if index == line_count - 1 {
                                // 1. multi line multiline comment with leading star
                                // *           @ts-ignore                */
                                //^^^^^^^^^^^^^
                                // 2. single line multiline comment
                                // /** @ts-ignore*/
                                // ^^^^
                                // Merge all these cases into one.
                                let normalized_text = line
                                    .trim_start_matches("/*")
                                    .trim_start_matches('*')
                                    .trim_start()
                                    .trim_start_matches('*')
                                    .trim_start();
                                offset += line.len() - normalized_text.len();
                                if normalized_text.starts_with("@ts-ignore") {
                                    ts_ignore_state = Some((i, offset));
                                }
                            } else {
                                offset += line.len();
                            }
                        });
                    }
                    _ => {}
                },
                None => {}
            }
            ts_ignore_state
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(RuleDiagnostic::warning(
            node.range(),
            markup! {
                "Prefer @ts-expect-error to get notified when suppression is no longer necessary."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let root = ctx.root();
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
            .for_each(|(i, piece)| {
                let text = piece.text();
                if state.0 == i {
                    let mut normalized_string = String::with_capacity(text.len());
                    // Replace `@ts-ignore` to `@ts-expect-error`
                    normalized_string.push_str(&text[0..state.1]);
                    normalized_string.push_str(TS_EXPECT_ERROR_SUPPRESSION);
                    normalized_string.push_str(&text[(state.1 + 10)..]);
                    next_leading_trivia.push(TriviaPiece::new(
                        piece.kind(),
                        normalized_string.len() as u32,
                    ));
                    next_leading_trivia_string.push_str(&normalized_string);
                } else {
                    next_leading_trivia.push(TriviaPiece::new(piece.kind(), text.len() as u32));
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
