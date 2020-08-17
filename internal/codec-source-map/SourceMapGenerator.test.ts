import {test} from "rome";
import {Mapping} from "@internal/codec-source-map/types";
import {ob1Coerce0, ob1Coerce1} from "@internal/ob1";
import {
	SourceMapConsumer,
	SourceMapGenerator,
} from "@internal/codec-source-map/index";
import {dedent} from "@internal/string-utils";

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
				source,
				original: {
					line: ob1Coerce1(originalLine),
					column: ob1Coerce0(originalColumn),
				},
				generated: {
					line: ob1Coerce1(generatedLine),
					column: ob1Coerce0(generatedColumn),
					index: ob1Coerce0(0),
				},
			};
		}

		generator = new SourceMapGenerator({
			file: "bundle.js",
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

test(
	"Verify generator comment, json & consumer",
	async (t) => {
		function toBase64(input: string) {
			return Buffer.from(input).toString("base64");
		}

		t.is(generator.toJSON(), JSON.stringify(generator.serialize()));

		t.is(
			generator.toComment(),
			`//# sourceMappingURL=data:application/json;charset=utf-8;base64,${toBase64(
				generator.toJSON(),
			)}`,
		);

		t.looksLike(
			generator.toConsumer(),
			SourceMapConsumer.fromJSON(generator.serialize()),
		);
	},
);
