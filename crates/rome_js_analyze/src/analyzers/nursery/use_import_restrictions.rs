use rome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::JsModuleSource;
use rome_rowan::{AstNode, SyntaxTokenText};
use std::path::{Path, PathBuf};

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
    type State = PathBuf;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let binding = ctx.query();
        let Ok(path) = binding.inner_string_text() else {
            return None;
        };

        get_restricted_import(&path)
    }

    fn diagnostic(ctx: &RuleContext<Self>, path: &Self::State) -> Option<RuleDiagnostic> {
        let parent = path.parent().and_then(Path::to_str).unwrap_or_default();
        let path = path.to_str().unwrap_or_default();

        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            ctx.query().range(),
            markup! {
                "Importing package private symbols is not allowed from outside the module directory."
            },
        )
        .note(markup! {
            "Please import from "<Emphasis>{parent}</Emphasis>" instead "
            "(you may need to re-export the symbol(s) from "<Emphasis>{path}</Emphasis>")."
        });

        Some(diagnostic)
    }
}

fn get_restricted_import(module_path: &SyntaxTokenText) -> Option<PathBuf> {
    let mut path = PathBuf::from(module_path.text());

    if !path.starts_with(".") && !path.starts_with("..") {
        return None;
    }

    if let Some(extension) = path.extension() {
        if !SOURCE_EXTENSIONS.contains(&extension.to_str().unwrap_or_default()) {
            return None; // Resource files are exempt.
        }

        if let Some(basename) = path.file_stem() {
            if INDEX_BASENAMES.contains(&basename.to_str().unwrap_or_default()) {
                path.pop(); // We pretend the index file was never there.
            }
        }
    }

    (path
        .components()
        .filter(|component| component.as_os_str() != "." && component.as_os_str() != "..")
        .count()
        > 1)
    .then_some(path)
}
