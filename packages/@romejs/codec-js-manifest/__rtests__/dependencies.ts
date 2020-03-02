/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import test from '@romejs/test';
import {parseDependencyPattern} from '../dependencies';
import {consumeUnknown} from '@romejs/consume';

test('can parse npm dependency patterns', t => {
  t.snapshot(parseDependencyPattern(consumeUnknown('npm:foo'), false));
  t.snapshot(parseDependencyPattern(consumeUnknown('npm:@foo/bar'), false));
  t.snapshot(parseDependencyPattern(consumeUnknown('npm:foo@1.0.0'), false));
  t.snapshot(
    parseDependencyPattern(consumeUnknown('npm:@foo/bar@1.0.0'), false),
  );
});
