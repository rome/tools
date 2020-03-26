/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {MasterRequest} from '@romejs/core';
import {createMasterCommand} from '../../commands';
import Linter, {LinterOptions} from '../linter/Linter';
import {commandCategories} from '../../commands';
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
      ? undefined : req.getDiagnosticPointerFromFlags({
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

    return new Promise((resolve, reject) => {
      if (req.query.requestFlags.watch) {
        initWatchLint(req, opts, reject);
      } else {
        resolve(runLint(req, opts));
      }
    });
  },
});

function initWatchLint(
  req: MasterRequest,
  opts: LinterOptions,
  reject: (err: Error) => void,
) {
  const {master, reporter} = req;

  // whenever a file change happens, we wait 250ms to do lint, this is in case there's multiple

  // files being linted, like if an autofix is triggered
  let queued = false;

  // whether or not we're currently linting
  let running = false;

  // if a file event happens while we're linting then we'll need to run the full lint again to make

  // sure it's up to date
  let runAgainAfterComplete = false;

  function runWatchLint() {
    if (running) {
      runAgainAfterComplete = true;
      return undefined;
    }

    queued = false;
    running = true;
    reporter.clear();

    runLint(req, opts).then(() => {
      running = false;

      if (runAgainAfterComplete) {
        runAgainAfterComplete = false;
        runWatchLint();
      }
    }, reject);
  }

  const listener = master.fileChangeEvent.subscribe(() => {
    if (running) {
      // queue up a lint to happen afterwards
      runWatchLint();
      return undefined;
    }

    if (queued) {
      // already have a timer waiting
      return undefined;
    }

    // queue up a lint
    queued = true;
    setTimeout(runWatchLint, 250);
  });

  req.endEvent.subscribe(() => {
    listener.unsubscribe();
  });

  runWatchLint();
}

async function runLint(req: MasterRequest, opts: LinterOptions): Promise<void> {
  const linter = new Linter(req, opts);
  await linter.lint();
}
