import {test} from "rome";
import {testLintMultiple} from "../testHelpers";

test(
	"disallow the scope prop on elements other than th",
	async (t) => {
		await testLintMultiple(
			t,
			[
				// INVALID
				"<div scope={scope} />",
				'<div scope="col" />',
				// VALID
				"<th scope={scope}></th>",
				'<th scope="col"></th>',
			],
			{category: "lint/jsx-a11y/scope"},
		);
	},
);
