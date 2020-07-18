/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	getErrorStructure,
	initErrorHooks,
	sourceMapManager,
} from "@romefrontend/v8";
import {BIN, MAP, VERSION} from "@romefrontend/core";
import cli from "../cli";
import server from "../server";
import testWorker from "../testWorker";
import worker from "../worker";
import {readFileTextSync} from "@romefrontend/fs";
import {SourceMapConsumer} from "@romefrontend/codec-source-map";
import {
	DiagnosticsProcessor,
	catchDiagnostics,
} from "@romefrontend/diagnostics";
import {printDiagnostics} from "@romefrontend/cli-diagnostics";
import {Reporter} from "@romefrontend/cli-reporter";

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

sourceMapManager.add(
	BIN.join(),
	SourceMapConsumer.fromJSONLazy(
		BIN.join(),
		() => JSON.parse(readFileTextSync(MAP)),
	),
);

export function executeCLIMain() {
	initErrorHooks();

	setInterval(
		() => {
			// We want to exit on our own terms
		},
		1_000_000,
	);

	catchDiagnostics(main).then(({diagnostics}) => {
		if (diagnostics !== undefined) {
			printDiagnostics({
				diagnostics,
				suppressions: [],
				printerOptions: {
					processor: new DiagnosticsProcessor(),
					reporter: Reporter.fromProcess(),
				},
			}).catch((err) => {
				console.error("Error while printing diagnostics");
				console.error(err.stack);
			}).finally(() => {
				process.exit(1);
			});
		}
	}).catch((err: Error) => {
		console.error("Error thrown inside the CLI handler");
		console.error(getErrorStructure(err).stack);
		process.exit(1);
	});
}

executeCLIMain();
