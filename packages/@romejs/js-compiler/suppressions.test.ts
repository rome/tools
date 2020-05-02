/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from 'rome';
import {extractSuppressionsFromProgram} from './suppressions';
import CompilerContext from './lib/CompilerContext';
import {parseJS} from '@romejs/js-parser';
import {dedent} from '@romejs/string-utils';

function extractSuppressionsFromSource(sourceText: string) {
  const ast = parseJS({
    sourceType: 'script',
    path: 'unknown',
    input: sourceText,
  });
  const context = new CompilerContext({ast});
  return extractSuppressionsFromProgram(context, ast);
}

test(
  'single category',
  async (t) => {
    t.snapshot(
      extractSuppressionsFromSource(
        dedent`
      // rome-ignore foo
      foo();

      /** rome-ignore bar */
      bar();

      /**
       * rome-ignore yes
       */
      yes();

      /**
       * hello
       * rome-ignore wow
       */
      wow();
    `,
      ),
    );
  },
);

test(
  'multiple categories',
  async (t) => {
    t.snapshot(
      extractSuppressionsFromSource(
        dedent`
      // rome-ignore foo dog
      foo();

      /** rome-ignore bar cat */
      bar();

      /**
       * rome-ignore yes frog
       */
      yes();

      /**
       * hello
       * rome-ignore wow fish
       */
      wow();
    `,
      ),
    );
  },
);

test(
  'duplicates',
  async (t) => {
    t.snapshot(
      extractSuppressionsFromSource(
        dedent`
      // rome-ignore dog dog
      foo();

      // rome-ignore dog cat dog
      bar();
    `,
      ),
    );
  },
);
