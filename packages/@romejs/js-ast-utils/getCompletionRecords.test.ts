/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import getCompletionRecords from './getCompletionRecords';
import {test} from 'rome';
import {parseJS} from '@romejs/js-parser';
import {functionDeclaration} from '@romejs/js-ast';

function helper(input: string) {
  return getCompletionRecords(functionDeclaration.assert(parseJS({
    path: 'unknown',
    input: `function foo(){${input}}`,
  }).body[0]).body);
}

test('invalid', async (t) => {
  await t.snapshot(helper(`{}`));
  await t.snapshot(helper(`'foobar';`));
  await t.snapshot(helper(`if (bar) {'foobar';}`));
  await t.snapshot(helper(`if (bar) {'foobar';} else {}`));
  await t.snapshot(helper(`switch (foo) {}`));
  await t.snapshot(helper(`switch (foo) {case 'bar': {}}`));
  await t.snapshot(helper(`switch (foo) {default: {}}`));
});

test('completions', async (t) => {
  await t.snapshot(helper(`return false;`));
  await t.snapshot(helper(`return; invalid;`));
  await t.snapshot(helper(`if (bar) {return false;}`));
  await t.snapshot(helper(`if (bar) {return false;} else {return true;}`));
  await t.snapshot(helper(`switch (foo) {default: {return false;}}`));
  await t.snapshot(helper(`switch (foo) {default: {return false;}}`));
});

test(
  'mix',
  async (t) => {
    await t.snapshot(helper(
      `switch (foo) {default: {if (true) {return false;}}}`,
    ));
  },
);
