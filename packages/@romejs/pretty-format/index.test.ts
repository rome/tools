/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import '@romejs/string-markup';
import prettyFormat from '@romejs/pretty-format';
import {test} from 'rome';
import {Dict} from '@romejs/typescript-helpers';

test(
	'strings',
	(t) => {
		t.is(prettyFormat('yes'), "'yes'");
	},
);

test(
	'numbers',
	(t) => {
		t.is(prettyFormat(NaN), 'NaN');
		t.is(prettyFormat(Infinity), 'Infinity');
		t.is(prettyFormat(-Infinity), '-Infinity');
		t.is(prettyFormat(-0), '-0');
		t.is(prettyFormat(1), '1');
		t.is(prettyFormat(10), '10');
		t.is(prettyFormat(100), '100');
		t.is(prettyFormat(1_000), '1_000');
		t.is(prettyFormat(10_000), '10_000');
		t.is(prettyFormat(100_000), '100_000');
		t.is(prettyFormat(1_000_000), '1_000_000');
		t.is(prettyFormat(10_000_000), '10_000_000');
		t.is(prettyFormat(100_000_000), '100_000_000');
		t.is(prettyFormat(1_000_000_000), '1_000_000_000');
	},
);

test(
	'booleans',
	(t) => {
		t.is(prettyFormat(true), 'true');
		t.is(prettyFormat(false), 'false');
	},
);

test(
	'null',
	(t) => {
		t.is(prettyFormat(null), 'null');
	},
);

test(
	'undefined',
	(t) => {
		t.is(prettyFormat(undefined), 'undefined');
	},
);

test(
	'arrays',
	(t) => {
		t.is(prettyFormat([1, 2]), `Array [\n  1\n  2\n]`);
		t.is(
			prettyFormat([1, [2, 3, [4, 5]]]),
			'Array [\n  1\n  Array [\n    2\n    3\n    Array [\n      4\n      5\n    ]\n  ]\n]',
		);
	},
);

test(
	'regexps',
	(t) => {
		t.is(prettyFormat(/foo/g), '/foo/g');
	},
);

test(
	'symbols',
	(t) => {
		t.is(prettyFormat(Symbol()), 'Symbol()');
		t.is(prettyFormat(Symbol('test')), 'Symbol(test)');
	},
);

test(
	'objects',
	(t) => {
		t.is(prettyFormat({}), 'Object {}');
		t.is(prettyFormat({foo: 'bar'}), "Object {foo: 'bar'}");
		t.is(prettyFormat({'foo||{}': 'bar'}), "Object {'foo||{}': 'bar'}");
		t.is(
			prettyFormat({
				[Symbol('foo')]: 'bar',
				[Symbol.iterator]: 'foo',
			}),
			"Object {\n  Symbol(foo): 'bar'\n  Symbol(Symbol.iterator): 'foo'\n}",
		);
	},
);

test(
	'iterables',
	(t) => {
		t.is(prettyFormat(new Set([1, 2, 3])), 'Set [\n  1\n  2\n  3\n]');
		t.is(
			prettyFormat(new Map([['a', 1], ['b', 2], ['c', 3]])),
			'Map [\n  a => 1\n  b => 2\n  c => 3\n]',
		);
	},
);

test(
	'functions',
	(t) => {
		t.is(prettyFormat(function() {}), 'Function anonymous');
		t.is(prettyFormat(function named() {}), 'Function named');

		function withProps() {}

		withProps.foo = function withPropsFoo() {};
		withProps.bar = 'yes';
		t.is(
			prettyFormat(withProps),
			"Function withProps {\n  bar: 'yes'\n  foo: Function withPropsFoo\n}",
		);

		t.is(prettyFormat(String.prototype.indexOf), 'NativeFunction indexOf');
	},
);

test(
	'circular detection',
	(t) => {
		// Parallel ref
		const parallel = {};
		t.is(
			prettyFormat({foo: parallel, bar: parallel}),
			`Object {\n  bar: Object {}\n  foo: Object {}\n}`,
		);

		// Circular ref
		const circular: Dict<unknown> = {};
		circular.obj = circular;
		t.is(prettyFormat(circular), 'Object {obj: Circular Object 0}');

		// Circular deep top ref
		const circularDeepTop: Dict<unknown> = {};
		circularDeepTop.foo = {
			bar: circularDeepTop,
		};
		t.is(
			prettyFormat(circularDeepTop),
			'Object {foo: Object {bar: Circular Object 0}}',
		);

		// circular deep ref
		const circularDeep: Dict<Dict<unknown>> = {foo: {}};
		circularDeep.foo.bar = circularDeep.foo;
		t.is(
			prettyFormat(circularDeep),
			'Object {foo: Object {bar: Circular Object 1}}',
		);
	},
);
