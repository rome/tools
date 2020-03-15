/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {toCamelCase} from './toCamelCase';
import test from '@romejs/test';

test('toCamelCase', t => {
  const testCases = [
    {input: 'rometest', expected: 'rometest'},
    {input: 'rome test', expected: 'romeTest'},
    {input: 'RoMe TeSt', expected: 'RoMeTeSt'},
    {input: 'ROME TEST', expected: 'ROMETEST'},
  ];

  testCases.forEach(td => {
    t.is(toCamelCase(td.input), td.expected);
  });
});
