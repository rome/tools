/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import packageJson from "../../../package.json";
import {
	AbsoluteFilePath,
	HOME_PATH,
	TEMP_PATH,
	createAbsoluteFilePath,
} from "@internal/path";
import {getEnvVar} from "@internal/cli-environment";
import os = require("os");

// Node flags to pass to all forked processes
export const CHILD_ARGS = ["--max-old-space-size=8192", "--trace-warnings"];

export function getBinPath(): AbsoluteFilePath {
	return createAbsoluteFilePath(__filename);
}

const MEGABYTE = 10_000;

// Version constants
export let VERSION = String(packageJson.version);
export let REQUIRED_NODE_VERSION_RANGE = String(packageJson.engines.node);

// Constants used to handle scaling
export const MAX_MASTER_BYTES_BEFORE_WORKERS = 0.5 * MEGABYTE;
export const MAX_WORKER_BYTES_BEFORE_ADD = MEGABYTE;
const CPU_COUNT: number = os.cpus().length;
export const MAX_WORKER_COUNT = Math.min(CPU_COUNT, 4);

// Vendor Rome and Trunk Rome could have the same version number if there was no release in between
// Ensure they are properly namespaced to avoid having daemon socket conflicts
if (getEnvVar("ROME_DEV").type === "ENABLED") {
	VERSION += "-dev";
}

// Misc
export const MOCKS_DIRECTORY_NAME = "__rmocks__";

// Used as a heartbeat timeout to indicate if a process is unresponsive
export const LAG_INTERVAL = 3_000;

// # Folders
// XDG environment variables information:
// - https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html
// - https://docs.racket-lang.org/basedir/index.html

function getEnvironmentDirectory(
	key: string,
	append?: string,
): undefined | AbsoluteFilePath {
	const env = process.env[key];
	if (env === undefined) {
		return undefined;
	}

	let dir = createAbsoluteFilePath(env);
	if (append !== undefined) {
		dir = dir.append(append);
	}
	return dir;
}

function getLocalAppDataDir(): AbsoluteFilePath {
	return (
		getEnvironmentDirectory("LOCALAPPDATA", "Rome") ??
		HOME_PATH.append("AppData", "Local", "Rome")
	);
}

// ## User config directory
// Where we store user configuration, recoverable files etc

// XDG/Linux: ~/.config/rome
// Windows: ~/AppData/Local/Rome/Config
// Mac: ~/Library/Preferences/Rome
function getUserConfigDirectory(): AbsoluteFilePath {
	const XDG_CONFIG_HOME = getEnvironmentDirectory("XDG_CONFIG_HOME", "rome");
	if (XDG_CONFIG_HOME !== undefined) {
		return XDG_CONFIG_HOME;
	}

	if (process.platform === "win32") {
		return getLocalAppDataDir().append("Config");
	}

	if (process.platform === "darwin") {
		return HOME_PATH.append("Library", "Preferences", "Rome");
	}

	return HOME_PATH.append(".config", "rome");
}

export const USER_CONFIG_DIRECTORY = getUserConfigDirectory();
export const USER_CONFIG_FILENAMES: Array<string> = [
	"config.json",
	"config.rjson",
];

export const DEFAULT_USER_CONFIG_RECOVERY_DIRECTORY = USER_CONFIG_DIRECTORY.append(
	"recovery",
);

// ## Cache
// User specific non-essential data files

// XDG/Linux: ~/.cache/rome
// Windows: ~/AppData/Local/Temp/Rome
// Mac: ~/Library/Caches/Rome
function getCacheDirectory(): AbsoluteFilePath {
	const XDG_CACHE_HOME = getEnvironmentDirectory("XDG_CACHE_HOME", "rome");
	if (XDG_CACHE_HOME !== undefined) {
		return XDG_CACHE_HOME;
	}

	if (process.platform === "win32") {
		// process.env.TEMP also exists, but most apps put caches here
		return getLocalAppDataDir().append("Cache");
	}

	if (process.platform === "darwin") {
		return HOME_PATH.append("Library", "Caches", "Rome");
	}

	return HOME_PATH.append(".cache", "rome");
}

export const DEFAULT_CACHE_PATH = getCacheDirectory();

// ## Data
// User specific data files

// XDG/Linux: ~/.local/share/rome
// Windows: ~/AppData/Local/Rome/Data
// Mac: ~/Library/Rome
function getDataDirectory(): AbsoluteFilePath {
	const XDG_DATA_HOME = getEnvironmentDirectory("XDG_DATA_HOME", "rome");
	if (XDG_DATA_HOME !== undefined) {
		return XDG_DATA_HOME;
	}

	if (process.platform === "win32") {
		return getLocalAppDataDir().append("Data");
	}

	if (process.platform === "darwin") {
		return HOME_PATH.append("Library", "Rome");
	}

	return HOME_PATH.append(".local", "share", "rome");
}

export const DATA_DIRECTORY = getDataDirectory();

// ## Runtime
// Ephemeral things like pipes and sockets or other objects restricted to the current run of the program
// > The directory MUST be owned by the user, and MUST be the only one having read and write access to it.
// > Its Unix access mode MUST be 0700.

// XDG/Linux: /run/user/$(id -u)
// Windows: $TEMP/rome
// Mac: $TEMP/rome
function getRuntimeDirectory(): AbsoluteFilePath {
	const XDG_RUNTIME_DIR = getEnvironmentDirectory("XDG_RUNTIME_DIR", "rome");
	if (XDG_RUNTIME_DIR !== undefined) {
		return XDG_RUNTIME_DIR;
	}

	return TEMP_PATH.append("rome");
}

export const RUNTIME_DIRECTORY = getRuntimeDirectory();

function createPipePath(name: string): AbsoluteFilePath {
	if (process.platform === "win32") {
		return createAbsoluteFilePath(String.raw`\\.\pipe\rome-${VERSION}-${name}`);
	} else {
		return RUNTIME_DIRECTORY.append(`${VERSION}-wait.sock`);
	}
}

export const SERVER_SOCKET_PATH = createPipePath("server");
export const CLI_SOCKET_PATH = createPipePath("server-wait");
