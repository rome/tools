/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {MasterRequest} from '@romejs/core';
import {createMasterCommand} from '../../commands';
import Linter from '../linter/Linter';
import {commandCategories} from '../../commands';

export default createMasterCommand({
  category: commandCategories.CODE_QUALITY,
  description: 'run lint against a set of files',

  async default(req: MasterRequest): Promise<void> {
    return new Promise((resolve, reject) => {
      if (req.query.requestFlags.watch) {
        initWatchLint(req, reject);
      } else {
        resolve(runLint(req));
      }
    });
  },
});

function initWatchLint(req: MasterRequest, reject: (err: Error) => void) {
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

    runLint(req).then(() => {
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

async function runLint(req: MasterRequest): Promise<void> {
  const linter = new Linter(req);
  await linter.lint();
}
