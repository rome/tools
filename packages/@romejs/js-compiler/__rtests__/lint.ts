/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

// TODO: Shift file into packages/@romejs/js-compiler/transforms/lint/__rtests__
// after all open linting PRs have been merged.

import test from '@romejs/test';
import lint from '../api/lint';
import {parseJS} from '@romejs/js-parser';
import {createUnknownFilePath} from '@romejs/path';
import {DEFAULT_PROJECT_CONFIG, ProjectConfig} from '@romejs/project';
import {ConstSourceType} from '@romejs/js-ast';

export const LINT_ENABLED_FORMAT_DISABLED_CONFIG: ProjectConfig = {
  ...DEFAULT_PROJECT_CONFIG,
  lint: {
    ...DEFAULT_PROJECT_CONFIG.lint,
    enabled: true,
  },
  format: {
    ...DEFAULT_PROJECT_CONFIG.format,
    enabled: false,
  },
};

export const LINT_AND_FORMAT_ENABLED_CONFIG: ProjectConfig = {
  ...LINT_ENABLED_FORMAT_DISABLED_CONFIG,
  format: {
    ...LINT_ENABLED_FORMAT_DISABLED_CONFIG.format,
    enabled: true,
  },
};

export async function testLint(
  input: string,
  config: ProjectConfig,
  sourceType: ConstSourceType = 'module',
) {
  return await lint({
    options: {},
    ast: parseJS({
      input,
      sourceType,
      path: createUnknownFilePath('unknown'),
    }),
    sourceText: input,
    project: {
      folder: undefined,
      config,
    },
  });
}

test('format disabled in project config should not regenerate the file', async t => {
  // Intentionally weird formatting
  const sourceText = 'foobar ( "yes" );';
  const res = await testLint(sourceText, LINT_ENABLED_FORMAT_DISABLED_CONFIG);
  t.is(res.src, sourceText);
});

test('format enabled in project config should result in regenerated file', async t => {
  const res = await testLint(
    'foobar ( "yes" );',
    LINT_AND_FORMAT_ENABLED_CONFIG,
  );
  t.is(res.src, "foobar('yes');\n");
});
