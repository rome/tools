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
import {DiagnosticSuppressions, Diagnostics} from "@internal/diagnostics";

function extractSuppressionsFromSource(
	sourceText: string,
): {
	suppressions: DiagnosticSuppressions;
	diagnostics: Diagnostics;
} {
	const ast = parseJS({
		sourceType: "script",
		path: "unknown",
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
		const suppressions = extractSuppressionsFromSource(
			dedent`
    // rome-ignore foo: explanation
    foo();

    /** rome-ignore bar: explanation */
    bar();

    /**
     * rome-ignore yes: explanation
     */
    yes();

    /**
     * hello
     * rome-ignore wow: explanation
     */
    wow();
  `,
		);

		t.is(suppressions.suppressions.length, 4);
		t.is(suppressions.diagnostics.length, 0);

		t.snapshot(suppressions);
	},
);

test(
	"multiple categories",
	async (t) => {
		const suppressions = extractSuppressionsFromSource(
			dedent`
    // rome-ignore foo dog: explanation
    foo();

    /** rome-ignore bar cat: explanation */
    bar();

    /**
     * rome-ignore yes frog: explanation
     */
    yes();

    /**
     * hello
     * rome-ignore wow fish: explanation
     */
    wow();
  `,
		);

		t.is(suppressions.suppressions.length, 8);
		t.is(suppressions.diagnostics.length, 0);

		t.snapshot(suppressions);
	},
);

test(
	"duplicates",
	async (t) => {
		const suppressions = extractSuppressionsFromSource(
			dedent`
    // rome-ignore dog dog: explanation
    foo();

    // rome-ignore dog cat dog: explanation
    bar();
  `,
		);

		t.is(suppressions.suppressions.length, 3);
		t.is(suppressions.diagnostics.length, 2);
		for (const diagnostic of suppressions.diagnostics) {
			t.is(diagnostic.description.category, "suppressions/duplicate");
		}

		t.snapshot(suppressions);
	},
);

test(
	"overlap suppressions",
	async (t) => {
		const suppressions = extractSuppressionsFromSource(
			dedent`
      // rome-ignore foo: explanation
      function foo_bar() {
        // rome-ignore foo: explanation
        bar_foo;
      }
  `,
		);

		t.is(suppressions.suppressions.length, 2);
		t.is(suppressions.diagnostics.length, 1);
		for (const diagnostic of suppressions.diagnostics) {
			t.is(diagnostic.description.category, "suppressions/overlap");
		}

		t.snapshot(suppressions);
	},
);

test(
	"overlap suppressions with suppressions in between overlaps",
	async (t) => {
		const suppressions = extractSuppressionsFromSource(
			dedent`
      // rome-ignore foo: explanation
      function foo_bar() {
        // rome-ignore bar: explanation
        // rome-ignore baz: explanation
        // rome-ignore foo: explanation
        bar_foo;
      }
  `,
		);

		t.is(suppressions.suppressions.length, 4);
		t.is(suppressions.diagnostics.length, 1);
		for (const diagnostic of suppressions.diagnostics) {
			t.is(diagnostic.description.category, "suppressions/overlap");
		}

		t.snapshot(suppressions);
	},
);

test(
	"overlap suppression with a non-overlap suppression",
	async (t) => {
		const suppressions = extractSuppressionsFromSource(
			dedent`
      // rome-ignore foo: explanation
      function foo_bar() {
        // rome-ignore foo: explanation
        bar_foo;
      }

      // rome-ignore foo: explanation
      baz()
  `,
		);

		t.is(suppressions.suppressions.length, 3);
		t.is(suppressions.diagnostics.length, 1);
		for (const diagnostic of suppressions.diagnostics) {
			t.is(diagnostic.description.category, "suppressions/overlap");
		}

		t.snapshot(suppressions);
	},
);

test(
	"multiple overlap suppressions",
	async (t) => {
		const suppressions = extractSuppressionsFromSource(
			dedent`
      // rome-ignore foo: explanation
      function foo_bar() {
        // rome-ignore foo: explanation
        // rome-ignore foo: explanation
        bar_foo;
      }

      // rome-ignore foo: explanation
      baz()
  `,
		);

		t.is(suppressions.suppressions.length, 4);
		t.is(suppressions.diagnostics.length, 2);
		for (const diagnostic of suppressions.diagnostics) {
			t.is(diagnostic.description.category, "suppressions/overlap");
		}

		t.snapshot(suppressions);
	},
);

test(
	"incorrect suppression comment",
	async (t) => {
		const suppressions = extractSuppressionsFromSource(
			dedent`
			// @rome-ignore foo
			boo()

			// rome-disable foo
			boo()

			// @rome-disable foo
			boo()

			// @rometools-ignore foo
			boo()

			// romefrontend-ignore foo
			boo()

			// @rometools-disable foo
			boo()

			// romefrontend-disable foo
			boo()
  `,
		);

		t.is(suppressions.suppressions.length, 0);
		t.is(suppressions.diagnostics.length, 7);
		for (const diagnostic of suppressions.diagnostics) {
			t.is(
				diagnostic.description.category,
				"suppressions/incorrectSuppressionStart",
			);
		}

		t.snapshot(suppressions);
	},
);

test(
	"missing explanation",
	async (t) => {
		const suppressions = extractSuppressionsFromSource(
			dedent`
			// rome-ignore foo
			boo()

			// rome-ignore foo:
			boo()
  		`,
		);

		t.is(suppressions.suppressions.length, 2);
		t.is(suppressions.diagnostics.length, 2);
		for (const diagnostic of suppressions.diagnostics) {
			t.is(diagnostic.description.category, "suppressions/missingExplanation");
		}

		t.snapshot(suppressions);
	},
);
