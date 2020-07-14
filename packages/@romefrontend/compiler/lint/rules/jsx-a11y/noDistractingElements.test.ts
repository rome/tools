import {test} from "rome";
import {testLint} from "../../utils/testing";

test(
	"jsx-a11y no distracting elements",
	async (t) => {
		await testLint(
			t,
			{
				invalid: ["<blink />", "<marquee />"],
				valid: ["<div />"],
				filename: "file.tsx",
				category: "lint/jsx-a11y/noDistractingElements",
			},
		);
	},
);
