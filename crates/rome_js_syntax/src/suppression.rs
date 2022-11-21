use rome_diagnostics_categories::Category;

/// Single instance of a suppression comment, with the following syntax:
///
/// `// rome-ignore { <category> { (<value>) }? }+: <reason>`
///
/// The category broadly describes what feature is being suppressed (formatting,
/// linting, ...) with the value being and optional, category-specific name of
/// a specific element to disable (for instance a specific lint name). A single
/// suppression may specify one or more categories + values, for instance to
/// disable multiple lints at once
///
/// A suppression must specify a reason: this part has no semantic meaning but
/// is required to document why a particular feature is being disable for this
/// line (lint false-positive, specific formatting requirements, ...)
#[derive(Debug, PartialEq, Eq)]
pub struct Suppression<'a> {
    /// List of categories for this suppression
    ///
    /// Categories are pair of the category name +
    /// an optional category value
    pub categories: Vec<(&'a Category, Option<&'a str>)>,
    /// Reason for this suppression comment to exist
    pub reason: &'a str,
}

pub fn parse_suppression_comment(comment: &str) -> impl Iterator<Item = Suppression> {
    let (head, mut comment) = comment.split_at(2);
    let is_block_comment = match head {
        "//" => false,
        "/*" => {
            comment = comment
                .strip_suffix("*/")
                .or_else(|| comment.strip_suffix(&['*', '/']))
                .unwrap_or(comment);
            true
        }
        token => panic!("comment with unknown opening token {token:?}, from {comment}"),
    };

    comment.lines().filter_map(move |line| {
        // Eat start of line whitespace
        let mut line = line.trim_start();

        // If we're in a block comment eat stars, then whitespace again
        if is_block_comment {
            line = line.trim_start_matches('*').trim_start()
        }

        // Check for the rome-ignore token or skip the line entirely
        line = line.strip_prefix("rome-ignore")?.trim_start();

        let mut categories = Vec::new();

        loop {
            // Find either a colon opening parenthesis or space
            let separator = line.find(|c: char| c == ':' || c == '(' || c.is_whitespace())?;

            let (category, rest) = line.split_at(separator);
            let category = category.trim_end();
            let category: Option<&'static Category> = if !category.is_empty() {
                Some(category.parse().ok()?)
            } else {
                None
            };

            // Skip over and match the separator
            let (separator, rest) = rest.split_at(1);

            match separator {
                // Colon token: stop parsing categories
                ":" => {
                    if let Some(category) = category {
                        categories.push((category, None));
                    }

                    line = rest.trim_start();
                    break;
                }
                // Paren token: parse a category + value
                "(" => {
                    let category = category?;
                    let paren = rest.find(')')?;

                    let (value, rest) = rest.split_at(paren);
                    let value = value.trim();

                    categories.push((category, Some(value)));

                    line = rest.strip_prefix(')').unwrap().trim_start();
                }
                // Whitespace: push a category without value
                _ => {
                    if let Some(category) = category {
                        categories.push((category, None));
                    }

                    line = rest.trim_start();
                }
            }
        }

        let reason = line.trim_end();
        Some(Suppression { categories, reason })
    })
}

#[cfg(test)]
mod tests {
    use rome_diagnostics_categories::category;

    use super::{parse_suppression_comment, Suppression};

    #[test]
    fn parse_simple_suppression() {
        assert_eq!(
            parse_suppression_comment("// rome-ignore parse: explanation1").collect::<Vec<_>>(),
            vec![Suppression {
                categories: vec![(category!("parse"), None)],
                reason: "explanation1"
            }],
        );

        assert_eq!(
            parse_suppression_comment("/** rome-ignore parse: explanation2 */").collect::<Vec<_>>(),
            vec![Suppression {
                categories: vec![(category!("parse"), None)],
                reason: "explanation2"
            }],
        );

        assert_eq!(
            parse_suppression_comment(
                "/**
                  * rome-ignore parse: explanation3
                  */"
            )
            .collect::<Vec<_>>(),
            vec![Suppression {
                categories: vec![(category!("parse"), None)],
                reason: "explanation3"
            }],
        );

        assert_eq!(
            parse_suppression_comment(
                "/**
                  * hello
                  * rome-ignore parse: explanation4
                  */"
            )
            .collect::<Vec<_>>(),
            vec![Suppression {
                categories: vec![(category!("parse"), None)],
                reason: "explanation4"
            }],
        );
    }
    #[test]
    fn parse_unclosed_block_comment_suppressions() {
        assert_eq!(
            parse_suppression_comment("/* rome-ignore format: explanation").collect::<Vec<_>>(),
            vec![Suppression {
                categories: vec![(category!("format"), None)],
                reason: "explanation"
            }],
        );

        assert_eq!(
            parse_suppression_comment("/* rome-ignore format: explanation *").collect::<Vec<_>>(),
            vec![Suppression {
                categories: vec![(category!("format"), None)],
                reason: "explanation"
            }],
        );

        assert_eq!(
            parse_suppression_comment("/* rome-ignore format: explanation /").collect::<Vec<_>>(),
            vec![Suppression {
                categories: vec![(category!("format"), None)],
                reason: "explanation"
            }],
        );
    }

    #[test]
    fn parse_multiple_suppression() {
        assert_eq!(
            parse_suppression_comment("// rome-ignore parse(foo) parse(dog): explanation")
                .collect::<Vec<_>>(),
            vec![Suppression {
                categories: vec![
                    (category!("parse"), Some("foo")),
                    (category!("parse"), Some("dog"))
                ],
                reason: "explanation"
            }],
        );

        assert_eq!(
            parse_suppression_comment("/** rome-ignore parse(bar) parse(cat): explanation */")
                .collect::<Vec<_>>(),
            vec![Suppression {
                categories: vec![
                    (category!("parse"), Some("bar")),
                    (category!("parse"), Some("cat"))
                ],
                reason: "explanation"
            }],
        );

        assert_eq!(
            parse_suppression_comment(
                "/**
                  * rome-ignore parse(yes) parse(frog): explanation
                  */"
            )
            .collect::<Vec<_>>(),
            vec![Suppression {
                categories: vec![
                    (category!("parse"), Some("yes")),
                    (category!("parse"), Some("frog"))
                ],
                reason: "explanation"
            }],
        );

        assert_eq!(
            parse_suppression_comment(
                "/**
                  * hello
                  * rome-ignore parse(wow) parse(fish): explanation
                  */"
            )
            .collect::<Vec<_>>(),
            vec![Suppression {
                categories: vec![
                    (category!("parse"), Some("wow")),
                    (category!("parse"), Some("fish"))
                ],
                reason: "explanation"
            }],
        );
    }

    #[test]
    fn parse_multiple_suppression_categories() {
        assert_eq!(
            parse_suppression_comment("// rome-ignore format lint: explanation")
                .collect::<Vec<_>>(),
            vec![Suppression {
                categories: vec![(category!("format"), None), (category!("lint"), None)],
                reason: "explanation"
            }],
        );
    }
}
