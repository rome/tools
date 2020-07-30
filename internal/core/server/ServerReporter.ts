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
} from "@internal/cli-reporter";
import Server from "./Server";

export default class ServerReporter extends Reporter {
	constructor(server: Server) {
		super({
			wrapperFactory: server.wrapFatal.bind(server),
		});
		this.server = server;
	}

	server: Server;

	// This is so all progress bars are renderer on each client. If we just use this.progressLocal then
	// while it would work, we would be doing all the rendering work on the server
	// The CLI also needs to know all the activeElements so it can properly draw and clear lines
	// We also create a progress bar for all connected LSP clients
	progress(opts?: ReporterProgressOptions): ReporterProgress {
		const progresses: Array<ReporterProgress> = [];

		for (const client of this.server.connectedClients) {
			progresses.push(client.reporter.progress(opts));
		}

		for (const server of this.server.connectedLSPServers) {
			progresses.push(server.createProgress(opts));
		}

		return mergeProgresses(progresses);
	}
}
