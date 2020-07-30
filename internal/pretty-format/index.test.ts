/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import "@internal/cli-layout";
import {prettyFormatToString} from "@internal/pretty-format";
import {test} from "rome";
import {Dict} from "@internal/typescript-helpers";

test(
	"strings",
	(t) => {
		t.inlineSnapshot(prettyFormatToString("yes"), '"yes"');
	},
);

test(
	"numbers",
	(t) => {
		t.inlineSnapshot(prettyFormatToString(NaN), "NaN");
		t.inlineSnapshot(prettyFormatToString(Infinity), "Infinity");
		t.inlineSnapshot(prettyFormatToString(-Infinity), "-Infinity");
		t.inlineSnapshot(prettyFormatToString(-0), "-0");
		t.inlineSnapshot(prettyFormatToString(1), "1");
		t.inlineSnapshot(prettyFormatToString(10), "10");
		t.inlineSnapshot(prettyFormatToString(100), "100");
		t.inlineSnapshot(prettyFormatToString(1_000), "1_000");
		t.inlineSnapshot(prettyFormatToString(10_000), "10_000");
		t.inlineSnapshot(prettyFormatToString(100_000), "100_000");
		t.inlineSnapshot(prettyFormatToString(1_000_000), "1_000_000");
		t.inlineSnapshot(prettyFormatToString(10_000_000), "10_000_000");
		t.inlineSnapshot(prettyFormatToString(100_000_000), "100_000_000");
		t.inlineSnapshot(prettyFormatToString(1_000_000_000), "1_000_000_000");
	},
);

test(
	"booleans",
	(t) => {
		t.inlineSnapshot(prettyFormatToString(true), "true");
		t.inlineSnapshot(prettyFormatToString(false), "false");
	},
);

test(
	"null",
	(t) => {
		t.inlineSnapshot(prettyFormatToString(null), "null");
	},
);

test(
	"undefined",
	(t) => {
		t.inlineSnapshot(prettyFormatToString(undefined), "undefined");
	},
);

test(
	"arrays",
	(t) => {
		t.inlineSnapshot(prettyFormatToString([1, 2]), "Array [\n\t1\n\t2\n]");
		t.inlineSnapshot(
			prettyFormatToString([1, [2, 3, [4, 5]]]),
			"Array [\n\t1\n\tArray [\n\t\t2\n\t\t3\n\t\tArray [\n\t\t\t4\n\t\t\t5\n\t\t]\n\t]\n]",
		);
	},
);

test(
	"regexps",
	(t) => {
		t.inlineSnapshot(prettyFormatToString(/foo/g), "/foo/g");
	},
);

test(
	"symbols",
	(t) => {
		t.inlineSnapshot(prettyFormatToString(Symbol()), "Symbol()");
		t.inlineSnapshot(prettyFormatToString(Symbol("test")), "Symbol(test)");
	},
);

test(
	"objects",
	(t) => {
		t.inlineSnapshot(prettyFormatToString({}), "Object {}");
		t.inlineSnapshot(prettyFormatToString({foo: "bar"}), 'Object {foo: "bar"}');
		t.inlineSnapshot(
			prettyFormatToString({"foo||{}": "bar"}),
			'Object {"foo||{}": "bar"}',
		);
		t.inlineSnapshot(
			prettyFormatToString({
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
			prettyFormatToString(new Set([1, 2, 3])),
			"Set [\n\t1\n\t2\n\t3\n]",
		);
		t.inlineSnapshot(
			prettyFormatToString(new Map([["a", 1], ["b", 2], ["c", 3]])),
			"Map [\n\ta => 1\n\tb => 2\n\tc => 3\n]",
		);
	},
);

test(
	"functions",
	(t) => {
		t.inlineSnapshot(prettyFormatToString(function() {}), "Function anonymous");
		t.inlineSnapshot(
			prettyFormatToString(function named() {}),
			"Function named",
		);

		function withProps() {}

		withProps.foo = function withPropsFoo() {};
		withProps.bar = "yes";
		t.inlineSnapshot(
			prettyFormatToString(withProps),
			'Function withProps {\n\tbar: "yes"\n\tfoo: Function withPropsFoo\n}',
		);

		t.inlineSnapshot(
			prettyFormatToString(String.prototype.indexOf),
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
			prettyFormatToString({foo: parallel, bar: parallel}),
			"Object {\n\tbar: Object {}\n\tfoo: Object {}\n}",
		);

		// Circular ref
		const circular: Dict<unknown> = {};
		circular.obj = circular;
		t.inlineSnapshot(
			prettyFormatToString(circular),
			"Object {obj: Circular Object 0}",
		);

		// Circular deep top ref
		const circularDeepTop: Dict<unknown> = {};
		circularDeepTop.foo = {
			bar: circularDeepTop,
		};
		t.inlineSnapshot(
			prettyFormatToString(circularDeepTop),
			"Object {foo: Object {bar: Circular Object 0}}",
		);

		// circular deep ref
		const circularDeep: Dict<Dict<unknown>> = {foo: {}};
		circularDeep.foo.bar = circularDeep.foo;
		t.inlineSnapshot(
			prettyFormatToString(circularDeep),
			"Object {foo: Object {bar: Circular Object 1}}",
		);
	},
);
