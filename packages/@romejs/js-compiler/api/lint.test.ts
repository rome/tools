/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

// TODO: Shift file into packages/@romejs/js-compiler/transforms/lint/__rtests__
// after all open linting PRs have been merged.

import test from '@romejs/test';
import lint from './lint';
import {parseJS} from '@romejs/js-parser';
import {createUnknownFilePath} from '@romejs/path';
import {DEFAULT_PROJECT_CONFIG} from '@romejs/project';
import {ConstSourceType} from '@romejs/js-ast';

export async function testLint(
  input: string,
  format: boolean = false,
  sourceType: ConstSourceType = 'module',
) {
  return await lint({
    options: {},
    format,
    ast: parseJS({
      input,
      sourceType,
      path: createUnknownFilePath('unknown'),
    }),
    sourceText: input,
    project: {
      folder: undefined,
      config: DEFAULT_PROJECT_CONFIG,
    },
  });
}

test('format disabled in project config should not regenerate the file', async t => {
  // Intentionally weird formatting
  const sourceText = 'foobar ( "yes" );';
  const res = await testLint(sourceText, false);
  t.is(res.src, sourceText);
});

test('format enabled in project config should result in regenerated file', async t => {
  const res = await testLint('foobar ( "yes" );', true);
  t.is(res.src, "foobar('yes');\n");
});
