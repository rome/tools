import {test} from "rome";
import {testLint} from "../../utils/testing";

const JSX_CASES = [
	"<div></div>",
	"<></>",
	"<Fragment></Fragment>",
	"<React.Fragment></React.Fragment>",
];

const NON_JSX_CASES = ["'<div></div>'"];

test(
	"jsx file extension",
	async (t) => {
		await testLint(
			t,
			{
				invalid: JSX_CASES,
				valid: NON_JSX_CASES,
			},
			{category: "lint/jsx/fileExtension", path: "test.js"},
		);

		await testLint(
			t,
			{
				valid: JSX_CASES,
			},
			{category: "lint/jsx/fileExtension", path: "test.jsx"},
		);

		await testLint(
			t,
			{
				valid: JSX_CASES,
			},
			{category: "lint/jsx/fileExtension", path: "test.tsx"},
		);
	},
);
