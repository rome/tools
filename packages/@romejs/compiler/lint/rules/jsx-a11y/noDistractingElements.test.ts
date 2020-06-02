import {test} from "rome";
import {testLint} from "../testHelpers";

test(
	"jsx-a11y no distracting elements",
	async (t) => {
		await testLint(
			t,
			{invalid: ["<blink />", "<marquee />"], valid: ["<div />"]},
			{category: "lint/jsx-a11y/noDistractingElements"},
		);
	},
);
