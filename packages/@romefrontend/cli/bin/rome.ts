/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {catchDiagnostics} from "@romefrontend/diagnostics";
import {printDiagnostics} from "@romefrontend/cli-diagnostics";
import {
	getErrorStructure,
	initErrorHooks,
	sourceMapManager,
} from "@romefrontend/v8";
import {Reporter} from "@romefrontend/cli-reporter";
import {BIN, MAP, VERSION} from "@romefrontend/core";
import cli from "../cli";
import server from "../server";
import testWorker from "../testWorker";
import worker from "../worker";
import {readFileTextSync} from "@romefrontend/fs";
import {SourceMapConsumer} from "@romefrontend/codec-source-map";

async function main(): Promise<void> {
	switch (
		process.env.ROME_PROCESS_VERSION === VERSION &&
		process.env.ROME_PROCESS_TYPE
	) {
		case "server":
			return server();

		case "worker":
			return worker();

		case "test-worker":
			return testWorker();

		default:
			return cli();
	}
}

setInterval(
	() => {
		// We want to exit on our own terms
	},
	1_000_000,
);

initErrorHooks();

sourceMapManager.add(
	BIN.join(),
	SourceMapConsumer.fromJSONLazy(
		BIN.join(),
		() => JSON.parse(readFileTextSync(MAP)),
	),
);

catchDiagnostics(main).then(({diagnostics}) => {
	if (diagnostics !== undefined) {
		const reporter = Reporter.fromProcess();
		printDiagnostics({
			diagnostics,
			suppressions: [],
			printerOptions: {
				reporter,
			},
		});
		process.exit(1);
	}
}).catch((err: Error) => {
	console.error("Error thrown inside the CLI handler");
	console.error(getErrorStructure(err).stack);
	process.exit(1);
});
