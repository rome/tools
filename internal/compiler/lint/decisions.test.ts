import {test} from "rome";
import {dedent} from "@internal/string-utils";
import {parseDecisionStrings} from "@internal/compiler";
import {createAbsoluteFilePath} from "@internal/path";
import lint from "./index";
import {parseJS} from "@internal/js-parser";
import {createDefaultProjectConfig} from "@internal/project";

test(
	"apply single autofix",
	async (t) => {
		const {lintCompilerOptionsPerFile} = parseDecisionStrings(
			["fix-lint/js/doubleEquals-foo.ts-2:0-0"],
			createAbsoluteFilePath("/"),
			() => {},
		);
		const compilerOptions = lintCompilerOptionsPerFile["/foo.ts"];

		const sourceText = dedent`
		let foo;
		foo == "bar";
	`;

		const res = await lint({
			applySafeFixes: true,
			project: {
				directory: undefined,
				config: createDefaultProjectConfig(),
			},
			options: {
				lint: compilerOptions,
			},
			sourceText,
			ast: parseJS({path: "unknown", input: sourceText}),
		});
		t.is(res.diagnostics.length, 1);
		t.snapshot(res.src);
	},
);
