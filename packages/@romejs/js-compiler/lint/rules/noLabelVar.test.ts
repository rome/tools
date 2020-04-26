/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from 'rome';
import {testLint} from '../testHelpers';

test(
  'no label var',
  async (t) => {
    await testLint(t, `
      const x = "test";
      x: const y = "test";
      `, {category: 'lint/noLabelVar'});

    await testLint(t, `
      const x = "test";
      z: const y = "test";
      `, {category: 'lint/noLabelVar'});
  },
);
