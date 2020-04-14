/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import analyzeDependencies from './analyzeDependencies';
import {DEFAULT_PROJECT_CONFIG} from '@romejs/project';
import {test} from 'rome';
import {parseJS} from '@romejs/js-parser';
import {ConstSourceType} from '@romejs/js-ast';
import {createUnknownFilePath} from '@romejs/path';

async function testAnalyzeDeps(input: string, sourceType: ConstSourceType) {
  return await analyzeDependencies({
    options: {},
    ast: parseJS({input, sourceType, path: createUnknownFilePath('unknown')}),
    sourceText: input,
    project: {
      folder: undefined,
      config: DEFAULT_PROJECT_CONFIG,
    },
  });
}

test('ignores require() call if shadowed', async (t) => {
  t.snapshot(await testAnalyzeDeps(`
      {
        function require() {}
        require('yes');
      }

      function yes() {
        function require() {}
        require('yes');
      }
    `, 'script'));
});
