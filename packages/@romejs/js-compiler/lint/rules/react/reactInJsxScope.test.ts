/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from 'rome';
import {testLint} from '../testHelpers';

test(
  '"React" in scope when using JSX',
  async (t) => {
    await testLint(
      t,
      `export function HelloWorldComponent() {
                return <div>
                    Hello World!!!
                </div>;
            }`,
      {
        category: 'lint/reactInJsxScope',
      },
    );
  },
);
