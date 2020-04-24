/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import '@romejs/string-markup';
import {PathPattern, parsePathPattern} from '@romejs/path-match';
import {test} from 'rome';

function _parsePathPattern(input: string): PathPattern {
  return parsePathPattern({input});
}

test('pattern', async (t) => {
  // Negate and wildcard
  await t.snapshot(_parsePathPattern('!foo'));
  await t.snapshot(_parsePathPattern(''));

  // Trailing slash and wildcards
  await t.snapshot(_parsePathPattern('/foo/bar'));
  await t.snapshot(_parsePathPattern('*/foo/bar'));
  await t.snapshot(_parsePathPattern('**/foo/bar'));
  await t.snapshot(_parsePathPattern('**/*foo/bar'));

  // Random
  await t.snapshot(_parsePathPattern('foo'));
  await t.snapshot(_parsePathPattern('foo/'));
  await t.snapshot(_parsePathPattern('foo/bar'));
  await t.snapshot(_parsePathPattern('foo//bar'));
  await t.snapshot(_parsePathPattern('foo/*/bar'));
  await t.snapshot(_parsePathPattern('foo/**/bar'));
  await t.snapshot(_parsePathPattern('foo/*bar'));
  await t.snapshot(_parsePathPattern('foo/bar*'));
  await t.snapshot(_parsePathPattern('foo/*bar*'));
  await t.snapshot(_parsePathPattern('foo/*bar*foob'));

  // Comments
  await t.snapshot(_parsePathPattern('# foobar'));
  await t.snapshot(_parsePathPattern('foo/bar # foobar'));
  await t.snapshot(_parsePathPattern('foo/bar\\#foobar'));
  await t.snapshot(_parsePathPattern('foo/\\#foobar'));

  // Windows separators
  await t.snapshot(_parsePathPattern('\\\\foo\\\\bar'));
  await t.snapshot(_parsePathPattern('*\\\\foo\\\\bar'));
  await t.snapshot(_parsePathPattern('**\\\\foo\\\\bar'));
  await t.snapshot(_parsePathPattern('**\\\\*foo\\\\bar'));
  await t.snapshot(_parsePathPattern('hello\\\\world'));
});
