/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {parseJS} from '@romejs/js-parser';
import {createUnknownFilePath} from '@romejs/path';
import test from '@romejs/test';
import {testLint} from '../../api/lint.test';

test(
  'no shadow restricted names',
  async (t) => {
    let failingCases = [
      'function NaN() {}',
      'let Set;',
      '!function Array() {}',
      'function test(JSON) {}',
      'try {  } catch(Object) {}',
    ];
    for (let failingCase of failingCases) {
      const res = await testLint(failingCase);
      if (!res.diagnostics.some((d) => d.description.category ===
          'lint/noShadowRestrictedNames')) {
        t.fail(
          `expected "\n${failingCase}\n" to report a lint/noShadowRestrictedNames diagnostic but it didn't`,
          [
            {
              type: 'inspect',
              data: parseJS({
                input: failingCase,
                sourceType: 'module',
                path: createUnknownFilePath('unknown'),
              }),
            },
            {type: 'inspect', data: res.diagnostics},
          ],
        );
      }
    }
  },
);
