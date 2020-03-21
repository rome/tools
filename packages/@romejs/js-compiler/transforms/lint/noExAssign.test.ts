/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import test from '@romejs/test';
import {testLint} from '../../api/lint.test';

test('no exception assign', async (t) => {
  const validTestCases = [
    `try { } catch (e) { three = 2 + 1; }`,
    'try { } catch ({e}) { this.something = 2; }',
    'function foo() { try { } catch (e) { return false; } }',
  ];

  const invalidTestCases = [
    'try { } catch (e) { e; e = 10; }',
    'try { } catch (ex) { console.log(\'test\'); ex = 10; }',
    'try { } catch (ex) { [ex, test] = []; }',
    'try { } catch ({message, name}) { message = \'test\'; name = 10; }',
    // "try { } catch (ex) { ({x: ex = 0} = {}); }",

    // "try { } catch (ex) { let a; ({x: a = ex = 0} = {}); }"
  ];

  for (const validTestCase of validTestCases) {
    const {diagnostics} = await testLint(validTestCase);

    t.falsy(diagnostics.find((d) => d.category === 'lint/noExAssign'));
  }

  for (const invalidTestCase of invalidTestCases) {
    let {diagnostics} = await testLint(invalidTestCase);

    t.truthy(diagnostics.find((d) => d.category === 'lint/noExAssign'));
  }
});
