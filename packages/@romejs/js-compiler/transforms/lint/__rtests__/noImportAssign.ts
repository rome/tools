/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {parseJS} from '@romejs/js-parser';
import {createUnknownFilePath} from '@romejs/path';
import test from '@romejs/test';
import {
  testLint,
  LINT_ENABLED_FORMAT_DISABLED_CONFIG,
} from '../../../__rtests__/lint';

test('no import assign', async t => {
  let failingCases = [
    'import x from "y";\nx=1;',
    'import x from "y";\n[x]=1;',
    'import x from "y";\n({x}=1);',
    'import x from "y";\nx++',
    'import x from "y";\n[...x]=1;',
    'import x from "y";\n({...x}=1);',
    'import x from "y";\nfor (x in y);',
    'import x from "y";\nx+=1',
    'import * as x from "y";\nx=1;',
    'import {x} from "y";\nx=1;',
  ];
  for (let failingCase of failingCases) {
    const res = await testLint(
      failingCase,
      LINT_ENABLED_FORMAT_DISABLED_CONFIG,
    );
    if (!res.diagnostics.some(d => d.category === 'lint/noImportAssign')) {
      t.fail(
        `expected "\n${failingCase}\n" to report a lint/noImportAssign diagnostic but it didn't`,
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
});
