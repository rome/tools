use std::cmp::Ordering;

use rome_analyze::{ActionCategory, Rule, RuleCategory, RuleDiagnostic, declare_rule, context::RuleContext};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{
    JsAnyImportClause::{
        self, JsImportBareClause, JsImportDefaultClause, JsImportNamedClause,
        JsImportNamespaceClause,
    },
    JsAnyNamedImport, JsAnyNamedImportSpecifier, JsAnyRoot, JsAnyStatement, JsExport,
    JsForStatement, JsForStatementFields, JsImport, JsImportFields, JsNamedImportSpecifier,
    JsNamedImportSpecifiers, JsSyntaxToken, TextSize, TriviaPieceKind, T,
};
use rome_rowan::{
    declare_node_union, AstNode, AstNodeExt, AstSeparatedList, SyntaxElement, SyntaxError,
    TriviaPiece,
};

use crate::{utils::natural_compare, JsRuleAction};

declare_rule! {
    /// Disallow multiple variable declarations in the same variable statement
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// let foo, bar;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// for (let i = 0, x = 1; i < arr.length; i++) {}
    /// ```
    pub(crate) UseSortedSpecifiers = "useSortedSpecifiers"
}

pub(crate) enum UseSortedSpecifiers {}

impl Rule for UseSortedSpecifiers {
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = JsAnyEImport;
    type Signals = Option<Self::State>;
    type State = Vec<JsAnyNamedImportSpecifier>;

    fn run(n: &RuleContext<Self>) -> Option<Self::State> {
        match n {
            JsAnyEImport::JsImport(import) => {
                let import_clause = import.import_clause().ok()?;
                match import_clause {
                    JsImportBareClause(_)
                    | JsImportDefaultClause(_)
                    | JsImportNamespaceClause(_) => return None,
                    JsImportNamedClause(import_named) => {
                        let named_import = import_named.named_import().ok()?;
                        match named_import {
                            JsAnyNamedImport::JsNamedImportSpecifiers(named_import_specifiers) => {
                                let specifiers = named_import_specifiers.specifiers();
                                // specifiers.separators();
                                // I use `vector collect` instead of `iter().is_sorted_by` because `iter().is_sorted_by` is not stable now
                                // reference https://github.com/rust-lang/rust/issues/53485
                                let specifier_vec = specifiers.iter().collect::<Vec<_>>();
                                // println!("{:#?}", specifier_vec);
                                let (sorted_specifier_vec, has_error) =
                                    sort_specifier_vec(specifier_vec);
                                // println!("{:#?}", sorted_specifier_vec);
                                if has_error {
                                    return None;
                                }
                                let has_diff = specifiers
                                    .iter()
                                    .zip(sorted_specifier_vec.iter())
                                    .any(|(a, b)| {
                                        // SAFETY: if any specifier has `Err` would early return because we use `has_error` checked above
                                        let a = a.unwrap();
                                        let b = b.clone().unwrap();
                                        a.to_string() != b.to_string()
                                    });
                                if has_diff {
                                    Some(
                                        sorted_specifier_vec
                                            .into_iter()
                                            .filter_map(|item| item.ok())
                                            .map(|item| {
                                                // drop the leading whitespace trivia and trailing whitespace trivia
                                                let syntax = item.into_syntax();
                                                let next_syntax = if let Some(last) =
                                                    syntax.last_token()
                                                {
                                                    // let trimmed_first = first.text_trimmed();
                                                    let mut last_none_whitespace_index = -1;
                                                    // let mut last_none_whitespace_len = 0;
                                                    let mut token_text =
                                                        last.text_trimmed().to_string();
                                                    let mut trailing = vec![];
                                                    let mut trivia_string = String::new();
                                                    for (index, leading_trivia) in
                                                        last.trailing_trivia().pieces().enumerate()
                                                    {
                                                        if !matches!(
                                                            leading_trivia.kind(),
                                                            TriviaPieceKind::Whitespace
                                                        ) {
                                                            last_none_whitespace_index = index as i32;
                                                            // break;
                                                        } else {
                                                            // token_text += leading_trivia.text();
                                                        }
                                                        trailing.push(TriviaPiece::new(
                                                            leading_trivia.kind(),
                                                            leading_trivia.text_len(),
                                                        ));
                                                        trivia_string += leading_trivia.text();
                                                    }
                                                    let none_whitespace_trivia_len = last
                                                        .trailing_trivia()
                                                        .pieces()
                                                        .take((last_none_whitespace_index + 1) as usize)
                                                        .fold(0usize, |acc, item| {
                                                            let len: usize = item.text_len().into();
                                                            acc + len
                                                        });
                                                    trailing
                                                        .truncate(none_whitespace_trivia_len + 1);
                                                    token_text += &trivia_string
                                                        [0..none_whitespace_trivia_len];
                                                    let next_last = JsSyntaxToken::new_detached(
                                                        last.kind(),
                                                        &token_text,
                                                        [],
                                                        trailing,
                                                    );
                                                    syntax
                                                        .replace_child(
                                                            SyntaxElement::Token(last),
                                                            SyntaxElement::Token(next_last),
                                                        )
                                                        .unwrap()
                                                    // first.with_leading_trivia(leading.into_iter());
                                                } else {
                                                    syntax
                                                };
                                                JsAnyNamedImportSpecifier::unwrap_cast(next_syntax)
                                                // item
                                            })
                                            .collect(),
                                    )
                                } else {
                                    None
                                }
                            }
                            JsAnyNamedImport::JsNamespaceImportSpecifier(_) => return None,
                        }
                    }
                }
            }
            JsAnyEImport::JsExport(export) => return None,
        }
    }

