import {test} from "rome";
import {testLint} from "../testHelpers";

test(
	"jsx-a11y no accessKey",
	async (t) => {
		await testLint(
			t,
			{
				invalid: ['<input accessKey="key" />', "<input accessKey={key} />"],
				valid: ["<input />", "<input accessKey={undefined} />"],
			},
			{category: "lint/jsx-a11y/noAccessKey"},
		);
	},
);
