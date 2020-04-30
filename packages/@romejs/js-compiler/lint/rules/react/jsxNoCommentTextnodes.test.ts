/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from 'rome';
import {testLintMultiple} from '../../testHelpers';

test(
  'case single statement',
  async (t) => {
    await testLintMultiple(
      t,
      [
        // INVALID
        'const a = <div>// comment</div>',
        'const a = <div>/* comment */</div>',
        // VALID
        'const a = <div>{/* comment */}</div>',
        'const a = <div className={"cls" /* comment */}></div>',
      ],
      {category: 'lint/react/jsxNoCommentTextnodes'},
    );
  },
);
