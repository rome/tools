import {createRelativeFilePath} from "@internal/path";
import {test} from "rome";
import {testLint} from "../../utils/testing";

test(
	"ts use simplified boolean expression",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [
					"function foo(x: boolean) { return !!x; }",
					"function foo(x: boolean) { return x === true; }",
					"function foo(x: boolean) { return false === x; }",
					"function foo(x: boolean, y: boolean) { return (x || y) === true }",
				],
				valid: [
					"function foo(x?: boolean) { return !!x; }",
					"function foo(x: boolean | number) { return x === true; }",
					"function foo(x: boolean | undefined) { return false === x; }",
				],
				path: createRelativeFilePath("file.ts"),
				category: "lint/ts/useSimplifiedBooleanExpression",
			},
		);
	},
);
