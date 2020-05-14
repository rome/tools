/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {MasterRequest} from '@romejs/core';
import Linter, {
  LinterCompilerOptionsPerFile,
  LinterOptions,
} from '../linter/Linter';
import {markup} from '@romejs/string-markup';
import {createMasterCommand} from '../commands';
import {
  LintCompilerOptionsDecisions,
  parseDecisionStrings,
} from '@romejs/js-compiler';
import {Consumer} from '@romejs/consume';
import {commandCategories} from '@romejs/core/common/commands';
import {createUnknownFilePath} from '@romejs/path';
import {LINTABLE_EXTENSIONS} from '@romejs/core/common/file-handlers';

type Flags = {
  decisions: Array<string>;
  save: boolean;
  changed: undefined | string;
  formatOnly: boolean;
};

export default createMasterCommand<Flags>({
  category: commandCategories.CODE_QUALITY,
  description: 'run lint against a set of files',
  allowRequestFlags: ['watch', 'review'],
  usage: '',
  examples: [],
  defineFlags(consumer: Consumer): Flags {
    return {
      decisions: consumer.get('decisions').asImplicitArray().map((item) =>
        item.asString()
      ),
      save: consumer.get('save').asBoolean(false),
      formatOnly: consumer.get('formatOnly').asBoolean(false),
      changed: consumer.get('changed').asStringOrVoid(),
    };
  },
  async callback(req: MasterRequest, flags: Flags): Promise<void> {
    const {reporter} = req;

    let lintCompilerOptionsPerFile: LinterCompilerOptionsPerFile = {};
    let globalDecisions: LintCompilerOptionsDecisions = [];
    const {decisions} = flags;
    if (decisions !== undefined) {
      ({lintCompilerOptionsPerFile, globalDecisions} = parseDecisionStrings(
        decisions,
        req.client.flags.cwd,
        (description) => {
          throw req.throwDiagnosticFlagError({
            description,
            target: {type: 'flag', key: 'decisions'},
          });
        },
      ));
    }

    // Look up arguments manually in vsc if we were passed a changes branch
    let args;
    if (flags.changed !== undefined) {
      // No arguments expected when using this flag
      req.expectArgumentLength(0);

      const client = await req.getVCSClient();
      const target = flags.changed === '' ? client.trunkBranch : flags.changed;
      args = await client.getModifiedFiles(target);

      // Only include lintable files
      args = args.filter((arg) => {
        const path = createUnknownFilePath(arg);

        for (const ext of LINTABLE_EXTENSIONS) {
          if (path.hasEndExtension(ext)) {
            return true;
          }
        }

        return false;
      });

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
      lintCompilerOptionsPerFile,
      globalDecisions,
      save: flags.save,
      formatOnly: flags.formatOnly,
      args,
    };

    const linter = new Linter(req, opts);
    await linter.run(req.query.requestFlags.watch);
  },
});
