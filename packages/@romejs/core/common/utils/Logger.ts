/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ReporterOptions} from '@romejs/cli-reporter';
import {AnyEvent} from '@romejs/events';
import {Reporter} from '@romejs/cli-reporter';
import {ReporterStream} from '@romejs/cli-reporter';

export default class Logger extends Reporter {
  constructor(name: string, event: AnyEvent, opts: ReporterOptions) {
    super({
      verbose: true,
      ...opts,
    });
    this._loggerName = name;
    this.event = event;
  }

  _loggerName: string;
  event: AnyEvent;

  isEnabled() {
    return this.event.hasSubscribers();
  }

  getMessagePrefix(stream: ReporterStream) {
    if (stream.format === 'none') {
      return `[${this._loggerName} ${process.pid}] `;
    } else {
      return `<dim>[${this._loggerName} ${process.pid}]</dim> `;
    }
  }
}
