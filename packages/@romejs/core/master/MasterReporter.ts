/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  Reporter,
  ReporterProgress,
  ReporterProgressOptions,
  mergeProgresses,
} from '@romejs/cli-reporter';
import Master from './Master';

export default class MasterReporter extends Reporter {
  constructor(master: Master) {
    super({
      wrapperFactory: master.wrapFatal.bind(master),
    });
    this.master = master;
  }

  master: Master;

  // This is so all progress bars are also shown on an LSP client, alongside connected CLIs
  progress(opts?: ReporterProgressOptions): ReporterProgress {
    const progresses: Array<ReporterProgress> = [this.progressLocal(opts)];

    for (const server of this.master.connectedLSPServers) {
      progresses.push(server.createProgress(opts));
    }

    return mergeProgresses(progresses);
  }
}
