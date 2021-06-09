/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from "rome";
import CompilerContext from "./lib/CompilerContext";
import {parseJS} from "@internal/js-parser";
import {dedent} from "@internal/string-utils";
import * as signals from "./signals";
import {
	DIAGNOSTIC_CATEGORIES,
	Diagnostic,
	DiagnosticSuppression,
	equalCategoryNames,
} from "@internal/diagnostics";

function extractSuppressionsFromSource(
	sourceText: string,
): {
	suppressions: DiagnosticSuppression[];
	diagnostics: Diagnostic[];
} {
	const ast = parseJS({
		sourceType: "script",
		input: sourceText,
	});
	const context = new CompilerContext({ast});
	// Populate reducers
	context.reduceRoot(() => signals.retain);
	return {
		diagnostics: context.diagnostics.getDiagnostics(),
		suppressions: context.suppressions,
	};
}

test(
	"single category",
	async (t) => {
		const result = extractSuppressionsFromSource(
			dedent`
				// rome-ignore parse: explanation1
				foo();

				/** rome-ignore parse: explanation2 */
				bar();

				/**
				 * rome-ignore parse: explanation3
				 */
				yes();

				/**
				 * hello
				 * rome-ignore parse: explanation4
				 */
				wow();
			`,
		);

		t.is(result.suppressions.length, 4);
		t.is(result.diagnostics.length, 0);

		t.snapshot(result);
	},
);

test(
	"multiple categories",
	async (t) => {
		const result = extractSuppressionsFromSource(
			dedent`
				// rome-ignore parse(foo) parse(dog): explanation
				foo();

				/** rome-ignore parse(bar) parse(cat): explanation */
				bar();

				/**
				 * rome-ignore parse(yes) parse(frog): explanation
				 */
				yes();

				/**
				 * hello
				 * rome-ignore parse(wow) parse(fish): explanation
				 */
				wow();
			`,
		);

		t.is(result.suppressions.length, 8);
		t.is(result.diagnostics.length, 0);

		t.snapshot(result);
	},
);

test(
	"duplicates",
	async (t) => {
		const result = extractSuppressionsFromSource(
			dedent`
				// rome-ignore parse(dog) parse(dog): explanation
				foo();

				// rome-ignore parse(dog) parse(cat) parse(dog): explanation
				bar();

				// rome-ignore parse parse: explanation
				bar();
			`,
		);

		t.is(result.suppressions.length, 4);
		t.is(result.diagnostics.length, 3);
		for (const diagnostic of result.diagnostics) {
			t.true(
				equalCategoryNames(
					diagnostic.description.category,
					DIAGNOSTIC_CATEGORIES["suppressions/duplicate"],
				),
			);
		}

		t.snapshot(result);
	},
);

test(
	"overlap suppressions",
	async (t) => {
		const result = extractSuppressionsFromSource(
			dedent`
				// rome-ignore parse: explanation
				function foo_bar() {
				  // rome-ignore parse: explanation
				  bar_foo;
				}
			`,
		);

		t.is(result.suppressions.length, 2);
		t.is(result.diagnostics.length, 1);
		for (const diagnostic of result.diagnostics) {
			t.true(
				equalCategoryNames(
					diagnostic.description.category,
					DIAGNOSTIC_CATEGORIES["suppressions/overlap"],
				),
			);
		}

		t.snapshot(result);
	},
);

test(
	"overlap suppressions with suppressions in between overlaps",
	async (t) => {
		const result = extractSuppressionsFromSource(
			dedent`
				// rome-ignore parse(foo): explanation
				function foo_bar() {
				  // rome-ignore parse(bar): explanation
				  // rome-ignore parse(baz): explanation
				  // rome-ignore parse(foo): explanation
				  bar_foo;
				}
			`,
		);

		t.is(result.suppressions.length, 4);
		t.is(result.diagnostics.length, 1);

		for (const diagnostic of result.diagnostics) {
			t.true(
				equalCategoryNames(
					diagnostic.description.category,
					DIAGNOSTIC_CATEGORIES["suppressions/overlap"],
				),
			);
		}

		t.snapshot(result);
	},
);

test(
	"overlap suppression with a non-overlap suppression",
	async (t) => {
		const result = extractSuppressionsFromSource(
			dedent`
				// rome-ignore parse: foo
				function foo_bar() {
				  // rome-ignore parse: foo
				  bar_foo;
				}

				// rome-ignore parse: foo
				baz()
			`,
		);

		t.is(result.suppressions.length, 3);
		t.is(result.diagnostics.length, 1);
		for (const diagnostic of result.diagnostics) {
			t.true(
				equalCategoryNames(
					diagnostic.description.category,
					DIAGNOSTIC_CATEGORIES["suppressions/overlap"],
				),
			);
		}

		t.snapshot(result);
	},
);

test(
	"multiple overlap suppressions",
	async (t) => {
		const result = extractSuppressionsFromSource(
			dedent`
				// rome-ignore parse: foo
				function foo_bar() {
				  // rome-ignore parse: foo
				  // rome-ignore parse: foo
				  bar_foo;
				}

				// rome-ignore parse: foo
				baz()
			`,
		);

		t.is(result.suppressions.length, 4);
		t.is(result.diagnostics.length, 2);
		for (const diagnostic of result.diagnostics) {
			t.true(
				equalCategoryNames(
					diagnostic.description.category,
					DIAGNOSTIC_CATEGORIES["suppressions/overlap"],
				),
			);
		}

		t.snapshot(result);
	},
);

test(
	"incorrect suppression comment",
	async (t) => {
		const result = extractSuppressionsFromSource(
			dedent`
				// @rome-ignore parse: foo1
				boo()

				// rome-disable parse: foo2
				boo()

				// @rome-disable parse: foo3
				boo()

				// @rometools-ignore parse: foo4
				boo()

				// romefrontend-ignore parse: foo5
				boo()

				// @rometools-disable parse: foo6
				boo()

				// romefrontend-disable parse: foo7
				boo()
			`,
		);

		t.is(result.suppressions.length, 0);
		t.is(result.diagnostics.length, 7);
		for (const diagnostic of result.diagnostics) {
			t.true(
				equalCategoryNames(
					diagnostic.description.category,
					DIAGNOSTIC_CATEGORIES["suppressions/incorrectSuppressionStart"],
				),
			);
		}

		t.snapshot(result);
	},
);

test(
	"missing explanation",
	async (t) => {
		const result = extractSuppressionsFromSource(
			dedent`
				// rome-ignore parse
				boo()

				// rome-ignore parse:
				boo()
			`,
		);

		t.is(result.suppressions.length, 2);
		t.is(result.diagnostics.length, 2);
		for (const diagnostic of result.diagnostics) {
			t.true(
				equalCategoryNames(
					diagnostic.description.category,
					DIAGNOSTIC_CATEGORIES["suppressions/missingExplanation"],
				),
			);
		}

		t.snapshot(result);
	},
);

test(
	"JSX comment",
	async (t) => {
		const result = extractSuppressionsFromSource(
			dedent`
				<div>
					{/* rome-ignore lint/react/noChildrenProp: this is intentional */}
					<Cmp children={"foo"} text="something">
					</Cmp>
				</div>;
			`,
		);

		t.is(result.suppressions.length, 1);
		t.is(result.diagnostics.length, 0);

		t.snapshot(result);
	},
);
