use rome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::JsModuleSource;
use rome_rowan::{AstNode, SyntaxTokenText};
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

const INDEX_BASENAMES: &[&str] = &["index", "mod"];

const SOURCE_EXTENSIONS: &[&str] = &["js", "ts", "cjs", "cts", "mjs", "mts", "jsx", "tsx"];

declare_rule! {
    /// Disallows imports from certain modules.
    ///
    /// This rules enforces the following restrictions:
    ///
    /// ## Package private visibility
    ///
    /// All exported symbols are considered to be "package private". This means that modules that
    /// reside in the same directory, as well as submodules of those "sibling" modules, are
    /// allowed to import them, while any other modules that are further away in the file system
    /// are restricted from importing them. A symbol's visibility may be extended by
    /// re-exporting from an index file.
    ///
    /// Notes:
    ///
    /// * This rule only applies to relative imports, since the API from external dependencies is
    ///   often out of your control.
    /// * This rule only applies to source imports. Imports for resources such as images or CSS
    ///   files are exempted.
    /// * A future improvement will relax the restriction from "all exported symbols" to those
    ///   that have an `@package` JSDoc annotation.
    ///
    /// This rule is intended to be extended with additional import restrictions.
    /// Please see the tracking issue to follow progress: https://github.com/rome/tools/issues/4678
    ///
    /// Source:
    ///
    /// * https://github.com/uhyo/eslint-plugin-import-access
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// // Attempt to import from `foo.js` from outside its `sub` module.
    /// import { fooPackageVariable } from "./sub/foo.js";
    ///
    /// // Attempt to import from `bar.ts` from outside its `aunt` module.
    /// import { barPackageVariable } from "../aunt/bar.ts";
    ///
    /// // Assumed to resolve to a JS/TS file.
    /// import { fooPackageVariable } from "./sub/foo";
    ///
    /// // If the `sub/foo` module is inaccessible, so is its index file.
    /// import { fooPackageVariable } from "./sub/foo/index.js";
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// // Imports within the same module are always allowed.
    /// import { fooPackageVariable } from "./foo.js";
    ///
    /// // Imports within the same module are always allowed.
    /// import { fooPackageVariable } from "./foo.js";
    ///
    /// // Resources (anything other than JS/TS files) are exempt.
    /// import { barResource } from "../aunt/bar.png";
    ///
    /// // A parent index file is accessible like other modules.
    /// import { internal } from "../../index.js";
    ///
    /// // If the `sub` module is accessible, so is its index file.
    /// import { subPackageVariable } from "./sub/index.js";
    ///
    /// // Library imports are exempt.
    /// import useAsync from "react-use/lib/useAsync";
    /// ```
    ///
    pub(crate) UseImportRestrictions {
        version: "next",
        name: "useImportRestrictions",
        recommended: false,
    }
}

impl Rule for UseImportRestrictions {
    type Query = Ast<JsModuleSource>;
    type State = ImportRestrictionsState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let binding = ctx.query();
        let Ok(path) = binding.inner_string_text() else {
            return None;
        };

        get_restricted_import(&path)
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let ImportRestrictionsState { path, suggestion } = state;

        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            ctx.query().range(),
            markup! {
                "Importing package private symbols is not allowed from outside the module directory."
            },
        );

        if let Some(suggestion) = suggestion {
            diagnostic = diagnostic.note(markup! {
                "Please import from "<Emphasis>{suggestion.display()}</Emphasis>" instead "
                "(you may need to re-export the symbol(s) from "<Emphasis>{path.display()}</Emphasis>")."
            });
        }

        Some(diagnostic)
    }
}

pub(crate) struct ImportRestrictionsState {
    /// The path that is being restricted.
    path: PathBuf,

    /// Optional suggestion from which to import instead.
    suggestion: Option<PathBuf>,
}

fn get_restricted_import(module_path: &SyntaxTokenText) -> Option<ImportRestrictionsState> {
    let mut path = PathBuf::from(module_path.text());

    if !path.starts_with(".") && !path.starts_with("..") {
        return None;
    }

    let mut index_filename = None;

    if let Some(extension) = path.extension() {
        if !SOURCE_EXTENSIONS.contains(&extension.to_str().unwrap_or_default()) {
            return None; // Resource files are exempt.
        }

        if let Some(basename) = path.file_stem() {
            if INDEX_BASENAMES.contains(&basename.to_str().unwrap_or_default()) {
                // We pop the index file because it shouldn't count as a path,
                // component, but we store the file name so we can add it to
                // both the reported path and the suggestion.
                index_filename = path.file_name().map(OsStr::to_owned);
                path.pop();
            }
        }
    }

    let is_restricted = path
        .components()
        .filter(|component| component.as_os_str() != "." && component.as_os_str() != "..")
        .count()
        > 1;
    if !is_restricted {
        return None;
    }

    let mut suggestion = path.parent().map(Path::to_owned);

    // Push the index file if it exists. This makes sure the reported path
    // matches the import path exactly.
    if let Some(index_filename) = index_filename {
        path.push(&index_filename);

        // Assumes the user probably wants to use an index file that has the
        // same name as the original.
        if let Some(alternative) = suggestion.as_mut() {
            alternative.push(index_filename);
        }
    }

    Some(ImportRestrictionsState { path, suggestion })
}
