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
import {parseDecisionStrings} from '@romejs/js-compiler';
import {Consumer} from '@romejs/consume';
import {commandCategories} from '@romejs/core/common/commands';

type Flags = {
  decisions: Array<string>;
  fix: boolean;
  changed: undefined | string;
  formatOnly: boolean;
};

export default createMasterCommand<Flags>({
  category: commandCategories.CODE_QUALITY,
  description: 'run lint against a set of files',
  allowRequestFlags: ['watch', 'review', 'allowDirty'],
  usage: '',
  examples: [],

  defineFlags(consumer: Consumer): Flags {
    return {
      decisions: consumer.get('decisions').asImplicitArray().map(
        (item) => item.asString(),
      ),
      fix: consumer.get('fix').asBoolean(false),
      formatOnly: consumer.get('formatOnly').asBoolean(false),
      changed: consumer.get('changed').asStringOrVoid(),
    };
  },

  async callback(req: MasterRequest, flags: Flags): Promise<void> {
    const {reporter} = req;

    if (req.query.requestFlags.review || flags.fix) {
      await req.assertCleanVSC();
    }

    let compilerOptionsPerFile: LinterOptions['compilerOptionsPerFile'] = {};
    const {decisions} = flags;
    if (decisions !== undefined) {
      compilerOptionsPerFile = parseDecisionStrings(
        decisions,
        req.client.flags.cwd,
        (description) => {
          throw req.throwDiagnosticFlagError({
            description,
            target: {type: 'flag', key: 'decisions'},
          });
        },
      );
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
      hasDecisions: flags.decisions.length > 0,
      compilerOptionsPerFile,
      fixLocation,
      formatOnly: flags.formatOnly,
      args,
    };

    const linter = new Linter(req, opts);
    await linter.run(req.query.requestFlags.watch);
  },
});
