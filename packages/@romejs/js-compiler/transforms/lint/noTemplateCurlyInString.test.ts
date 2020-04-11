/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import test from '@romejs/test';
import {testLint} from '../../api/lint.test';

test('no template curly in string', async (t) => {
  await testLint(t, `
        const user = "Faustina";
        const helloUser = "Hello, \${user}!";

        // mark consts as used
        console.log(user, helloUser)
      `, {category: 'lint/noTemplateCurlyInString'});
});
