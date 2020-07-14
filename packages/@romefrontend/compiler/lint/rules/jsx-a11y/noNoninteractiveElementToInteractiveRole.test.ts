import {test} from "rome";
import {testLint} from "../../utils/testing";

test(
	"jsx-a11y no noninteractive element to interactive role",
	async (t) => {
		await testLint(
			t,
			{
				invalid: ["<h1 role='checkbox'></h1>"],
				valid: ["<h1 role='article'></h1>"],
				filename: "file.tsx",
				category: "lint/jsx-a11y/noNoninteractiveElementToInteractiveRole",
			},
		);
	},
);
