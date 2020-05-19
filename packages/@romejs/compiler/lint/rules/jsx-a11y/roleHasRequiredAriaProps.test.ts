import {test} from "rome";
import {testLintMultiple} from "../testHelpers";

test(
	"role has required aria props",
	async (t) => {
		await testLintMultiple(
			t,
			[
				// INVALID
				"<span role='checkbox'></span>",
				// VALID
				"<span role='checkbox' aria-checked='true'></span>",
			],
			{category: "lint/jsx-a11y/roleHasRequiredAriaProps"},
		);
	},
);
