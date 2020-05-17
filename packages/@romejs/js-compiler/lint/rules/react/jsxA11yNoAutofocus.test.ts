import {test} from "rome";
import {testLintMultiple} from "../testHelpers";

test(
	"disallow the autoFocus prop",
	async (t) => {
		await testLintMultiple(
			t,
			[
				// INVALID
				"<input autoFocus />",
				'<input autoFocus="true" />',
				'<input autoFocus="false" />',
				// VALID
				"<input />",
				"<input autoFocus={undefined} />",
			],
			{category: "lint/jsxA11yNoAutofocus"},
		);
	},
);
