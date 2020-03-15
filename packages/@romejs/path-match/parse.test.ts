/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import '@romejs/string-markup';
import {parsePathPattern, PathPattern} from '@romejs/path-match';
import test from '@romejs/test';

function _parsePathPattern(input: string): PathPattern {
  return parsePathPattern({input});
}

test('pattern', t => {
  // Negate and wildcard
  t.snapshot(_parsePathPattern('!foo'));
  t.snapshot(_parsePathPattern(''));

  // Trailing slash and wildcards
  t.snapshot(_parsePathPattern('/foo/bar'));
  t.snapshot(_parsePathPattern('*/foo/bar'));
  t.snapshot(_parsePathPattern('**/foo/bar'));
  t.snapshot(_parsePathPattern('**/*foo/bar'));

  // Random
  t.snapshot(_parsePathPattern('foo'));
  t.snapshot(_parsePathPattern('foo/'));
  t.snapshot(_parsePathPattern('foo/bar'));
  t.snapshot(_parsePathPattern('foo//bar'));
  t.snapshot(_parsePathPattern('foo/*/bar'));
  t.snapshot(_parsePathPattern('foo/**/bar'));
  t.snapshot(_parsePathPattern('foo/*bar'));
  t.snapshot(_parsePathPattern('foo/bar*'));
  t.snapshot(_parsePathPattern('foo/*bar*'));
  t.snapshot(_parsePathPattern('foo/*bar*foob'));

  // Comments
  t.snapshot(_parsePathPattern('# foobar'));
  t.snapshot(_parsePathPattern('foo/bar # foobar'));
  t.snapshot(_parsePathPattern('foo/bar\\#foobar'));
  t.snapshot(_parsePathPattern('foo/\\#foobar'));

  // Windows separators
  t.snapshot(_parsePathPattern('\\\\foo\\\\bar'));
  t.snapshot(_parsePathPattern('*\\\\foo\\\\bar'));
  t.snapshot(_parsePathPattern('**\\\\foo\\\\bar'));
  t.snapshot(_parsePathPattern('**\\\\*foo\\\\bar'));
  t.snapshot(_parsePathPattern('hello\\\\world'));
});
