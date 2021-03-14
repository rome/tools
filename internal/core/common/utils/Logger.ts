/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Reporter, ReporterOptions} from "@internal/cli-reporter";
import {Markup, markup} from "@internal/markup";
import {Resource} from "@internal/resources";
import workerThreads = require("worker_threads");

export default class Logger extends Reporter {
	constructor(resources: Resource, opts: ReporterOptions, loggerType: string) {
		super(`Logger<${loggerType}>`, opts);
		this.loggerType = loggerType;
		resources.add(this);
	}

	private loggerType: string;

	protected getMessagePrefix(): Markup {
		const inner = `${this.loggerType}:${workerThreads.threadId}`;
		// TODO: Disable timestamp for the timebeing. It's way too noisy when displayed in the console.
		//const timestamp = new Date().toISOString();
		//return markup`<dim>[${timestamp}] [${inner}]</dim> `;
		return markup`<dim>[${inner}]</dim> `;
	}
}
