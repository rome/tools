/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import "@romefrontend/cli-layout";
import prettyFormat from "@romefrontend/pretty-format";
import {test} from "rome";
import {Dict} from "@romefrontend/typescript-helpers";

test(
	"strings",
	(t) => {
		t.inlineSnapshot(prettyFormat("yes"), '"yes"');
	},
);

test(
	"numbers",
	(t) => {
		t.inlineSnapshot(prettyFormat(NaN), "NaN");
		t.inlineSnapshot(prettyFormat(Infinity), "Infinity");
		t.inlineSnapshot(prettyFormat(-Infinity), "-Infinity");
		t.inlineSnapshot(prettyFormat(-0), "-0");
		t.inlineSnapshot(prettyFormat(1), "1");
		t.inlineSnapshot(prettyFormat(10), "10");
		t.inlineSnapshot(prettyFormat(100), "100");
		t.inlineSnapshot(prettyFormat(1_000), "1_000");
		t.inlineSnapshot(prettyFormat(10_000), "10_000");
		t.inlineSnapshot(prettyFormat(100_000), "100_000");
		t.inlineSnapshot(prettyFormat(1_000_000), "1_000_000");
		t.inlineSnapshot(prettyFormat(10_000_000), "10_000_000");
		t.inlineSnapshot(prettyFormat(100_000_000), "100_000_000");
		t.inlineSnapshot(prettyFormat(1_000_000_000), "1_000_000_000");
	},
);

test(
	"booleans",
	(t) => {
		t.inlineSnapshot(prettyFormat(true), "true");
		t.inlineSnapshot(prettyFormat(false), "false");
	},
);

test(
	"null",
	(t) => {
		t.inlineSnapshot(prettyFormat(null), "null");
	},
);

test(
	"undefined",
	(t) => {
		t.inlineSnapshot(prettyFormat(undefined), "undefined");
	},
);

test(
	"arrays",
	(t) => {
		t.inlineSnapshot(prettyFormat([1, 2]), "Array [\n\t1\n\t2\n]");
		t.inlineSnapshot(
			prettyFormat([1, [2, 3, [4, 5]]]),
			"Array [\n\t1\n\tArray [\n\t\t2\n\t\t3\n\t\tArray [\n\t\t\t4\n\t\t\t5\n\t\t]\n\t]\n]",
		);
	},
);

test(
	"regexps",
	(t) => {
		t.inlineSnapshot(prettyFormat(/foo/g), "/foo/g");
	},
);

test(
	"symbols",
	(t) => {
		t.inlineSnapshot(prettyFormat(Symbol()), "Symbol()");
		t.inlineSnapshot(prettyFormat(Symbol("test")), "Symbol(test)");
	},
);

test(
	"objects",
	(t) => {
		t.inlineSnapshot(prettyFormat({}), "Object {}");
		t.inlineSnapshot(prettyFormat({foo: "bar"}), 'Object {foo: "bar"}');
		t.inlineSnapshot(
			prettyFormat({"foo||{}": "bar"}),
			'Object {"foo||{}": "bar"}',
		);
		t.inlineSnapshot(
			prettyFormat({
				[Symbol("foo")]: "bar",
				[Symbol.iterator]: "foo",
			}),
			'Object {\n\tSymbol(foo): "bar"\n\tSymbol(Symbol.iterator): "foo"\n}',
		);
	},
);

test(
	"iterables",
	(t) => {
		t.inlineSnapshot(
			prettyFormat(new Set([1, 2, 3])),
			"Set [\n\t1\n\t2\n\t3\n]",
		);
		t.inlineSnapshot(
			prettyFormat(new Map([["a", 1], ["b", 2], ["c", 3]])),
			"Map [\n\ta => 1\n\tb => 2\n\tc => 3\n]",
		);
	},
);

test(
	"functions",
	(t) => {
		t.inlineSnapshot(prettyFormat(function() {}), "Function anonymous");
		t.inlineSnapshot(prettyFormat(function named() {}), "Function named");

		function withProps() {}

		withProps.foo = function withPropsFoo() {};
		withProps.bar = "yes";
		t.inlineSnapshot(
			prettyFormat(withProps),
			'Function withProps {\n\tbar: "yes"\n\tfoo: Function withPropsFoo\n}',
		);

		t.inlineSnapshot(
			prettyFormat(String.prototype.indexOf),
			"NativeFunction indexOf",
		);
	},
);

test(
	"circular detection",
	(t) => {
		// Parallel ref
		const parallel = {};
		t.inlineSnapshot(
			prettyFormat({foo: parallel, bar: parallel}),
			"Object {\n\tbar: Object {}\n\tfoo: Object {}\n}",
		);

		// Circular ref
		const circular: Dict<unknown> = {};
		circular.obj = circular;
		t.inlineSnapshot(prettyFormat(circular), "Object {obj: Circular Object 0}");

		// Circular deep top ref
		const circularDeepTop: Dict<unknown> = {};
		circularDeepTop.foo = {
			bar: circularDeepTop,
		};
		t.inlineSnapshot(
			prettyFormat(circularDeepTop),
			"Object {foo: Object {bar: Circular Object 0}}",
		);

		// circular deep ref
		const circularDeep: Dict<Dict<unknown>> = {foo: {}};
		circularDeep.foo.bar = circularDeep.foo;
		t.inlineSnapshot(
			prettyFormat(circularDeep),
			"Object {foo: Object {bar: Circular Object 1}}",
		);
	},
);
