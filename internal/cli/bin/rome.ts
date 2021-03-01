/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {errorSourceMaps, initErrorHooks} from "@internal/v8";
import {VERSION, getBinPath} from "@internal/core";
import cli from "../cli";
import server from "../server";
import worker from "../worker";
import {SourceMapConsumer} from "@internal/codec-source-map";
import {Reporter} from "@internal/cli-reporter";
import {markup} from "@internal/markup";
import FatalErrorHandler from "@internal/core/common/FatalErrorHandler";

async function main(): Promise<void> {
	switch (
		process.env.ROME_PROCESS_VERSION === VERSION &&
		process.env.ROME_PROCESS_TYPE
	) {
		case "server":
			return server();

		case "worker":
			return worker();

		default:
			return cli();
	}
}

const bin = getBinPath();
errorSourceMaps.add(
	bin,
	SourceMapConsumer.fromJSONLazy(
		bin,
		() => JSON.parse(getBinPath().addExtension(".map").readFileTextSync()),
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

	const fatalErrorHandler = new FatalErrorHandler({
		getOptions() {
			return {
				source: markup`cli`,
				reporter: Reporter.fromProcess(),
			};
		},
	});

	fatalErrorHandler.wrapPromise(main());
}

executeCLIMain();
