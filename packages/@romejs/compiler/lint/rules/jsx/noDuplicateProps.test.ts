import {test} from "rome";
import {testLint} from "../../utils/testing";

test(
	"react jsx no duplicate props",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					'<Hello foo="bar" foo="baz" />',
					'<div style="{}" style="{}" id="foo" id="bar" />',
				],
				valid: ['<Hello foo="bar" />', '<div  style="{}" />'],
			},
			{category: "lint/jsx/noDuplicateProps"},
		);
	},
);
