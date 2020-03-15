/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import '@romejs/core';
import {parseJSON, messages} from '@romejs/codec-json';
import test from '@romejs/test';
import {ParserOptions} from '@romejs/parser-core';
import {createUnknownFilePath} from '@romejs/path';

// These are just some very basic tests, most of it is already covered by test262-parse so most are redundant

function parseExtJSON(opts: ParserOptions) {
  return parseJSON({...opts, path: createUnknownFilePath('input.rjson')});
}

test('comments', t => {
  // comment at beginning
  t.true(parseExtJSON({input: '// comment\ntrue'}));
  t.true(parseExtJSON({input: '/* comment */\ntrue'}));
  t.true(parseExtJSON({input: '/* comment */ true'}));

  // comment at end
  t.true(parseExtJSON({input: 'true\n// comment'}));
  t.true(parseExtJSON({input: 'true\n/* comment */'}));
  t.true(parseExtJSON({input: 'true/* comment */'}));

  // comment before object property
  t.looksLike(parseExtJSON({input: '{/* comment */ "foo": "bar"}'}), {
    foo: 'bar',
  });
  t.looksLike(parseExtJSON({input: '{// comment\n"foo": "bar"}'}), {
    foo: 'bar',
  });

  // comment before object property value
  t.looksLike(parseExtJSON({input: '{"foo": /* comment */ "bar"}'}), {
    foo: 'bar',
  });
  t.looksLike(parseExtJSON({input: '{"foo": // comment\n"bar"}'}), {
    foo: 'bar',
  });

  // comment after object property value
  t.looksLike(parseExtJSON({input: '{"foo": "bar" /* comment */,}'}), {
    foo: 'bar',
  });
  t.looksLike(parseExtJSON({input: '{"foo": "bar" // comment\n,}'}), {
    foo: 'bar',
  });

  // comment after object property
  t.looksLike(parseExtJSON({input: '{"foo": "bar", /* comment */}'}), {
    foo: 'bar',
  });
  t.looksLike(parseExtJSON({input: '{"foo": "bar", // comment\n}'}), {
    foo: 'bar',
  });

  // comment before array element
  t.looksLike(parseExtJSON({input: '[/* comment */ "foo"]'}), ['foo']);
  t.looksLike(parseExtJSON({input: '[//comment\n"foo"]'}), ['foo']);

  // comment after array element
  t.looksLike(parseExtJSON({input: '["foo" /* comment */]'}), ['foo']);
  t.looksLike(parseExtJSON({input: '["foo" //comment\n]'}), ['foo']);

  // comment after array element value
  t.looksLike(parseExtJSON({input: '["foo" /* comment */, "bar"]'}), [
    'foo',
    'bar',
  ]);
  t.looksLike(parseExtJSON({input: '["foo" //comment\n, "bar"]'}), [
    'foo',
    'bar',
  ]);

  // comment only in array
  t.looksLike(parseExtJSON({input: '[/* comment */]'}), []);
  t.looksLike(parseExtJSON({input: '[// comment\n]'}), []);

  // comment only in object
  t.looksLike(parseExtJSON({input: '{/* comment */}'}), {});
  t.looksLike(parseExtJSON({input: '{// comment\n}'}), {});

  // ensure closed block comment
  t.throws(() => {
    parseExtJSON({input: 'true /* unclosed comment'});
  }, messages.UNCLOSED_BLOCK_COMMENT());
});

test('numbers', t => {
  t.is(parseExtJSON({input: '1'}), 1);
  t.is(parseExtJSON({input: '12'}), 12);
  t.is(parseExtJSON({input: '123'}), 123);
  t.is(parseExtJSON({input: '1.2'}), 1.2);
  t.is(parseExtJSON({input: '1234.21234'}), 1234.21234);
  t.is(parseExtJSON({input: '0.5e+5'}), 0.5e5);
  t.is(parseExtJSON({input: '0.5e-5'}), 0.5e-5);
  t.is(parseExtJSON({input: '0.5E+5'}), 0.5e5);
  t.is(parseExtJSON({input: '0.5E-5'}), 0.5e-5);
});