    fn diagnostic(node: &Self::Query, _: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::warning(
            node.range(),
            "The specifiers of the import declaration should be sorted alphabetically.",
        ))
    }

    fn action(root: JsAnyRoot, node: &Self::Query, state: &Self::State) -> Option<JsRuleAction> {
        let root = match node {
            JsAnyEImport::JsImport(import) => {
                let import_clause = import.import_clause().ok()?;
                match import_clause {
                    JsImportBareClause(_)
                    | JsImportDefaultClause(_)
                    | JsImportNamespaceClause(_) => None,
                    JsImportNamedClause(import_named) => {
                        // println!("{:#?}", state);
                        let named_import = import_named.named_import().ok()?;
                        match named_import {
                            JsAnyNamedImport::JsNamedImportSpecifiers(named_import_specifiers) => {
                                let specifiers = named_import_specifiers.specifiers();
                                // println!("{:?}", named_import_specifiers.r_curly_token());
                                let l_curly_token = named_import_specifiers.l_curly_token();
                                let r_curly_token = named_import_specifiers.r_curly_token();
                                let mut separators = specifiers.separators();
                                let root =
                                    root.replace_node(
                                        named_import_specifiers,
                                        make::js_named_import_specifiers(
                                            l_curly_token.ok()?,
                                            make::js_named_import_specifier_list(state.iter().map(
                                                |item| (item.clone(), separators.next().and_then(|item| item.ok())),
                                            )),
                                            r_curly_token.ok()?,
                                        ),
                                    );
                                println!("root: {}", root.clone()?);
                                root
                            }
                            JsAnyNamedImport::JsNamespaceImportSpecifier(_) => None,
                        }
                    }
                }
            }
            JsAnyEImport::JsExport(_) => None,
        }?;
        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Use a while loop" }.to_owned(),
            root,
        })
    }
}

fn sort_specifier_vec(
    mut specifier_vec: Vec<Result<JsAnyNamedImportSpecifier, SyntaxError>>,
) -> (Vec<Result<JsAnyNamedImportSpecifier, SyntaxError>>, bool) {
    let mut has_error = false;
    specifier_vec.sort_by(|a, b| {
        if a.is_err() || b.is_err() {
            has_error = true;
            return Ordering::Equal;
        }
        // SAFETY: We have been check if a, b specifier is error above.
        let a = a.clone().unwrap();
        let b = b.clone().unwrap();
        let a_local = match &a {
            rome_js_syntax::JsAnyNamedImportSpecifier::JsNamedImportSpecifier(specifier) => {
                match specifier.local_name() {
                    Ok(name) => name.to_string(),
                    Err(_) => {
                        has_error = true;
                        String::new()
                    }
                }
            }
            rome_js_syntax::JsAnyNamedImportSpecifier::JsShorthandNamedImportSpecifier(
                specifier,
            ) => specifier.to_string(),
            rome_js_syntax::JsAnyNamedImportSpecifier::JsUnknownNamedImportSpecifier(_) => {
                has_error = true;
                String::new()
            }
        };
        let b_local = match &b {
            rome_js_syntax::JsAnyNamedImportSpecifier::JsNamedImportSpecifier(specifier) => {
                match specifier.local_name() {
                    Ok(name) => name.to_string(),
                    Err(_) => {
                        has_error = true;
                        String::new()
                    }
                }
            }
            rome_js_syntax::JsAnyNamedImportSpecifier::JsShorthandNamedImportSpecifier(
                specifier,
            ) => specifier.to_string(),
            rome_js_syntax::JsAnyNamedImportSpecifier::JsUnknownNamedImportSpecifier(_) => {
                has_error = true;
                String::new()
            }
        };
        let local_natural_order = natural_compare(&a_local, &b_local, false);
        if local_natural_order == 0 {
            let a_exported = match &a {
                rome_js_syntax::JsAnyNamedImportSpecifier::JsNamedImportSpecifier(specifier) => {
                    match specifier.name() {
                        Ok(name) => name.to_string(),
                        Err(_) => {
                            has_error = true;
                            String::new()
                        }
                    }
                }
                rome_js_syntax::JsAnyNamedImportSpecifier::JsShorthandNamedImportSpecifier(
                    specifier,
                ) => specifier.to_string(),
                rome_js_syntax::JsAnyNamedImportSpecifier::JsUnknownNamedImportSpecifier(_) => {
                    has_error = true;
                    String::new()
                }
            };
            let b_exported = match b {
                rome_js_syntax::JsAnyNamedImportSpecifier::JsNamedImportSpecifier(specifier) => {
                    match specifier.name() {
                        Ok(name) => name.to_string(),
                        Err(_) => {
                            has_error = true;
                            String::new()
                        }
                    }
                }
                rome_js_syntax::JsAnyNamedImportSpecifier::JsShorthandNamedImportSpecifier(
                    specifier,
                ) => specifier.to_string(),
                rome_js_syntax::JsAnyNamedImportSpecifier::JsUnknownNamedImportSpecifier(_) => {
                    has_error = true;
                    String::new()
                }
            };
            let exported_natural_order = natural_compare(&a_exported, &b_exported, false);
            if exported_natural_order == 0 {
                return Ordering::Equal;
            } else if exported_natural_order < 0 {
                return Ordering::Less;
            } else {
                return Ordering::Greater;
            }
        } else if local_natural_order < 0 {
            return Ordering::Less;
        } else {
            return Ordering::Greater;
        }
    });
    (specifier_vec, has_error)
}

declare_node_union! {
    /// Enum for [JsImport] and [JsExport]
    #[allow(dead_code)]
    pub(crate) JsAnyEImport  = JsImport | JsExport
}
