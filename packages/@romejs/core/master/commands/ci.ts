/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {commandCategories} from '../../commands';
import {createMasterCommand} from '../../commands';
import {MasterRequest} from '@romejs/core';
import Linter from '../linter/Linter';
import test from './test';

export default createMasterCommand({
  category: commandCategories.CODE_QUALITY,
  description: 'run lint and tests',

  async default(req: MasterRequest): Promise<void> {
    const {reporter} = req;

    reporter.heading('Running lint');
    const linter = new Linter(req);
    await linter.lint(false);

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
