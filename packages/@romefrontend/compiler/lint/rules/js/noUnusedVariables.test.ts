import {test} from "rome";
import {testLint} from "../../utils/testing";
import {dedent} from "@romefrontend/string-utils";

test(
	"enforce to not declare variables that are not used",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					`const a = 4;`,
					`let a = 4;`,
					dedent`
						function foo(myVar) {
							console.log("foo");
						};
					`,
					dedent`
						function foo(myVar) {
							console.log("foo");
						}
						foo();
					`,
					dedent`
						const foo = (b) => {
							console.log("foo");
						};
					`,
				],
				valid: [
					dedent`
						function foo(_, b) {
							console.log(b)
						};
						foo();
					`,
					dedent`
						const foo = (_, b) => {
							console.log(b)
						};
						foo();
					`,
				],
			},
			{category: "lint/js/noUnusedVariables"},
		);
	},
);
