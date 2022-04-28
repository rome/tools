use crate::utils::has_formatter_suppressions;
use crate::{Format, FormatElement, FormatResult, Formatter};
use rome_formatter::format_element::get_lines_before;
use rome_formatter::{concat_elements, empty_line, format_elements, hard_line_break};
use rome_js_syntax::{
    JsAnyImportClause, JsAnyModuleItem, JsImportBareClause, JsImportDefaultClause,
    JsImportNamedClause, JsImportNamespaceClause, JsModuleItemList, JsSyntaxNode,
};
use rome_rowan::AstNode;
use std::cmp::Ordering;
use std::fmt::Debug;

impl Format for JsModuleItemList {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        if rome_flags::unstable().sort_imports() {
            unstable_sort_imports(self.clone(), formatter)
        } else {
            Ok(formatter.format_list(self.clone()))
        }
    }
}

/// Small data structure to track all the [JsImport] found inside a [JsModuleItemList]
///
/// Because of the fact that the list items can appear in every order, and that potential statements
/// can be found between [JsImport], we track each [JsImport] until we find another item that is not
/// a [JsImport]. When this condition is met, we sort the [JsImport] found so far, empty the list, and then we start
/// to track them again.
///
/// Given the following example:
///
/// ```js
/// import * as fs from "node:fs";
/// import "some-polyfill";
///
/// window.loadPolyfill();
///
/// import { sort } from "lodash";
/// import "bootstrap"
///
/// ```
///
/// The first two imports will be sorted by themselves, because the third item found is not a [JsImport]
/// and we can't moving it around because it might depend on some side effect of the previous nodes.
/// The statement `import "some-polyfill"` will be put at the top.
///
///  The end result will be something like:
///
/// ```js
/// import "some-polyfill";
/// import * as fs from "node:fs";
///
/// window.loadPolyfill();
///
/// import "bootstrap"
/// import { sort } from "lodash";
/// ```
///
///
/// [JsImport]: rome_js_syntax::JsImport
/// [JsModuleItemList]: rome_js_syntax::JsModuleItemList
#[derive(Debug, Default)]
struct SortedImports {
    import_list: Vec<Import>,
    result: Vec<FormatElement>,
}

impl SortedImports {
    /// Given a reference to the type of the import, it stores it inside an intermediate list
    pub fn store_formatted_import_clause(
        &mut self,
        import_clause: JsAnyImportClause,
        formatted: FormatElement,
        trailing_lines: usize,
    ) {
        let has_suppression = has_formatter_suppressions(import_clause.syntax());
        if !has_suppression {
            if let JsAnyImportClause::JsImportBareClause(import_clause) = import_clause {
                self.import_list.push(Import::PossiblyWithSideEffects {
                    node: import_clause,
                    formatted,
                    trailing_lines,
                })
            } else {
                self.import_list.push(Import::Safe {
                    node: import_clause.into(),
                    formatted,
                    trailing_lines,
                })
            }
        } else {
            self.import_list.push(Import::Ignored {
                formatted,
                trailing_lines,
            })
        }
    }

    /// It stores any module item that is not a [JsImport]
    ///
    /// [JsImport]: rome_js_syntax::JsImport
    pub fn store_formatted_module_item(
        &mut self,
        item: JsAnyModuleItem,
        formatted: FormatElement,
        trailing_lines: usize,
    ) {
        // we don't want to deliberately store JsImport nodes because they should be treated differently
        debug_assert!(!matches!(item, JsAnyModuleItem::JsImport(_)));
        if !self.import_list.is_empty() {
            self.sort_and_store_import_list(false);
        }
        self.result.push(if trailing_lines > 1 {
            format_elements![formatted, empty_line()]
        } else {
            format_elements![formatted, hard_line_break()]
        })
    }

    /// It consumes and sort possible dangling imports, and then format the whole list
    pub fn into_format_element(mut self) -> FormatElement {
        // we retrieve potential dangling items inside the import list
        if !self.import_list.is_empty() {
            self.sort_and_store_import_list(true);
        }

        concat_elements(self.result.into_iter())
    }

    /// It sorts the [JsImport] stored so far and then empty them
    ///
    /// [JsImport]: rome_js_syntax::JsImport
    fn sort_and_store_import_list(&mut self, is_last: bool) {
        self.import_list
            .sort_unstable_by(|left, right| left.compare(right));
        let formatted_list = self.formatted_import_list(is_last);
        self.result.push(formatted_list);
    }

    fn formatted_import_list(&mut self, is_last: bool) -> FormatElement {
        let mut found_trailing_lines = false;
        let len = self.import_list.len();
        let formatted_list = self
            .import_list
            .drain(..)
            .enumerate()
            .map(|(index, import_item)| {
                // When sorting, we might have a case where between the last import of the list and the next statement
                // that is not an import, e.g. expression statement, we have various empty lines.
                //
                // After sorting occurs, that last import might shift in first position. While doing so, we want to keep
                // the empty lines that we found consistent. This logic makes sure of that.
                if import_item.has_trailing_lines() {
                    found_trailing_lines = true
                }
                let formatted = import_item.into_format_element();
                if index + 1 == len {
                    // this is needed to cover the edge case where the document terminates with only
                    // import statements. In this case, we don't care about possible empty lines because
                    // there aren't any other statements after that
                    if is_last {
                        format_elements![formatted, hard_line_break()]
                    } else if found_trailing_lines {
                        format_elements![formatted, empty_line()]
                    } else {
                        format_elements![formatted, hard_line_break()]
                    }
                } else {
                    format_elements![formatted, hard_line_break()]
                }
            });

        concat_elements(formatted_list)
    }
}

