import {test} from "rome";
import buildPatchCodeFrame from "./buildPatchCodeFrame";
import stringDiff from "@internal/string-diff";
import {markupToPlainText} from "@internal/cli-layout";
import {dedent} from "@internal/string-utils";
import {joinMarkupLines} from "@internal/markup";

type Test = {
	before: string;
	after: string;
};

const tests: Array<Test> = [
	// Single line removal
	{
		before: "foo",
		after: "bar",
	},
	// Multiple line removal
	{
		before: "Sebastian\nMcKenzie",
		after: "Ana\nBarreto",
	},
	// Whitespace addition
	{
		before: "Sebastian",
		after: "Seb astian",
	},
	// Multiple whitespace addition
	{
		before: "Sebastian",
		after: "Seb  astian",
	},
	// Tab addition
	{
		before: "Sebastian",
		after: "Seb\tastian",
	},
	// Multiple tab addition
	{
		before: "Sebastian",
		after: "Seb\t\tastian",
	},
	// Some large code
	{
		before: dedent`
			let namedBackReference = "";
			let namedBackReferenceIndex = ob1Get0(index) + 2;
			let namedBackReferenceChar = input[namedBackReferenceIndex];
			if (namedBackReferenceChar === "<") {
				namedBackReferenceChar = input[namedBackReferenceIndex];
				while (
					namedBackReferenceChar !== ">" &&
					namedBackReferenceIndex < input.length
				) {
					namedBackReference += namedBackReferenceChar;
					namedBackReferenceIndex++;
					namedBackReferenceChar = input[namedBackReferenceIndex];
				}
				if (namedBackReferenceChar === ">") {
					namedBackReference += namedBackReferenceChar;
					namedBackReferenceIndex++;
				}
				return this.finishComplexToken(
					"NamedBackReferenceCharacter",
					{
						value: namedBackReference,
						escaped: true,
					},
					ob1Coerce0(namedBackReferenceIndex),
				);
			}
		`,
		after: dedent`
			let value = "";
			let [char, next] = this.getInputChar(index, 2);

			if (char === "<") {
				while (!this.isEOF(next)) {
					value += char;
					[char, next] = this.getInputChar(index, 1);

					if (char === ">") {
						break;
					}
				}

				return this.finishComplexToken(
					"NamedBackReferenceCharacter",
					{
						value,
						escaped: true,
					},
					index,
				);
			}
		`,
	},
	// https://github.com/romefrontend/rome/issues/679
	{
		before: dedent`
			<section>
				<>
					<div />
					<div />
				</>
			</section/>
		`,
		after: dedent`
			<section>
				<div />
				<div />
			</section>
		`,
	},
	// Control characters
	{
		before: "\x01",
		after: "",
	},
	{
		before: "\x7f",
		after: "",
	},
];

test(
	"buildPatchCodeFrame",
	(t) => {
		for (const {before, after} of tests) {
			const lines = [];
			lines.push("# Before");
			lines.push(before);
			lines.push("");
			lines.push("# After");
			lines.push(after);
			lines.push("");
			lines.push("# Diff");
			lines.push(
				joinMarkupLines(
					markupToPlainText(
						buildPatchCodeFrame(
							{
								type: "diff",
								language: "unknown",
								diff: stringDiff(before, after),
							},
							true,
						).frame,
					),
				),
			);
			t.snapshot(lines.join("\n"));
		}
	},
);
