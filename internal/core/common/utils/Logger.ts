/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Reporter, ReporterOptions} from "@internal/cli-reporter";
import {AnyMarkup, markup} from "@internal/markup";
import workerThreads = require("worker_threads");

export default class Logger extends Reporter {
	constructor(opts: ReporterOptions, loggerType: string) {
		super(opts);
		this.loggerType = loggerType;
	}

	private loggerType: string;

	protected getMessagePrefix(): AnyMarkup {
		const inner = `${this.loggerType} ${process.pid}:${workerThreads.threadId}`;
		const timestamp = new Date().toISOString();
		return markup`<dim>[${timestamp}] [${inner}]</dim> `;
	}
}