test('strings', t => {
  t.is(parseExtJSON({input: '"foo"'}), 'foo');
  t.is(parseExtJSON({input: '"foo\u1234"'}), 'foo\u1234');
  t.is(parseExtJSON({input: '"foo\\n"'}), 'foo\n');
  t.is(parseExtJSON({input: '"foo\\t"'}), 'foo\t');

  t.throws(() => {
    parseExtJSON({input: '"foo'});
  }, messages.UNCLOSED_STRING());

  t.throws(() => {
    parseExtJSON({input: '"foo\n"'});
  }, messages.UNCLOSED_STRING());

  t.throws(() => {
    parseExtJSON({input: "'foo'"});
  }, messages.SINGLE_QUOTE_USAGE());

  // TODO escMessage.INVALID_HEX_DIGIT_FOR_ESCAPE
  // TODO escMessage.INVALID_STRING_CHARACTER
  // TODO escMessage.NOT_ENOUGH_CODE_POINTS
});

test('booleans', t => {
  t.is(parseExtJSON({input: 'true'}), true);
  t.is(parseExtJSON({input: 'false'}), false);
});

test('null', t => {
  t.is(parseExtJSON({input: 'null'}), null);
});

test('undefined', t => {
  t.throws(() => {
    t.is(parseExtJSON({input: 'undefined'}), undefined);
  }, messages.UNDEFINED_IN_JSON());
});

test('arrays', t => {
  t.looksLike(parseExtJSON({input: '[]'}), []);
  t.looksLike(parseExtJSON({input: '[1, 2, 3]'}), [1, 2, 3]);
  t.looksLike(parseExtJSON({input: '[[1, 2, 3]]'}), [[1, 2, 3]]);

  t.throws(() => {
    parseExtJSON({input: '[,]'});
  }, messages.REDUNDANT_COMMA());

  t.throws(() => {
    parseExtJSON({input: '[1,,]'});
  }, messages.REDUNDANT_COMMA());

  t.throws(() => {
    parseExtJSON({input: '[1, /*comment*/,]'});
  }, messages.REDUNDANT_COMMA());

  t.throws(() => {
    parseExtJSON({input: '["foo": "bar"]'});
  }, messages.MISTAKEN_ARRAY_IDENTITY());
});

test('objects', t => {
  t.looksLike(parseExtJSON({input: '{}'}), {});
  t.looksLike(parseExtJSON({input: '{"foo": "bar"}'}), {foo: 'bar'});
  t.looksLike(parseExtJSON({input: '{"foo": "bar", "bar": "foo"}'}), {
    foo: 'bar',
    bar: 'foo',
  });

  t.throws(() => {
    parseExtJSON({input: '{,}'});
  }, messages.REDUNDANT_COMMA());

  t.throws(() => {
    parseExtJSON({input: '{"foo": "bar",,}'});
  }, messages.REDUNDANT_COMMA());

  t.throws(() => {
    parseExtJSON({input: '{"foo": "bar", /*comment*/,}'});
  }, messages.REDUNDANT_COMMA());
});

test('regular JSON', t => {
  t.throws(() => {
    parseJSON({input: '{foo: "bar"}'});
  }, messages.PROPERTY_KEY_UNQUOTED_IN_JSON());

  t.throws(() => {
    parseJSON({input: '// foobar\ntrue'});
  }, messages.COMMENTS_IN_JSON());

  t.throws(() => {
    parseJSON({input: '/* foobar */\ntrue'});
  }, messages.COMMENTS_IN_JSON());

  t.throws(() => {
    parseJSON({input: '{"foo": "bar",}'});
  }, messages.TRAILING_COMMA_IN_JSON());

  t.throws(() => {
    parseJSON({input: '["foo",]'});
  }, messages.TRAILING_COMMA_IN_JSON());
});
