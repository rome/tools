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
} from "@romejs/cli-reporter";
import Server from "./Server";

export default class ServerReporter extends Reporter {
	constructor(server: Server) {
		super({
			wrapperFactory: server.wrapFatal.bind(server),
		});
		this.server = server;
	}

	server: Server;

	// This is so all progress bars are also shown on an LSP client, alongside connected CLIs
	progress(opts?: ReporterProgressOptions): ReporterProgress {
		const progresses: Array<ReporterProgress> = [this.progressLocal(opts)];

		for (const server of this.server.connectedLSPServers) {
			progresses.push(server.createProgress(opts));
		}

		return mergeProgresses(progresses);
	}
}
