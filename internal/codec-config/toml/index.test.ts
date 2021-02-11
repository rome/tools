import "@internal/core";
import {test} from "rome";
import {json, toml} from "@internal/codec-config";
import {dedent} from "@internal/string-utils";

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
			'foo = ["bar", "lorem"]',
		);
	},
);
