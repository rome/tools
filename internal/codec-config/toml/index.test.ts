import "@internal/core";
import {getFixtures} from "@internal/test-helpers";
import {test} from "rome";
import {json, toml} from "@internal/codec-config";
import {dedent} from "@internal/string-utils";
import {DiagnosticsError} from "@internal/diagnostics";
import {isObject} from "@internal/typescript-helpers";

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
				async (t) => {
					if (flatName === "invalid") {
						await t.throwsAsync(
							async () => {
								const file = fixture.files.assert(`${basename}.toml`);
								const input = await file.readAsText();
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

						const jsonStruct = JSON.parse(
							await jsonFile.readAsText(),
							(key, obj) => {
								if (isObject(obj) && typeof obj.type === "string") {
									switch (obj.type) {
										case "integer":
										case "float":
											return Number(obj.value);

										case "bool":
											return obj.value === "true";

										case "datetime": {
											if (typeof obj.value === "string") {
												return new Date(obj.value);
											}
											break;
										}

										case "array":
										case "string":
											return obj.value;
									}
								}
								return obj;
							},
						);

						const tomlValue = toml.parse({
							path: tomlFile.relative,
							input: await tomlFile.readAsText(),
							includeSourceTextInDiagnostics: true,
						});

						t.deepEquals(tomlValue, jsonStruct);

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
