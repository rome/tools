/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Reporter, ReporterOptions} from '@romejs/cli-reporter';

export default class Logger extends Reporter {
  constructor(name: string, isEnabled: () => boolean, opts: ReporterOptions) {
    super({
      verbose: true,
      ...opts,
    });
    this._loggerName = name;
    this.isEnabled = isEnabled;
  }

  _loggerName: string;

  getMessagePrefix() {
    return `<dim>[${this._loggerName} ${process.pid}]</dim> `;
  }
}
