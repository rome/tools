/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import packageJson from "../../../package.json";
import {HOME_PATH, TEMP_PATH, createAbsoluteFilePath} from "@internal/path";
import {getEnvVar} from "@internal/cli-environment";
import os = require("os");

// Node flags to pass to all forked processes
export const CHILD_ARGS = ["--max-old-space-size=8192", "--trace-warnings"];

// @ts-ignore: this will be wrong if we weren't the entry node script
export const BIN = createAbsoluteFilePath(
	process.mainModule === undefined
		? module.filename
		: process.mainModule.filename,
);
export const MAP = BIN.addExtension(".map");

const MEGABYTE = 10_000;

// Where we store user configuration, recoverable files etc
export const USER_CONFIG_DIRECTORY = HOME_PATH.append(".config", "rome");
export const USER_CONFIG_FILENAMES: Array<string> = [
	"config.json",
	"config.rjson",
];

// Constants used to handle scaling
export const MAX_MASTER_BYTES_BEFORE_WORKERS = 0.5 * MEGABYTE;
export const MAX_WORKER_BYTES_BEFORE_ADD = MEGABYTE;
const CPU_COUNT: number = os.cpus().length;
export const MAX_WORKER_COUNT = Math.min(CPU_COUNT, 4);

// Verson constants
export let VERSION = String(packageJson.version);
export let REQUIRED_NODE_VERSION_RANGE = String(packageJson.engines.node);

// Vendor Rome and Trunk Rome could have the same version number if there was no release in between
// Ensure they are properly namespaced to avoid having daemon socket conflicts
if (getEnvVar("ROME_DEV").type === "ENABLED") {
	VERSION += "-dev";
}
export const SOCKET_PATH = TEMP_PATH.append(`rome-${VERSION}.sock`);
export const CLI_SOCKET_PATH = TEMP_PATH.append(`rome-wait-${VERSION}.sock`);

// Misc
export const MOCKS_DIRECTORY_NAME = "__rmocks__";

// Used as a timeout to indicate if a
export const LAG_INTERVAL = 3_000;
