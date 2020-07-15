import {test} from "rome";
import {testLint} from "../../utils/testing";

test(
	"jsx-a11y no accessKey",
	async (t) => {
		await testLint(
			t,
			{
				invalid: ['<input accessKey="key" />', "<input accessKey={key} />"],
				valid: ["<input />", "<input accessKey={undefined} />"],
				filename: "file.tsx",
				category: "lint/jsx-a11y/noAccessKey",
			},
		);
	},
);
