import {test} from "rome";
import {testLintMultiple} from "../testHelpers";

test(
	"jsx a11y no distracting elements",
	async (t) => {
		await testLintMultiple(
			t,
			[
				// INVALID
				"<blink />",
				"<marquee />",
				// VALID
				"<div />",
			],
			{category: "lint/jsx-a11y/noDistractingElements"},
		);
	},
);
