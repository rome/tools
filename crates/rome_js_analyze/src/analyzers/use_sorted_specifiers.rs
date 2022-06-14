use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{
    JsAnyImportClause::{
        JsImportBareClause, JsImportDefaultClause, JsImportNamedClause, JsImportNamespaceClause,
    },
    JsAnyNamedImport, JsAnyRoot, JsAnyStatement, JsExport, JsForStatement, JsForStatementFields,
    JsImport, JsImportFields, JsNamedImportSpecifier, JsNamedImportSpecifiers, T,
};
use rome_rowan::{declare_node_union, AstSeparatedList, AstNode};

use crate::{
    registry::{JsRuleAction, Rule, RuleDiagnostic},
    ActionCategory, RuleCategory,
};

pub(crate) enum UseSortedSpecifiers {}

impl Rule for UseSortedSpecifiers {
    const NAME: &'static str = "useSortedSpecifiers";
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = JsAnyEImport;
    type State = ();

    fn run(n: &Self::Query) -> Option<Self::State> {
        match n {
            JsAnyEImport::JsImport(import) => {
                let import_clause = import.import_clause().ok()?;
                match import_clause {
                    JsImportBareClause(_)
                    | JsImportDefaultClause(_)
                    | JsImportNamespaceClause(_) => None,
                    JsImportNamedClause(import_named) => {
                        let named_import = import_named.named_import().ok()?;
                        match named_import {
                            JsAnyNamedImport::JsNamedImportSpecifiers(named_import_specifiers) => {
                                let specifiers = named_import_specifiers.specifiers();
                                specifiers.iter().is_sorted_by(|a, b| {
                                    if a.is_err() || b.is_err() {
                                        return None;
                                    }
                                    // SAFETY: We have been check if a, b specifier is error above.
                                    let a_specifier = a.unwrap();
                                    let b_specifier = b.unwrap();
                                    let a_local = match a_specifier {
                                        rome_js_syntax::JsAnyNamedImportSpecifier::JsNamedImportSpecifier(specifier) => specifier.to_string(),
                                        rome_js_syntax::JsAnyNamedImportSpecifier::JsShorthandNamedImportSpecifier(_) => todo!(),
                                        rome_js_syntax::JsAnyNamedImportSpecifier::JsUnknownNamedImportSpecifier(_) => todo!(),
                                    };
                                    let b_local = match b_specifier {
                                        rome_js_syntax::JsAnyNamedImportSpecifier::JsNamedImportSpecifier(_) => todo!(),
                                        rome_js_syntax::JsAnyNamedImportSpecifier::JsShorthandNamedImportSpecifier(_) => todo!(),
                                        rome_js_syntax::JsAnyNamedImportSpecifier::JsUnknownNamedImportSpecifier(_) => return None,
                                    };
                                });
                                // for sp in specifiers {
                                //     let sp = sp.ok()?;
                                    
                                // }
                                todo!()
                            },
                            JsAnyNamedImport::JsNamespaceImportSpecifier(_) => None,
                        }

                    },
                }
            }
            JsAnyEImport::JsExport(export) => {
                None
            }
        }
        None
    }

    fn diagnostic(node: &Self::Query, _: &Self::State) -> Option<RuleDiagnostic> {
        None
    }

    fn action(root: JsAnyRoot, node: &Self::Query, _: &Self::State) -> Option<JsRuleAction> {
        None
    }
}

declare_node_union! {
    /// Enum for [JsImport] and [JsExport]
    #[allow(dead_code)]
    pub(crate) JsAnyEImport  = JsImport | JsExport
}

// fn get_local_name(specifier: JsAnyNamedImportSpecifier) {
//     match specifier {

//     }
// }