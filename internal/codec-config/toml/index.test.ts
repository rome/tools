import "@internal/core";
import {getFixtures} from "@internal/test-helpers";
import {test} from "rome";
import {json, toml} from "@internal/codec-config";
import {dedent} from "@internal/string-utils";
import {DiagnosticsError} from "@internal/diagnostics";

test(
	"should convert a plain key/value",
	(t) => {
		t.inlineSnapshot(
			toml.stringifyFromConsumer(json.consume({input: '{ "foo": "bar" }'})),
			'foo = "bar"',
		);
	},
);

test(
	"should convert nested objects",
	(t) => {
		t.inlineSnapshot(
			toml.stringifyFromConsumer(
				json.consume({
					input: dedent`
						{
						"foo": {
								"lorem": "ipsum",
								"test1": "test2"
							}
						}
					`,
				}),
			),
			'[foo]\nlorem = "ipsum"\ntest1 = "test2"',
		);
	},
);

test(
	"should convert numbers",
	(t) => {
		t.inlineSnapshot(
			toml.stringifyFromConsumer(json.consume({input: '{ "foo": 198 }'})),
			"foo = 198",
		);
	},
);

test(
	"should convert an array",
	(t) => {
		t.inlineSnapshot(
			toml.stringifyFromConsumer(
				json.consume({input: '{ "foo": ["bar", "lorem"] }'}),
			),
			'foo = [\n\t"bar",\n\t"lorem",\n]',
		);
	},
);

async function declareTests() {
	for (const fixture of await getFixtures()) {
		const basenames = new Set(
			Array.from(
				fixture.files.values(),
				(file) => file.relative.getExtensionlessBasename(),
			),
		);
		const flatName = fixture.name[0];

		for (const basename of basenames) {
			test(
				[flatName, basename],
				(t) => {
					return;
					if (flatName === "invalid") {
						t.throws(
							() => {
								const file = fixture.files.assert(`${basename}.toml`);
								const input = file.contentAsText();
								toml.parse({
									path: file.relative,
									input,
								});
							},
							DiagnosticsError,
						);
						return;
					}

					if (flatName === "valid") {
						const tomlFile = fixture.files.assert(`${basename}.toml`);
						const jsonFile = fixture.files.assert(`${basename}.json`);

						const jsonStruct = JSON.parse(jsonFile.contentAsText());

						const tomlValue = toml.parse({
							path: tomlFile.relative,
							input: tomlFile.contentAsText(),
							includeSourceTextInDiagnostics: true,
						});

						return;
					}

					throw new Error(`Unknown fixture name ${flatName}`);
				},
			);
		}
	}
}

// @ts-ignore Doesn't support top-level await
await declareTests();
