import {test} from "rome";
import {testLint} from "../testHelpers";

test(
	"jsx-a11y scope",
	async (t) => {
		await testLint(
			t,
			{
				invalid: ["<div scope={scope} />", '<div scope="col" />'],
				valid: ["<th scope={scope}></th>", '<th scope="col"></th>'],
			},
			{category: "lint/jsx-a11y/scope"},
		);
	},
);
