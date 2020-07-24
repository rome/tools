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
import {DEFAULT_TERMINAL_FEATURES} from "@romefrontend/cli-environment";
import {Markup, markup} from "@romefrontend/cli-layout";

export type InfoPrefixLogger = (msg: Markup) => void;

export default class Logger extends Reporter {
	constructor(
		loggerType: string,
		opts: ReporterOptions,
		{write, check}: {
			check: () => boolean;
			write: (chunk: string) => void;
		},
	) {
		super(opts);
		this.loggerType = loggerType;

		this.conditionalStream = this.attachConditionalStream(
			{
				type: "all",
				format: "markup",
				features: {
					...DEFAULT_TERMINAL_FEATURES,
					columns: undefined,
				},
				write,
			},
			check,
		);
	}

	conditionalStream: ReporterConditionalStream;
	loggerType: string;

	infoPrefix(prefix: Markup): InfoPrefixLogger {
		return (msg) => {
			this.info(markup`${prefix} ${msg}`);
		};
	}

	updateStream() {
		this.conditionalStream.update();
	}

	getMessagePrefix(): Markup {
		const inner = `${this.loggerType} ${process.pid}`;
		const timestamp = new Date().toISOString();
		return markup`<dim>[${timestamp}] [${inner}]</dim> `;
	}
}
