/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import test from '@romejs/test';
import lint from '../api/lint';
import {parseJS} from '@romejs/js-parser';
import {createUnknownFilePath} from '@romejs/path';
import {DEFAULT_PROJECT_CONFIG, ProjectConfig} from '@romejs/project';

const LINT_ENABLED_FORMAT_DISABLED_CONFIG: ProjectConfig = {
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

const LINT_AND_FORMAT_ENABLED_CONFIG: ProjectConfig = {
  ...LINT_ENABLED_FORMAT_DISABLED_CONFIG,
  format: {
    ...LINT_ENABLED_FORMAT_DISABLED_CONFIG.format,
    enabled: true,
  },
};

async function testLint(input: string, config: ProjectConfig) {
  return await lint({
    options: {},
    ast: parseJS({
      input,
      sourceType: 'module',
      path: createUnknownFilePath('unknown'),
    }),
    sourceText: input,
    project: {
      folder: undefined,
      config,
    },
  });
}

test('empty file', async t => {
  t.snapshot(await testLint('', LINT_ENABLED_FORMAT_DISABLED_CONFIG));
});

test('undeclared variable', async t => {
  const res = await testLint('foobar;', LINT_ENABLED_FORMAT_DISABLED_CONFIG);
  t.snapshot(res);

  // Redundant because of the snapshot above, but this is what we actually care about
  t.looksLike(res.diagnostics, [
    {
      category: 'lint/undeclaredVariables',
      filename: 'unknown',
      language: 'js',
      message: 'Undeclared variable <emphasis>foobar</emphasis>',
      mtime: undefined,
      sourceType: 'module',
      origins: [{category: 'lint'}],
      end: {
        column: 6,
        index: 6,
        line: 1,
      },
      start: {
        column: 0,
        index: 0,
        line: 1,
      },
    },
  ]);
});

test('no async promise executor', async t => {
  const validTestCases = [
    'new Promise(() => {})',
    'new Promise(() => {}, async function unrelated() {})',
    'class Foo {} new Foo(async () => {})',
  ];
  const invalidTestCases = [
    'new Promise(async function foo() {})',
    'new Promise(async () => {})',
    'new Promise(((((async () => {})))))',
  ];
  for (const validTestCase of validTestCases) {
    const {diagnostics} = await testLint(
      validTestCase,
      LINT_ENABLED_FORMAT_DISABLED_CONFIG,
    );
    t.is(diagnostics.length, 0);
  }
  for (const invalidTestCase of invalidTestCases) {
    t.snapshot(
      await testLint(invalidTestCase, LINT_ENABLED_FORMAT_DISABLED_CONFIG),
    );
  }
});

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

test('no label var', async t => {
  const badLabel = await testLint(
    `
  const x = "test";
  x: const y = "test";
  `,
    LINT_ENABLED_FORMAT_DISABLED_CONFIG,
  );

  t.truthy(badLabel.diagnostics.find(d => d.category === 'lint/noLabelVar'));

  const okLabel = await testLint(
    `
  const x = "test";
  z: const y = "test";
  `,
    LINT_ENABLED_FORMAT_DISABLED_CONFIG,
  );

  t.falsy(okLabel.diagnostics.find(d => d.category === 'lint/noLabelVar'));
});
