/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {MasterRequest} from '@romejs/core';
import Linter, {LinterOptions} from '../linter/Linter';
import {markup} from '@romejs/string-markup';
import {createMasterCommand} from '../commands';
import lintCommand, {LintCommandFlags} from '../../client/commands/lint';

export default createMasterCommand<LintCommandFlags>({
  ...lintCommand,

  async callback(req: MasterRequest, flags: LintCommandFlags): Promise<void> {
    const {reporter} = req;

    if (req.query.requestFlags.review || flags.fix) {
      await req.assertCleanVSC();
    }

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

      const client = await req.getVCSClient();
      const target = flags.changed === '' ? client.trunkBranch : flags.changed;
      args = await client.getModifiedFiles(target);

      if (args.length === 0) {
        reporter.warn(`No files changed from <emphasis>${target}</emphasis>`);
      } else {
        reporter.info(`Files changed from <emphasis>${target}</emphasis>`);
        reporter.list(args.map((arg) => markup`<filelink target="${arg}" />`));
        reporter.hr();
      }
    }

    const opts: LinterOptions = {
      compilerOptionsPerFile: flags.compilerOptionsPerFile,
      fixLocation,
      args,
    };

    const linter = new Linter(req, opts);
    await linter.run(req.query.requestFlags.watch);
  },
});
