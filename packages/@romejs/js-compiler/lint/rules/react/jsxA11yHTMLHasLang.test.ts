/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from 'rome';

import {testLintMultiple} from '../testHelpers';

test(
  'require a lang attribute on <html> JSX elements',
  async (t) => {
    await testLintMultiple(
      t,
      [
        // INVALID
        '<html></html>',
        '<html {...props}></html>',
        '<html lang=""></html>',
        '<html lang={""}></html>',
        '<html lang={undefined}></html>',
        '<html lang={false}></html>',
        '<html lang={true}></html>',
        '<html lang={42}></html>',
        // VALID
        '<html lang="en"></html>',
        '<html lang={language}></html>',
      ],
      {category: 'lint/jsxA11yHTMLHasLang'},
    );
  },
);
