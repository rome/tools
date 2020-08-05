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
} from "@internal/cli-reporter";
import Server from "./Server";

export default class ServerReporter extends Reporter {
	constructor(server: Server) {
		super({
			wrapperFactory: server.wrapFatal.bind(server),
		});
		this.server = server;
	}

	private server: Server;

	public progress(opts?: ReporterProgressOptions): ReporterProgress {
		return this.server.createConnectedProgress(opts);
	}
}
