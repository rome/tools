/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import "@internal/core";
import {rjson} from "@internal/codec-config";
import {test} from "rome";
import {ParserOptions} from "@internal/parser-core";
import {createUnknownPath} from "@internal/path";
import {Dict} from "@internal/typescript-helpers";

function consumeExtJSON(opts: ParserOptions) {
	return rjson.consume({
		...opts,
		path: createUnknownPath("input.rjson"),
	});
}

test(
	"arrays",
	(t) => {
		t.inlineSnapshot(
			rjson.stringifyFromConsumer(consumeExtJSON({input: "[]"})),
			"[]",
		);
		t.inlineSnapshot(
			rjson.stringifyFromConsumer(consumeExtJSON({input: "[1]"})),
			"[\n\t1\n]",
		);
		t.inlineSnapshot(
			rjson.stringifyFromConsumer(consumeExtJSON({input: "[1,]"})),
			"[\n\t1\n]",
		);
		t.inlineSnapshot(
			rjson.stringifyFromConsumer(consumeExtJSON({input: "[1, 2, 3]"})),
			"[\n\t1\n\t2\n\t3\n]",
		);
	},
);

test(
	"booleans",
	(t) => {
		t.inlineSnapshot(
			rjson.stringifyFromConsumer(consumeExtJSON({input: "true"})),
			"true",
		);
		t.inlineSnapshot(
			rjson.stringifyFromConsumer(consumeExtJSON({input: "false"})),
			"false",
		);
	},
);

test(
	"numbers",
	(t) => {
		t.inlineSnapshot(
			rjson.stringifyFromConsumer(consumeExtJSON({input: "1"})),
			"1",
		);
		t.inlineSnapshot(
			rjson.stringifyFromConsumer(consumeExtJSON({input: "12"})),
			"12",
		);
		t.inlineSnapshot(
			rjson.stringifyFromConsumer(consumeExtJSON({input: "123"})),
			"123",
		);
		t.inlineSnapshot(
			rjson.stringifyFromConsumer(consumeExtJSON({input: "123.45"})),
			"123.45",
		);
		t.inlineSnapshot(
			rjson.stringifyFromConsumer(
				consumeExtJSON({input: "1.2341234123412341e+27"}),
			),
			"1.2341234123412341e+27",
		);
		t.inlineSnapshot(
			rjson.stringifyFromConsumer(
				consumeExtJSON({input: "1.2341234123412341E+27"}),
			),
			"1.2341234123412341e+27",
		);
	},
);

test(
	"null",
	(t) => {
		t.inlineSnapshot(
			rjson.stringifyFromConsumer(consumeExtJSON({input: "null"})),
			"null",
		);

		const funcToNull = consumeExtJSON({input: "1"});
		funcToNull.consumer.setValue(() => {});
		t.inlineSnapshot(rjson.stringifyFromConsumer(funcToNull), "null");

		const undefinedToNull = consumeExtJSON({input: "1"});
		undefinedToNull.consumer.setValue(undefined);
		t.inlineSnapshot(rjson.stringifyFromConsumer(undefinedToNull), "null");

		const NaNToNull = consumeExtJSON({input: "1"});
		NaNToNull.consumer.setValue(NaN);
		t.inlineSnapshot(rjson.stringifyFromConsumer(NaNToNull), "NaN");
	},
);

test(
	"objects",
	(t) => {
		t.inlineSnapshot(
			rjson.stringifyFromConsumer(consumeExtJSON({input: "{}"})),
			"{}",
		);
		t.inlineSnapshot(
			rjson.stringifyFromConsumer(consumeExtJSON({input: '{"foo":"bar"}'})),
			'foo: "bar"',
		);
		t.inlineSnapshot(
			rjson.stringifyFromConsumer(consumeExtJSON({input: '{"foo":"bar",}'})),
			'foo: "bar"',
		);
		t.inlineSnapshot(
			rjson.stringifyFromConsumer(
				consumeExtJSON({input: '{"foo":"bar", "bar": "foo"}'}),
			),
			'bar: "foo"\nfoo: "bar"',
		);

		// ignore functions and undefined
		const ret = consumeExtJSON({input: "{}"});
		ret.consumer.get("foo").setValue("bar");
		ret.consumer.get("func").setValue(function() {});
		ret.consumer.get("undef").setValue(undefined);
		t.inlineSnapshot(rjson.stringifyFromConsumer(ret), 'foo: "bar"');
	},
);

