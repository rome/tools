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

test("discovers require('module') call", async (t) => {
  t.snapshot(await testAnalyzeDeps(`
      import * as foo from 'foo';

      function yeah() {
        require('bar');
      }
    `, 'module'));
});

test('ignores require(dynamic) call', async (t) => {
  t.snapshot(await testAnalyzeDeps(`
      require(dynamic);
    `, 'module'));
});

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

test("discovers async import('foo')", async (t) => {
  t.snapshot(await testAnalyzeDeps(`
      import('./foo');

      function yes() {
        import('./bar');
      }
    `, 'module'));
});

test('discovers local export specifiers', async (t) => {
  t.snapshot(await testAnalyzeDeps(`
      export {foo, bar, yes as no};
    `, 'module'));
});

test('discovers export declarations', async (t) => {
  t.snapshot(await testAnalyzeDeps(`
      export const yes = '';
      export function foo() {}
      export class Bar {}
    `, 'module'));
});

test('discovers export default', async (t) => {
  t.snapshot(await testAnalyzeDeps(`
      export default 'yes';
    `, 'module'));
});

test('discovers export from', async (t) => {
  t.snapshot(await testAnalyzeDeps(`
      export {foo, bar, default as no, boo as noo} from 'foobar';
    `, 'module'));
});

test('discovers export star', async (t) => {
  t.snapshot(await testAnalyzeDeps(`
      export * from 'foobar';
    `, 'module'));
});

test('discovers import star', async (t) => {
  t.snapshot(await testAnalyzeDeps(`
      import * as bar from 'foobar';
    `, 'module'));
});

test('discovers import specifiers', async (t) => {
  t.snapshot(await testAnalyzeDeps(`
      import {bar, foo, default as lol, ya as to} from 'foobar';
    `, 'module'));
});

test('discovers import default', async (t) => {
  t.snapshot(await testAnalyzeDeps(`
      import bar from 'foobar';
    `, 'module'));
});

test('discovers commonjs exports', async (t) => {
  t.snapshot(await testAnalyzeDeps(`
      exports.yes = function() {};
    `, 'script'));
});

test('discovers commonjs module.exports', async (t) => {
  t.snapshot(await testAnalyzeDeps(`
      module.exports = function() {};
    `, 'script'));
});

test('discovers top level await', async (t) => {
  t.snapshot(await testAnalyzeDeps(`
      await foobar();
    `, 'module'));
});

test('correctly identifies a file with es imports as es', async (t) => {
  t.snapshot(await testAnalyzeDeps(`
      import 'bar';
    `, 'module'));
});

test('correctly identifies a file with es exports as es', async (t) => {
  t.snapshot(await testAnalyzeDeps(`
      export const foo = 'bar';
    `, 'module'));
});

test('correctly identifies a file with cjs exports as cjs', async (t) => {
  t.snapshot(await testAnalyzeDeps(`
      exports.foo = 'bar';
    `, 'module'));
});

test(
  'correctly identifies a file with no imports or exports as unknown',
  async (
    t,
  ) => {
    t.snapshot(await testAnalyzeDeps(`
      foo();
    `, 'module'));
  },
);

test('disallow mix of es and cjs exports', async (t) => {
  t.snapshot(await testAnalyzeDeps(`
      export const foo = 'bar';
      exports.bar = 'foo';
    `, 'script'));
});

test('defines topLevelLocalBindings', async (t) => {
  t.snapshot(await testAnalyzeDeps(`
    import {bar} from 'foo';
    const foo = 'bar';
  `, 'module'));
});
