/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {initErrorHooks, sourceMapManager} from "@internal/v8";
import {VERSION, getBinPath} from "@internal/core";
import cli from "../cli";
import server from "../server";
import testWorker from "../testWorker";
import worker from "../worker";
import {SourceMapConsumer} from "@internal/codec-source-map";
import {Reporter} from "@internal/cli-reporter";
import {markup} from "@internal/markup";
import handleFatalError from "@internal/core/common/handleFatalError";
import {readFileTextSync} from "@internal/fs";

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

const bin = getBinPath().join();
sourceMapManager.add(
	bin,
	SourceMapConsumer.fromJSONLazy(
		bin,
		() => JSON.parse(readFileTextSync(getBinPath().addExtension(".map"))),
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

	main().catch((error: Error) => {
		handleFatalError({
			error,
			source: markup`cli`,
			reporter: Reporter.fromProcess(),
		});
	});
}

executeCLIMain();
