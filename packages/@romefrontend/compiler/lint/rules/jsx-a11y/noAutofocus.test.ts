import {test} from "rome";
import {testLint} from "../../utils/testing";

test(
	"jsx-a11y no autoFocus",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					"<input autoFocus />",
					'<input autoFocus="true" />',
					'<input autoFocus={"false"} />',
				],
				valid: ["<input />", "<input autoFocus={undefined} />"],
				filename: "file.tsx",
				category: "lint/jsx-a11y/noAutofocus",
			},
		);
	},
);
