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
import {AnyMarkup, markup} from "@romefrontend/cli-layout";

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

	updateStream() {
		this.conditionalStream.update();
	}

	getMessagePrefix(): AnyMarkup {
		const inner = `${this.loggerType} ${process.pid}`;
		const timestamp = new Date().toISOString();
		return markup`<dim>[${timestamp}] [${inner}]</dim> `;
	}
}
