/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {MasterRequest} from '@romejs/core';
import {createMasterCommand, commandCategories} from '../../commands';
import Linter, {LinterOptions} from '../linter/Linter';

import {Consumer} from '@romejs/consume';

type Flags = {
  fix: boolean;
  changed: undefined | string;
};

export default createMasterCommand<Flags>({
  category: commandCategories.CODE_QUALITY,
  description: 'run lint against a set of files',

  defineFlags(consumer: Consumer): Flags {
    return {
      fix: consumer.get('fix').asBoolean(false),
      changed: consumer.get('changed').asStringOrVoid(),
    };
  },

  async default(req: MasterRequest, flags: Flags): Promise<void> {
    const {reporter} = req;

    const fixLocation = flags.fix === false
      ? undefined
      : req.getDiagnosticPointerFromFlags({
        type: 'flag',
        key: 'fix',
      });

    // Look up arguments manually in vsc if we were passed a changes branch
    let args;
    if (flags.changed !== undefined) {
      // No arguments expected when using this flag
      req.expectArgumentLength(0);

      const client = await req.master.projectManager.getVCSClient(
        await req.assertClientCwdProject(),
      );
      const target = flags.changed === '' ? client.trunkBranch : flags.changed;
      args = await client.getModifiedFiles(target);

      if (args.length === 0) {
        reporter.warn(`No files changed from <emphasis>${target}</emphasis>`);
      } else {
        reporter.info(`Files changed from <emphasis>${target}</emphasis>`);
        reporter.list(args.map((arg) => `<filelink target="${arg}" />`));
        reporter.hr();
      }
    }

    const opts: LinterOptions = {
      fixLocation,
      args,
    };

    const linter = new Linter(req, opts);
    await linter.run(req.query.requestFlags.watch);
  },
});
