/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {PartialDiagnostics} from '@romejs/diagnostics';
import {DEFAULT_PROJECT_CONFIG} from '@romejs/project';
import test from '@romejs/test';
import {check} from '@romejs/js-analysis';
import {parseJS} from '@romejs/js-parser';
import {createUnknownFilePath} from '@romejs/path';

async function testCheck(code: string): Promise<PartialDiagnostics> {
  const ast = parseJS({
    input: code,
    sourceType: 'module',
    path: createUnknownFilePath('unknown'),
  });

  return check({
    ast,
    project: {
      folder: undefined,
      config: DEFAULT_PROJECT_CONFIG,
    },
    provider: {
      getExportTypes() {
        return Promise.reject('unsupported');
      },
    },
  });
}

test('discovers require(\'module\') call', async () => {
  testCheck;

  /*const diagnostics = await testCheck(`
    const a: number = '';
  `);

  console.log(diagnostics);*/
});
