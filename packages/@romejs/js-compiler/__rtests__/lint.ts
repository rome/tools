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
import {ConstSourceType} from '@romejs/js-ast';

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

async function testLint(
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

test('sparse array', async t => {
  const res = await testLint(`[1,,2]`, LINT_ENABLED_FORMAT_DISABLED_CONFIG);

  t.snapshot(res);
});

test('unsafe negation', async t => {
  const res = await testLint(
    `!1 in [1,2]`,
    LINT_ENABLED_FORMAT_DISABLED_CONFIG,
  );

  t.snapshot(res);
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

test('disallows comparing negative zero', async t => {
  const sourceTextA = '(1 >= -0)';

  const sourceTextB = '(1 >= 0)';

  const res1 = await testLint(sourceTextA, LINT_AND_FORMAT_ENABLED_CONFIG);
  t.looksLike(res1.diagnostics, [
    {
      category: 'lint/noCompareNegZero',
      filename: 'unknown',
      language: 'js',
      message: "Do not use the '>=' operator to compare against -0.",
      mtime: undefined,
      sourceType: 'module',
      origins: [{category: 'lint'}],
      end: {
        column: 8,
        index: 8,
        line: 1,
      },
      start: {
        column: 1,
        index: 1,
        line: 1,
      },
    },
  ]);

  const res2 = await testLint(sourceTextB, LINT_AND_FORMAT_ENABLED_CONFIG);
  t.looksLike(res2.diagnostics, []);
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

test('no duplicate keys', async t => {
  const res = await testLint(
    `
    const foo = {
      test: true,
      test2: true,
      test: false,
    }

    // mark const as used
    console.log(foo);
    `,
    LINT_ENABLED_FORMAT_DISABLED_CONFIG,
  );

  t.looksLike(res.diagnostics, [
    {
      category: 'lint/noDuplicateKeys',
      filename: 'unknown',
      language: 'js',
      message: 'Duplicate key <emphasis>test</emphasis>',
      mtime: undefined,
      sourceType: 'module',
      origins: [{category: 'lint'}],
      end: {
        column: 17,
        index: 73,
        line: 5,
      },
      start: {
        column: 6,
        index: 62,
        line: 5,
      },
    },
  ]);
});

test('disallow var', async t => {
  const res = await testLint(
    'var foobar;\nfoobar',
    LINT_ENABLED_FORMAT_DISABLED_CONFIG,
  );
  t.snapshot(res);

  // Redundant because of the snapshot above, but this is what we actually care about
  t.looksLike(res.diagnostics, [
    {
      category: 'lint/disallowVar',
      filename: 'unknown',
      language: 'js',
      message:
        'Variable declarations using `var` are disallowed, use `let` or `const` instead.',
      mtime: undefined,
      sourceType: 'module',
      origins: [{category: 'lint'}],
      end: {
        column: 11,
        index: 11,
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

test('disallow unsafe usage of break, continue, throw and return', async t => {
  const returnTest = await testLint(
    `
    function greet1() {
      try {
        throw new Error("Try")
      } catch(err) {
          throw err;
      } finally {
          return 1;
      }
    }
    
    greet1();
    `,
    LINT_ENABLED_FORMAT_DISABLED_CONFIG,
  );

  t.truthy(
    returnTest.diagnostics.find(
      d => d.message === `Unsafe usage of ReturnStatement.`,
    ),
  );

  const breakTest = await testLint(
    `
    
    function greet2() {
      try {
        throw new Error("Try")
      } catch(err) {
          throw err;
      } finally {
          break;
      }
    }

    greet2();
    `,
    LINT_ENABLED_FORMAT_DISABLED_CONFIG,
  );

  t.truthy(
    breakTest.diagnostics.find(
      d => d.message === `Unsafe usage of BreakStatement.`,
    ),
  );

  const continueTest = await testLint(
    `
    function greet3() {
      try {
        throw new Error("Try")
      } catch(err) {
          throw err;
      } finally {
          continue;
      }
    }
    
    greet3();
    `,
    LINT_ENABLED_FORMAT_DISABLED_CONFIG,
  );

  t.truthy(
    continueTest.diagnostics.find(
      d => d.message === `Unsafe usage of ContinueStatement.`,
    ),
  );

  const throwTest = await testLint(
    `
    function greet4() {
      try {
        throw new Error("Try")
      } catch(err) {
          throw err;
      } finally {
          throw new Error("Finally");
      }
    }

    greet4();
    `,
    LINT_ENABLED_FORMAT_DISABLED_CONFIG,
  );

  t.truthy(
    throwTest.diagnostics.find(
      d => d.message === `Unsafe usage of ThrowStatement.`,
    ),
  );
});

test('no delete vars', async t => {
  const res = await testLint(
    `
    const foo = "test";
    delete foo;
    `,
    LINT_ENABLED_FORMAT_DISABLED_CONFIG,
    'script',
  );

  t.looksLike(res.diagnostics, [
    {
      category: 'lint/noDeleteVars',
      message: 'Variables should not be deleted.',
      mtime: undefined,
      filename: 'unknown',
      start: {index: 29, line: 3, column: 4},
      end: {index: 39, line: 3, column: 14},
      language: 'js',
      sourceType: 'script',
      origins: [{category: 'lint'}],
    },
  ]);
});

test('no template curly in string', async t => {
  const res = await testLint(
    `
    const user = "Faustina";
    const helloUser = "Hello, \${user}!";

    // mark consts as used
    console.log(user, helloUser)
    `,
    LINT_ENABLED_FORMAT_DISABLED_CONFIG,
  );

  t.looksLike(res.diagnostics, [
    {
      category: 'lint/noTemplateCurlyInString',
      filename: 'unknown',
      language: 'js',
      message: 'Unexpected template string expression.',
      mtime: undefined,
      sourceType: 'module',
      origins: [{category: 'lint'}],
      end: {column: 39, index: 69, line: 3},
      start: {column: 22, index: 52, line: 3},
    },
  ]);
});