/// Convenient enum to categorize imports that might have side effects against the ones that
/// might not.
///
/// Internally, we assume that bare import clauses **might** contain side effects, while the rest
/// **might not** contain side effects
enum Import {
    PossiblyWithSideEffects {
        node: JsImportBareClause,
        formatted: FormatElement,
        trailing_lines: usize,
    },
    Safe {
        node: SafeImport,
        formatted: FormatElement,
        trailing_lines: usize,
    },
    Ignored {
        formatted: FormatElement,
        trailing_lines: usize,
    },
}

/// Convenient enum to make the comparison of safe imports simpler
#[allow(clippy::enum_variant_names)]
enum SafeImport {
    JsImportNamedClause(JsImportNamedClause),
    JsImportDefaultClause(JsImportDefaultClause),
    JsImportNamespaceClause(JsImportNamespaceClause),
}

impl SafeImport {
    pub fn get_source_text(&self) -> FormatResult<String> {
        Ok(match self {
            SafeImport::JsImportNamedClause(node) => node.source()?.text(),
            SafeImport::JsImportDefaultClause(node) => node.source()?.text(),
            SafeImport::JsImportNamespaceClause(node) => node.source()?.text(),
        })
    }

    pub fn compare(&self, other: &Self) -> Ordering {
        // In case the source is missing, we swallow the error and keep the ordering as it is
        let self_source = self.get_source_text().ok();
        let other_self_source = other.get_source_text().ok();
        match (self_source, other_self_source) {
            (Some(self_source), Some(other_self_source)) => self_source.cmp(&other_self_source),

            _ => Ordering::Equal,
        }
    }
}

impl From<JsAnyImportClause> for SafeImport {
    fn from(any_node: JsAnyImportClause) -> Self {
        match any_node {
            JsAnyImportClause::JsImportDefaultClause(node) => {
                SafeImport::JsImportDefaultClause(node)
            }
            JsAnyImportClause::JsImportNamedClause(node) => SafeImport::JsImportNamedClause(node),
            JsAnyImportClause::JsImportNamespaceClause(node) => {
                SafeImport::JsImportNamespaceClause(node)
            }
            _ => unreachable!("JsImportBareClause should not be tracked as variant in this enum"),
        }
    }
}

impl Import {
    /// Consumes self to to return a [FormatElement]
    pub fn into_format_element(self) -> FormatElement {
        match self {
            Import::PossiblyWithSideEffects { formatted, .. } => formatted,
            Import::Safe { formatted, .. } => formatted,
            Import::Ignored { formatted, .. } => formatted,
        }
    }

    /// Tells if the current import found some trailing lines
    pub fn has_trailing_lines(&self) -> bool {
        match self {
            Import::PossiblyWithSideEffects { trailing_lines, .. } => *trailing_lines > 1,
            Import::Safe { trailing_lines, .. } => *trailing_lines > 1,
            Import::Ignored { trailing_lines, .. } => *trailing_lines > 1,
        }
    }

    pub fn compare(&self, other: &Self) -> Ordering {
        match (self, other) {
            (
                Import::PossiblyWithSideEffects { node, .. },
                Import::PossiblyWithSideEffects {
                    node: other_node, ..
                },
            ) => node.text().cmp(&other_node.text()),
            (_, Import::PossiblyWithSideEffects { .. }) => Ordering::Greater,
            (Import::PossiblyWithSideEffects { .. }, _) => Ordering::Less,
            (
                Import::Safe { node, .. },
                Import::Safe {
                    node: other_node, ..
                },
            ) => node.compare(other_node),
            _ => Ordering::Equal,
        }
    }
}

impl Debug for Import {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Import::PossiblyWithSideEffects { trailing_lines, .. } => {
                write!(f, "Side effects {trailing_lines}")
            }
            Import::Safe { trailing_lines, .. } => write!(f, "Safe {trailing_lines}"),
            Import::Ignored { trailing_lines, .. } => write!(f, "Ignored {trailing_lines}"),
        }
    }
}

/// Function that implements the sorting of imports
fn unstable_sort_imports(
    list: JsModuleItemList,
    formatter: &Formatter,
) -> FormatResult<FormatElement> {
    let mut sorted_imports = SortedImports::default();
    let mut peekable_list = list.into_iter().peekable();
    while let Some(item) = peekable_list.next() {
        // before applying sorting, we want to know how many empty lines there are between the current node
        // and the next one, so we maintain possible empty lines when we reformat the statements
        let next_item = peekable_list.peek();
        let trailing_lines = next_item.map_or(0, |next_item| get_lines_before(next_item.syntax()));

        if let JsAnyModuleItem::JsImport(import) = item {
            let formatted = import.format(formatter)?;
            sorted_imports.store_formatted_import_clause(
                import.import_clause()?,
                formatted,
                trailing_lines,
            );
        } else {
            let state = formatter.snapshot();

            // we might encounter unknown nodes, hence we catch the error case and we restore the snapshot
            let formatted = match item.format(formatter) {
                Err(_) => {
                    formatter.restore(state);
                    // Lists that yield errors are formatted as they were unknown nodes.
                    // Doing so, the formatter formats the nodes/tokens as is.
                    formatter.format_unknown(item.syntax())
                }
                Ok(element) => element,
            };
            sorted_imports.store_formatted_module_item(item, formatted, trailing_lines);
        }
    }

    Ok(sorted_imports.into_format_element())
}
