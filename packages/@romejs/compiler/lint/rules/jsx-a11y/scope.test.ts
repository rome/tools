import {test} from "rome";
import {testLintMultiple} from "../testHelpers";

test(
	"jsx-a11y scope",
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