const complexTest = `// root comment
/* and another!*/
foo: {
  // comment before property
  bar: {nested: true}
  great: 1.233e+58
  yes: null
}
// hello!
hello: [
  // comment before element
  "world"
  2
  3.53
]`;
test(
	"complex",
	(t) => {
		const consumer = consumeExtJSON({input: complexTest});
		t.inlineSnapshot(
			rjson.stringifyFromConsumer(consumer),
			'// root comment\n/* and another!*/\nfoo: {\n\t// comment before property\n\tbar: {\n\t\tnested: true\n}\n\tgreat: 1.233e+58\n\tyes: null\n}\n// hello!\nhello: [\n\t// comment before element\n\t"world"\n\t2\n\t3.53\n]',
		);
	},
);

test(
	"comments",
	(t) => {
		t.inlineSnapshot(
			rjson.stringifyFromConsumer(consumeExtJSON({input: "// foo\ntrue"})),
			"// foo\ntrue",
		);
		t.inlineSnapshot(
			rjson.stringifyFromConsumer(consumeExtJSON({input: "true\n// foo"})),
			"// foo\ntrue",
		);

		//# Comments - loose

		// comments at end of object
		t.inlineSnapshot(
			rjson.stringifyFromConsumer(
				consumeExtJSON({
					input: `{
    "foo": "bar",
    // end comment
  }`,
				}),
			),
			'foo: "bar"\n// end comment',
		);
		// comments at end of array
		t.inlineSnapshot(
			rjson.stringifyFromConsumer(
				consumeExtJSON({
					input: `[
    "foobar",
    // end comment
  ]`,
				}),
			),
			'[\n\t"foobar"\n\t// end comment\n]',
		);
		// comments in empty array
		t.inlineSnapshot(
			rjson.stringifyFromConsumer(
				consumeExtJSON({
					input: `[
    // inner comment
  ]`,
				}),
			),
			"[\n\t// inner comment\n]",
		);
		// comments in empty object
		t.inlineSnapshot(
			rjson.stringifyFromConsumer(
				consumeExtJSON({
					input: `{
    // inner comment
  }`,
				}),
			),
			"{\n  // inner comment\n}",
		);

		//# Comments - object property

		// before property
		t.inlineSnapshot(
			rjson.stringifyFromConsumer(
				consumeExtJSON({
					input: `{
    /* bar */
    "foo": "bar",
  }`,
				}),
			),
			'/* bar */\nfoo: "bar"',
		);
		// before value
		t.inlineSnapshot(
			rjson.stringifyFromConsumer(
				consumeExtJSON({
					input: `{
    "foo": /* bar */ "bar",
  }`,
				}),
			),
			'/* bar */\nfoo: "bar"',
		);
		// after value
		t.inlineSnapshot(
			rjson.stringifyFromConsumer(
				consumeExtJSON({
					input: `{
    "foo": "bar" /* bar */,
  }`,
				}),
			),
			'/* bar */\nfoo: "bar"',
		);

		//# Comments - array element

		// before element
		t.inlineSnapshot(
			rjson.stringifyFromConsumer(
				consumeExtJSON({
					input: `[
    /* bar */
    "foo",
  ]`,
				}),
			),
			'[\n\t/* bar */\n\t"foo"\n]',
		);
		// after value
		t.inlineSnapshot(
			rjson.stringifyFromConsumer(
				consumeExtJSON({
					input: `[
    "foo" /* bar */,
  ]`,
				}),
			),
			'[\n\t/* bar */\n\t"foo"\n]',
		);
	},
);

test(
	"recursion",
	(t) => {
		t.throws(() => {
			const ret = consumeExtJSON({input: "{}"});
			const foo: Dict<unknown> = {};
			foo.bar = foo;
			ret.consumer.get("foo").setValue(foo);
			rjson.stringifyFromConsumer(ret);
		});
	},
);
