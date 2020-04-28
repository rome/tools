/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from 'rome';
import {testLint} from '../testHelpers';

test(
  'prefer template',
  async (t) => {
    await testLint(
      t,
      `console.log(1 + 'foo')`,
      {
        category: 'lint/preferTemplate',
      },
    );

    await testLint(
      t,
      `console.log((1 * 2) + 'foo')`,
      {
        category: 'lint/preferTemplate',
      },
    );

    await testLint(
      t,
      `console.log(1 + 'foo' + 2 + 'bar' + 'baz' + 3)`,
      {
        category: 'lint/preferTemplate',
      },
    );

    await testLint(
      t,
      `console.log((1 + 'foo') * 2)`,
      {
        category: 'lint/preferTemplate',
      },
    );

    await testLint(
      t,
      `console.log((1 * (2 + 'foo')) + 'bar')`,
      {
        category: 'lint/preferTemplate',
      },
    );

    await testLint(
      t,
      `console.log('foo' + 'bar')`,
      {
        category: 'lint/preferTemplate',
      },
    );

    await testLint(
      t,
      `console.log(\`foo\` + 1)`,
      {
        category: 'lint/preferTemplate',
      },
    );

    await testLint(
      t,
      `console.log('foo' + \`bar\${\`baz\${'bat' + 'bam'}\`}\` + 'boo')`,
      {
        category: 'lint/preferTemplate',
      },
    );

    await testLint(
      t,
      `console.log('foo' + 1 + 2)`,
      {
        category: 'lint/preferTemplate',
      },
    );
  },
);
