/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {consumeJSON} from "@romefrontend/codec-json";
import {
	USER_CONFIG_DIRECTORY,
	USER_CONFIG_FILENAMES,
	VERSION,
} from "./constants";
import {AbsoluteFilePath, TEMP_PATH} from "@romefrontend/path";
import {existsSync, readFileTextSync} from "@romefrontend/fs";
import {Consumer} from "@romefrontend/consume";
import {descriptions} from "@romefrontend/diagnostics";

export type UserConfig = {
	configPath: undefined | AbsoluteFilePath;
	cachePath: AbsoluteFilePath;
	recoveryPath: AbsoluteFilePath;
	syntaxTheme: undefined | Consumer;
};

export const DEFAULT_USER_CONFIG: UserConfig = {
	configPath: undefined,
	cachePath: TEMP_PATH.append(`rome-${VERSION}`),
	recoveryPath: USER_CONFIG_DIRECTORY.append("recovery"),
	syntaxTheme: undefined,
};

export function normalizeUserConfig(
	consumer: Consumer,
	configPath: AbsoluteFilePath,
): UserConfig {
	const userConfig: UserConfig = {
		...DEFAULT_USER_CONFIG,
	};

	if (consumer.has("cachePath")) {
		userConfig.cachePath = consumer.get("cachePath").asAbsoluteFilePath(
			undefined,
			configPath.getParent(),
		);
	}

	if (consumer.has("vscodeTheme")) {
		const prop = consumer.get("vscodeTheme");
		const path = prop.asAbsoluteFilePath(undefined, configPath.getParent());

		if (existsSync(path)) {
			const input = readFileTextSync(path);

			userConfig.syntaxTheme = consumeJSON({
				consumeDiagnosticCategory: "parse/vscodeTheme",
				input,
				path,
			});
		} else {
			throw prop.unexpected(descriptions.USER_CONFIG.VSCODE_THEME_NOT_FOUND);
		}
	}

	consumer.enforceUsedProperties("config property");

	return userConfig;
}

let loadedUserConfig: undefined | UserConfig;

export function getUserConfigFile():
	| undefined
	| {
			consumer: Consumer;
			configPath: AbsoluteFilePath;
		} {
	for (const configFilename of USER_CONFIG_FILENAMES) {
		const configPath = USER_CONFIG_DIRECTORY.append(configFilename);

		if (!existsSync(configPath)) {
			continue;
		}

		const configFile = readFileTextSync(configPath);
		const consumer = consumeJSON({
			path: configPath,
			input: configFile,
		});
		return {consumer, configPath};
	}

	return undefined;
}

export function loadUserConfig(): UserConfig {
	if (loadedUserConfig !== undefined) {
		return loadedUserConfig;
	}

	const res = getUserConfigFile();
	if (res === undefined) {
		loadedUserConfig = DEFAULT_USER_CONFIG;
		return loadedUserConfig;
	} else {
		loadedUserConfig = normalizeUserConfig(res.consumer, res.configPath);
		return loadedUserConfig;
	}
}
