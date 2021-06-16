import {test} from "rome";
import {Rules} from "@internal/project/lint";
import {mergeRules} from "@internal/project/lint/merge";

test(
	"merge rules with recommended",
	(t) => {
		const a: Rules = {
			recommended: true,
		};

		const b: Rules = {};

		t.namedSnapshot("First configuration", a);
		t.namedSnapshot("Second configuration", b);
		t.namedSnapshot("B to A", mergeRules(a, b));
		t.namedSnapshot("A to B", mergeRules(b, a));
	},
);

test(
	"merge rules with the same category",
	(t) => {
		const a: Rules = {
			a11y: true,
		};

		const b: Rules = {
			a11y: false,
		};

		const result = mergeRules(a, b);

		// @ts-expect-error
		t.is(result.a11y, false);

		t.namedSnapshot("First configuration", a);
		t.namedSnapshot("Second configuration", b);
		t.namedSnapshot("Result", result);
	},
);

test(
	"merge rules with different categories",
	(t) => {
		const a: Rules = {
			a11y: true,
			react: false,
			ts: true,
		};

		const b: Rules = {
			a11y: true,
			react: true,
			ts: false,
		};

		const result = mergeRules(a, b);

		// @ts-expect-error
		t.is(result.a11y, true);
		// @ts-expect-error
		t.is(result.react, true);
		// @ts-expect-error
		t.is(result.ts, false);

		t.namedSnapshot("First configuration", a);
		t.namedSnapshot("Second configuration", b);
		t.namedSnapshot("Result", result);
	},
);

test(
	"merge rules with categories and single rules",
	(t) => {
		const a: Rules = {
			a11y: true,
			react: false,
			ts: new Map([["noExplicitAny", true]]),
			js: new Map([["noArguments", true]]),
		};

		const b: Rules = {
			a11y: true,
			react: {
				recommended: true,
			},
			ts: false,
		};

		const result = mergeRules(a, b);

		// @ts-expect-error
		t.is(result.a11y, true);
		// @ts-expect-error
		t.looksLike(result.react, {recommended: true});
		// @ts-expect-error
		t.is(result.ts, false);
		// @ts-expect-error
		t.looksLike(result.js, new Map([["noArguments", true]]));

		t.namedSnapshot("First configuration", a);
		t.namedSnapshot("Second configuration", b);
		t.namedSnapshot("Result", result);
	},
);
