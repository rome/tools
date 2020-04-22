/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from 'rome';
import {testLintMultiple} from '../../api/lint.test';

test('no inner declarations', async (t) => {
  await testLintMultiple(t, [
    `function outer() {
        if (true) {
            function inner() {}
        }
    }`,
    `if (true) {
        function inner() {}
    }`,
    `try {
        function inner() {}
    } catch (ex) {
        function innerInner() {}
    }`,
  ], {category: 'lint/noInnerDeclarations'});
});
