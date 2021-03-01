import {test} from "rome";
import {Mapping} from "@internal/codec-source-map/types";
import {OneIndexed, ZeroIndexed} from "@internal/numbers";
import {SourceMapGenerator} from "@internal/codec-source-map/index";
import {dedent} from "@internal/string-utils";
import {createPath, createRelativePath} from "@internal/path";

// TODO: This should NOT be shared amongst tests
let generator: SourceMapGenerator;

test(
	"Verify generator serialization",
	async (t) => {
		function generateMapping(
			name: string,
			source: string,
			generatedLine: number,
			generatedColumn: number,
			originalLine: number,
			originalColumn: number,
		): Mapping {
			return {
				name,
				source: createPath(source),
				original: {
					line: new OneIndexed(originalLine),
					column: new ZeroIndexed(originalColumn),
				},
				generated: {
					line: new OneIndexed(generatedLine),
					column: new ZeroIndexed(generatedColumn),
					index: new ZeroIndexed(),
				},
			};
		}

		generator = new SourceMapGenerator({
			path: createRelativePath("bundle.js"),
			sourceRoot: "..",
		});

		generator.addMapping(generateMapping("foo", "js/file1.js", 2, 4, 1, 6));
		generator.addMapping(generateMapping("bar", "js/file1.js", 2, 24, 3, 9));
		generator.addMapping(generateMapping("hello", "js/file2.js", 2, 4, 1, 4));

		generator.setSourceContent(
			"js/file1.js",
			dedent`
				const foo = "foo";

				function bar() {
					return foo + "bar";
				}
			`,
		);

		generator.setSourceContent(
			"js/file2.js",
			dedent`
				let hello = "world";
			`,
		);

		let materializeWasCalled = false;

		generator.addMaterializer(() => {
			materializeWasCalled = true;
		});

		t.looksLike(
			generator.serialize(),
			{
				version: 3,
				file: "bundle.js",
				names: ["foo", "bar", "hello"],
				mappings: ";IAAMA,ACAFE,oBDEKD",
				sourceRoot: "..",
				sources: ["js/file1.js", "js/file2.js"],
				sourcesContent: [
					dedent`
						const foo = "foo";

						function bar() {
							return foo + "bar";
						}
					`,
					dedent`
						let hello = "world";
					`,
				],
			},
		);

		t.true(materializeWasCalled);
	},
);
