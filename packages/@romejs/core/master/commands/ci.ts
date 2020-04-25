/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {DiagnosticsPrinter} from '@romejs/cli-diagnostics';
import {MasterRequest} from '@romejs/core';
import {commandCategories} from '../../common/commands';
import {createMasterCommand} from '../commands';
import lint from './lint';
import test from './test';
import {Consumer} from '@romejs/consume';

async function runChildCommand(
  req: MasterRequest,
  fn: () => Promise<void>,
): Promise<void> {
  try {
    await fn();
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

type Flags = {
  fix: boolean;
};

export default createMasterCommand({
  category: commandCategories.CODE_QUALITY,
  description: 'run lint and tests',
  usage: '',
  examples: [],
  allowRequestFlags: ['allowDirty'],

  defineFlags(consumer: Consumer): Flags {
    return {
      fix: consumer.get('fix').asBoolean(false),
    };
  },

  async callback(req: MasterRequest, flags: Flags): Promise<void> {
    const {reporter} = req;

    if (flags.fix) {
      await req.assertCleanVSC();
    }

    reporter.heading('Running lint');
    await runChildCommand(req, async () => {
      await lint.callback(req, {
        formatOnly: false,
        decisions: [],
        save: flags.fix,
        changed: undefined,
      });
    });

    reporter.heading('Running tests');
    await runChildCommand(req, async () => {
      await test.callback(req, {
        coverage: false,
        freezeSnapshots: !flags.fix,
        updateSnapshots: flags.fix,
        showAllCoverage: false,
        syncTests: false,
      });
    });
  },
});
