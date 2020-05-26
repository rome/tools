/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Reporter, ReporterOptions} from "@romejs/cli-reporter";
import {AbsoluteFilePath} from "@romejs/path";

export type LoggerOptions = {
	cwd?: AbsoluteFilePath;
	excludePid?: boolean;
	type: string;
};

export type PartialLoggerOptions = Partial<LoggerOptions>;

export default class Logger extends Reporter {
	constructor(
		loggerOptions: LoggerOptions,
		isEnabled: () => boolean,
		opts: ReporterOptions,
	) {
		super({
			verbose: true,
			...opts,
			markupOptions: {
				cwd: loggerOptions.cwd,
				...opts.markupOptions,
			},
		});
		this.loggerOptions = loggerOptions;
		this.isEnabled = isEnabled;
	}

	loggerOptions: LoggerOptions;

	getMessagePrefix() {
		let inner = this.loggerOptions.type;
		if (!this.loggerOptions.excludePid) {
			inner += ` ${process.pid}`;
		}
		return `<dim>[${inner}]</dim> `;
	}
}
