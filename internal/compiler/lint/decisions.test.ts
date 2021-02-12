import {test} from "rome";
import {dedent} from "@internal/string-utils";
import {parseDecisionStrings} from "@internal/compiler";
import {createAbsoluteFilePath, createRelativeFilePath} from "@internal/path";
import lint from "./index";
import {parseJS} from "@internal/js-parser";
import {OneIndexed, ZeroIndexed} from "@internal/math";

test(
	"apply single autofix",
	async (t) => {
		const {lintCompilerOptionsPerFile} = parseDecisionStrings({
			path: createRelativeFilePath("test"),
			decisions: [
				{
					value: "fix-lint/js/noDoubleEquals-foo.ts-2:0-0",
					start: {line: new OneIndexed(), column: new ZeroIndexed()},
				},
			],
			cwd: createAbsoluteFilePath("/"),
			unexpected: () => {},
		});
		const compilerOptions = lintCompilerOptionsPerFile["/foo.ts"];

		const sourceText = dedent`
			let foo;
			foo == "bar";
		`;

		const res = await lint({
			applySafeFixes: true,
			options: {
				lint: compilerOptions,
			},
			sourceText,
			ast: parseJS({input: sourceText}),
		});
		t.is(res.diagnostics.length, 1);
		t.snapshot(res.formatted);
	},
);
