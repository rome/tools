import "@internal/core";
import {test} from "rome";
import {json, toml} from "@internal/codec-config";
import {ParserOptions} from "@internal/parser-core";
import {createUnknownPath} from "@internal/path";
import {dedent} from "@internal/string-utils";

function consumeExtTOML(opts: ParserOptions) {
	return json.consume({
		...opts,
		path: createUnknownPath("input.json"),
	});
}

test(
	"should convert a plain key/value",
	(t) => {
		t.inlineSnapshot(
			toml.stringifyFromConsumer(consumeExtTOML({input: '{ "foo": "bar" }'})),
			'foo = "bar"',
		);
	},
);

test(
	"should convert nested objects",
	(t) => {
		t.inlineSnapshot(
			toml.stringifyFromConsumer(
				consumeExtTOML({
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
			toml.stringifyFromConsumer(consumeExtTOML({input: '{ "foo": 198 }'})),
			"foo = 198",
		);
	},
);

test(
	"should convert an array",
	(t) => {
		t.inlineSnapshot(
			toml.stringifyFromConsumer(
				consumeExtTOML({input: '{ "foo": ["bar", "lorem"] }'}),
			),
			'foo = ["bar", "lorem"]',
		);
	},
);
