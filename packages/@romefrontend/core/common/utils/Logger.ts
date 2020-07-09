/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	Reporter,
	ReporterConditionalStream,
	ReporterOptions,
} from "@romefrontend/cli-reporter";
import {AbsoluteFilePath} from "@romefrontend/path";
import {TERMINAL_FEATURES_ALL} from "@romefrontend/environment";

export type LoggerOptions = {
	cwd?: AbsoluteFilePath;
	excludePid?: boolean;
	type: string;
};

export type PartialLoggerOptions = Partial<LoggerOptions>;

export default class Logger extends Reporter {
	constructor(
		loggerOptions: LoggerOptions,
		opts: ReporterOptions,
		{write, check}: {
			check: () => boolean;
			write: (chunk: string) => void;
		},
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

		this.conditionalStream = this.attachConditionalStream(
			{
				type: "all",
				format: "none",
				columns: Infinity,
				features: TERMINAL_FEATURES_ALL,
				write,
			},
			check,
		);
	}

	conditionalStream: ReporterConditionalStream;
	loggerOptions: LoggerOptions;

	updateStream() {
		this.conditionalStream.update();
	}

	_getMessagePrefix() {
		let inner = this.loggerOptions.type;
		if (!this.loggerOptions.excludePid) {
			inner += ` ${process.pid}`;
		}
		return `<dim>[${inner}]</dim> `;
	}
}
