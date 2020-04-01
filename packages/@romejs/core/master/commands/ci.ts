/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {DiagnosticsPrinter} from '@romejs/cli-diagnostics';
import {MasterRequest} from '@romejs/core';
import {commandCategories, createMasterCommand} from '../../commands';
import lint from './lint';
import test from './test';

async function runChildCommand(
  req: MasterRequest,
  fn: () => Promise<void>,
): Promise<void> {
  try {
    await fn();
    throw new Error('Expected a printer');
  } catch (err) {
    if (err instanceof DiagnosticsPrinter) {
      // If the command raises diagnostics, it is safe to throw the printer.
      // By doing so, the `ci` command bails and is marked as failed.
      if (err.hasDiagnostics()) {
        throw err;
      } else {
        req.master.handleRequestError(req, err);
      }
    } else {
      throw err;
    }
  }
}

export default createMasterCommand({
  category: commandCategories.CODE_QUALITY,
  description: 'run lint and tests',

  async default(req: MasterRequest): Promise<void> {
    const {reporter} = req;

    reporter.heading('Running lint');
    await runChildCommand(req, async () => {
      await lint.default(req, {
        fix: false,
        changed: undefined,
      });
    });

    reporter.heading('Running tests');
    await runChildCommand(req, async () => {
      await test.default(req, {
        coverage: true,
        freezeSnapshots: true,
        updateSnapshots: false,
        showAllCoverage: true,
        syncTests: false,
      });
    });
  },
});
