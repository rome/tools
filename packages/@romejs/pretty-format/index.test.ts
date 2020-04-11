/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import '@romejs/string-markup';
import format from '@romejs/pretty-format';
import test from '@romejs/test';
import {Dict} from '@romejs/typescript-helpers';

test('strings', (t) => {
  t.is(format('yes'), "'yes'");
});

test('numbers', (t) => {
  t.is(format(NaN), 'NaN');
  t.is(format(Infinity), 'Infinity');
  t.is(format(-Infinity), '-Infinity');
  t.is(format(-0), '-0');
  t.is(format(1), '1');
  t.is(format(10), '10');
  t.is(format(100), '100');
  t.is(format(1_000), '1_000');
  t.is(format(10_000), '10_000');
  t.is(format(100_000), '100_000');
  t.is(format(1_000_000), '1_000_000');
  t.is(format(10_000_000), '10_000_000');
  t.is(format(100_000_000), '100_000_000');
  t.is(format(1_000_000_000), '1_000_000_000');
});

test('booleans', (t) => {
  t.is(format(true), 'true');
  t.is(format(false), 'false');
});

test('null', (t) => {
  t.is(format(null), 'null');
});

test('undefined', (t) => {
  t.is(format(undefined), 'undefined');
});

test(
  'arrays',
  (t) => {
    t.is(format([1, 2]), `Array [\n  1\n  2\n]`);
    t.is(
      format([1, [2, 3, [4, 5]]]),
      'Array [\n  1\n  Array [\n    2\n    3\n    Array [\n      4\n      5\n    ]\n  ]\n]',
    );
  },
);

test('regexps', (t) => {
  t.is(format(/foo/g), '/foo/g');
});

test('symbols', (t) => {
  t.is(format(Symbol()), 'Symbol()');
  t.is(format(Symbol('test')), 'Symbol(test)');
});

test('objects', (t) => {
  t.is(format({}), 'Object {}');
  t.is(format({foo: 'bar'}), "Object {foo: 'bar'}");
  t.is(format({'foo||{}': 'bar'}), "Object {'foo||{}': 'bar'}");
  t.is(format({
    [Symbol('foo')]: 'bar',
    [Symbol.iterator]: 'foo',
  }), "Object {\n  Symbol(foo): 'bar'\n  Symbol(Symbol.iterator): 'foo'\n}");
});

test('iterables', (t) => {
  t.is(format(new Set([1, 2, 3])), 'Set [\n  1\n  2\n  3\n]');
  t.is(format(new Map([
    ['a', 1],
    ['b', 2],
    ['c', 3],
  ])), 'Map [\n  a => 1\n  b => 2\n  c => 3\n]');
});

test('functions', (t) => {
  t.is(format(function() {}), 'Function anonymous');
  t.is(format(function named() {}), 'Function named');

  function withProps() {}

  withProps.foo = function withPropsFoo() {};
  withProps.bar = 'yes';
  t.is(
    format(withProps),
    "Function withProps {\n  bar: 'yes'\n  foo: Function withPropsFoo\n}",
  );

  t.is(format(String.prototype.indexOf), 'NativeFunction indexOf');
});

test(
  'circular detection',
  (t) => {
    // Parallel ref
    const parallel = {};
    t.is(
      format({foo: parallel, bar: parallel}),
      `Object {\n  bar: Object {}\n  foo: Object {}\n}`,
    );

    // Circular ref
    const circular: Dict<unknown> = {};
    circular.obj = circular;
    t.is(format(circular), 'Object {obj: Circular Object 0}');

    // Circular deep top ref
    const circularDeepTop: Dict<unknown> = {};
    circularDeepTop.foo = {
      bar: circularDeepTop,
    };
    t.is(
      format(circularDeepTop),
      'Object {foo: Object {bar: Circular Object 0}}',
    );

    // circular deep ref
    const circularDeep: Dict<Dict<unknown>> = {foo: {}};
    circularDeep.foo.bar = circularDeep.foo;
    t.is(format(circularDeep), 'Object {foo: Object {bar: Circular Object 1}}');
  },
);
