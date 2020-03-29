/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {commandCategories, createMasterCommand} from '../../commands';
import {MasterRequest} from '@romejs/core';
import test from './test';
import lint from './lint';

export default createMasterCommand({
  category: commandCategories.CODE_QUALITY,
  description: 'run lint and tests',

  async default(req: MasterRequest): Promise<void> {
    const {reporter} = req;

    reporter.heading('Running lint');
    await lint.default(req, {
      fix: false,
      changed: undefined,
    });

    reporter.heading('Running tests');
    await test.default(req, {
      coverage: true,
      freezeSnapshots: true,
      updateSnapshots: false,
      showAllCoverage: true,
      syncTests: false,
    });
  },
});
