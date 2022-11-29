use crate::utils::batch::JsBatchMutation;
use rome_analyze::SuppressionCommentEmitterPayload;
use rome_js_factory::make::{jsx_expression_child, token};
use rome_js_syntax::jsx_ext::AnyJsxElement;
use rome_js_syntax::{
    AnyJsxChild, JsLanguage, JsSyntaxKind, JsSyntaxToken, JsTemplateElement, JsxChildList,
    JsxElement, JsxOpeningElement, JsxSelfClosingElement, JsxText, TextRange, T,
};
use rome_rowan::{AstNode, AstNodeExt, TokenAtOffset, TriviaPieceKind};

/// We now try to "guess" the token where to apply the suppression comment.
/// Considering that the detection of suppression comments in the linter is "line based", we start
/// querying the node covered by the text range of the diagnostic, until we find the first token that has a newline
/// among its leading trivia.
///
/// If we're not able to find any token, it means that the range is
/// placed at row 1, so we take the root itself.
pub(crate) fn apply_suppression_comment(payload: SuppressionCommentEmitterPayload<JsLanguage>) {
    let SuppressionCommentEmitterPayload {
        token_offset,
        mutation,
        suppression_text,
        diagnostic_text_range,
    } = payload;
    // retrieve the most suited, most left token where the diagnostics was emitted
    let original_token = peek_token_from_offset(token_offset, diagnostic_text_range);

    // considering that our suppression system works via lines, we need to look for the first newline,
    // so we can place the comment there
    let first_token_with_newline = original_token.as_ref().map(|original_token| {
        match find_first_token_with_newline(original_token.clone()) {
            None => (original_token.clone(), false),
            Some(token) => token,
        }
    });

    if let Some((first_token_with_newline, should_insert_leading_newline)) =
        first_token_with_newline
    {
        // we check if the token that has the newline is inside a JSX element: JsxOpeningElement or JsxSelfClosingElement
        let current_jsx_element = first_token_with_newline.parent().and_then(|parent| {
            if AnyJsxElement::can_cast(parent.kind()) || JsxText::can_cast(parent.kind()) {
                Some(parent)
            } else {
                None
            }
        });

        // When inside a JSX element, we have to apply different logics when applying suppression comments.
        // Newlines are inside JsxText.
        if let Some(current_jsx_element) = current_jsx_element {
            // quick check is the element is inside a list
            if current_jsx_element
                .parent()
                .map(|p| JsxChildList::can_cast(p.kind()))
                .unwrap_or_default()
            {
                let jsx_comment = jsx_expression_child(
                    token(T!['{']).with_trailing_trivia([(
                        TriviaPieceKind::SingleLineComment,
                        format!("/* {}: suppressed */", suppression_text).as_str(),
                    )]),
                    token(T!['}']),
                )
                .build();
                if let Some(current_element) = JsxOpeningElement::cast_ref(&current_jsx_element) {
                    let parent = current_element.parent::<JsxElement>();
                    if let Some(parent) = parent {
                        mutation.add_jsx_elements_before_element(
                            &parent.into(),
                            &[AnyJsxChild::JsxExpressionChild(jsx_comment)],
                        );
                    }
                } else if let Some(current_element) =
                    JsxSelfClosingElement::cast_ref(&current_jsx_element)
                {
                    mutation.add_jsx_elements_before_element(
                        &AnyJsxChild::JsxSelfClosingElement(current_element),
                        &[AnyJsxChild::JsxExpressionChild(jsx_comment)],
                    );
                } else if let Some(current_element) = JsxText::cast_ref(&current_jsx_element) {
                    // We want to add an additional JsxText to keep the indentation
                    mutation.add_jsx_elements_before_element(
                        &AnyJsxChild::JsxText(current_element.clone()),
                        &[
                            AnyJsxChild::JsxText(current_element.detach()),
                            AnyJsxChild::JsxExpressionChild(jsx_comment),
                        ],
                    );
                }
            } else {
                let mut new_token = first_token_with_newline.clone();
                if !should_insert_leading_newline {
                    new_token = new_token.with_leading_trivia([
                        (TriviaPieceKind::Newline, "\n"),
                        (
                            TriviaPieceKind::SingleLineComment,
                            format!("// {}: suppressed ", suppression_text).as_str(),
                        ),
                        (TriviaPieceKind::Newline, "\n"),
                    ])
                } else {
                    new_token = new_token.with_leading_trivia([
                        (
                            TriviaPieceKind::SingleLineComment,
                            format!("// {}: suppressed ", suppression_text).as_str(),
                        ),
                        (TriviaPieceKind::Newline, "\n"),
                    ])
                };
                mutation.replace_token_transfer_trivia(first_token_with_newline, new_token);
            }
        } else {
            let mut new_token = first_token_with_newline.clone();
            if !should_insert_leading_newline {
                new_token = new_token.with_leading_trivia([
                    (TriviaPieceKind::Newline, "\n"),
                    (
                        TriviaPieceKind::SingleLineComment,
                        format!("// {}: suppressed ", suppression_text).as_str(),
                    ),
                    (TriviaPieceKind::Newline, "\n"),
                ])
            } else {
                new_token = new_token.with_leading_trivia([
                    (
                        TriviaPieceKind::SingleLineComment,
                        format!("// {}: suppressed ", suppression_text).as_str(),
                    ),
                    (TriviaPieceKind::Newline, "\n"),
                ])
            };
            mutation.replace_token_transfer_trivia(first_token_with_newline, new_token);
        }
    }
}

