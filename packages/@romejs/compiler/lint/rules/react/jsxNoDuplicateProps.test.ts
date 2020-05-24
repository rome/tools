import {test} from "rome";
import {testLintMultiple} from "../testHelpers";

test(
	"react jsx no duplicate props",
	async (t) => {
		await testLintMultiple(
			t,
			[
				// INVALID
				'<Hello foo="bar" foo="baz" />',
				'<div style="{}" style="{}" id="foo" id="bar" />',
				// VALID
				'<Hello foo="bar" />',
				'<div  style="{}" />',
			],
			{category: "lint/react/jsxNoDuplicateProps"},
		);
	},
);
