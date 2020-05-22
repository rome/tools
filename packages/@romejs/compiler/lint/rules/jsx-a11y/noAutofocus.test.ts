import {test} from "rome";
import {testLintMultiple} from "../testHelpers";

test(
	"jsx a11y no autoFocus",
	async (t) => {
		await testLintMultiple(
			t,
			[
				// INVALID
				"<input autoFocus />",
				'<input autoFocus="true" />',
				'<input autoFocus={"false"} />',
				// VALID
				"<input />",
				"<input autoFocus={undefined} />",
			],
			{category: "lint/jsx-a11y/noAutofocus"},
		);
	},
);