/// It checks if the current token has leading trivia newline. If not, it
/// it peeks the previous token and recursively call itself.
///
/// Due to the nature of JSX, sometimes the current token might contain text that contains
/// some newline. In case that happens, we choose that token.
///
/// Due to the nature of JavaScript templates, we also check if the tokens we browse are
/// `${` and if so, we stop there.
fn find_first_token_with_newline(token: JsSyntaxToken) -> Option<(JsSyntaxToken, bool)> {
    let mut current_token = token;
    let mut should_insert_leading_newline = false;
    loop {
        let trivia = current_token.leading_trivia();
        if trivia.pieces().any(|trivia| trivia.is_newline())
            || current_token.text_trimmed().contains('\n')
            || current_token.text_trimmed().contains("\r\n")
        {
            break;
        } else if matches!(current_token.kind(), JsSyntaxKind::DOLLAR_CURLY)
            && current_token
                .parent()
                .map(|p| JsTemplateElement::can_cast(p.kind()))
                .unwrap_or_default()
        {
            if let Some(next_token) = current_token.next_token() {
                current_token = next_token;
                break;
            } else {
                continue;
            }
        } else if let Some(token) = current_token.prev_token() {
            current_token = token;
            continue;
        } else {
            should_insert_leading_newline = true;
            break;
        }
    }
    // If the flag has been set to `true`, it means we are at the beginning of the file.
    if !should_insert_leading_newline {
        // Still, if there's a a multiline comment, we want to try to attach the suppression comment
        // to the existing multiline comment without newlines.
        should_insert_leading_newline = current_token
            .leading_trivia()
            .pieces()
            .all(|piece| !piece.kind().is_multiline_comment());
    }
    Some((current_token, should_insert_leading_newline))
}

/// This function peeks the token from a given offset and the range of the emitted diagnostic.
///
/// Sometimes, the offset is between tokens, we need to decide which one to take.
///
/// For example:
/// ```jsx
/// function f() {
///     return <div
///     ><img /> {/* <--- diagnostic emitted in this line */}
///     </div>
/// }
/// ```
///
/// In these case it's best to peek the right token, because it belongs to the node where error actually occurred,
/// and becomes easier to add the suppression comment.
fn peek_token_from_offset(
    token_offset: TokenAtOffset<JsSyntaxToken>,
    diagnostic_text_range: &TextRange,
) -> Option<JsSyntaxToken> {
    match token_offset {
        TokenAtOffset::None => None,
        TokenAtOffset::Single(token) => Some(token),
        TokenAtOffset::Between(left_token, right_token) => {
            let chosen_token = if right_token.text_range().start() == diagnostic_text_range.start()
            {
                right_token
            } else {
                left_token
            };
            Some(chosen_token)
        }
    }
}
