import {test} from "rome";
import {testLintMultiple} from "../testHelpers";
import {dedent} from "@romejs/string-utils";

test(
	"enforce to not declare variables that are not used",
	async (t) => {
		await testLintMultiple(
			t,
			[
				// INVALID
				// `const a = 4;`,
				// `let a = 4;`,
				// dedent`function foo(myVar) {
				// 	console.log("foo");
				// };`,
				dedent`function foo(myVar) { 
					console.log("foo"); 
				};
				foo();`,
				// dedent`const foo = (b) => {
				// 	console.log("foo");
				// };`,
				// // VALID
				// dedent`function foo(_, b) {
				// 	console.log(b)
				// };
				// foo();`,
				// dedent`const foo = (_, b) => {
				// 	console.log(b)
				// };
				// foo();`,
			],
			{category: "lint/js/unusedVariables"},
		);
	},
);
