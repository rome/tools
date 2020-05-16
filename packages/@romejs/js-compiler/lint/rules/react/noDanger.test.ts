/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from 'rome';
import {testLintMultiple} from '../testHelpers';

test(
  'no danger',
  async (t) => {
    await testLintMultiple(
      t,
      [
        // INVALID
        '<div dangerouslySetInnerHTML={{ __html: "Hello World" }}></div>;',
        // VALID
        '<div>Hello World</div>;',
      ],
      {category: 'lint/noDanger'},
    );
  },
);
