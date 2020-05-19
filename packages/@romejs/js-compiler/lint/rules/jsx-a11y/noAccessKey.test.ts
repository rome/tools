import {test} from "rome";
import {testLintMultiple} from "../testHelpers";

test(
	"disallow the accessKey prop",
	async (t) => {
		await testLintMultiple(
			t,
			[
				// INVALID
				'<input accessKey="key" />',
				"<input accessKey={key} />",
				// VALID
				"<input />",
				"<input accessKey={undefined} />",
			],
			{category: "lint/jsx-a11y/noAccessKey"},
		);
	},
);
