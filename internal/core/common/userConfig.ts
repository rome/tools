/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {json} from "@internal/codec-config";
import {
	DEFAULT_CACHE_PATH,
	DEFAULT_USER_CONFIG_RECOVERY_DIRECTORY,
	USER_CONFIG_DIRECTORY,
	USER_CONFIG_FILENAMES,
} from "./constants";
import {AbsoluteFilePath} from "@internal/path";
import {Consumer} from "@internal/consume";
import {descriptions} from "@internal/diagnostics";

export type UserConfig = {
	configPath: undefined | AbsoluteFilePath;
	cacheDirectory: AbsoluteFilePath;
	recoveryPath: AbsoluteFilePath;
	syntaxTheme: undefined | Consumer;
};

export const DEFAULT_USER_CONFIG: UserConfig = {
	configPath: undefined,
	cacheDirectory: DEFAULT_CACHE_PATH,
	recoveryPath: DEFAULT_USER_CONFIG_RECOVERY_DIRECTORY,
	syntaxTheme: undefined,
};

export async function normalizeUserConfig(
	consumer: Consumer,
	configPath: AbsoluteFilePath,
): Promise<UserConfig> {
	const userConfig: UserConfig = {
		...DEFAULT_USER_CONFIG,
	};

	if (consumer.has("cachePath")) {
		userConfig.cacheDirectory = consumer.get("cachePath").asAbsoluteFilePath(
			undefined,
			configPath.getParent(),
		);
	}

	if (consumer.has("vscodeTheme")) {
		const prop = consumer.get("vscodeTheme");
		const path = prop.asAbsoluteFilePath(undefined, configPath.getParent());

		if (await path.exists()) {
			const input = await path.readFileText();

			userConfig.syntaxTheme = json.consumeValue({
				consumeDiagnosticCategoryValue: "vscodeTheme",
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

export async function getUserConfigFile(): Promise<
	| undefined
	| {
			consumer: Consumer;
			configPath: AbsoluteFilePath;
		}
> {
	for (const configFilename of USER_CONFIG_FILENAMES) {
		const configPath = USER_CONFIG_DIRECTORY.append(configFilename);

		if (await configPath.notExists()) {
			continue;
		}

		const configFile = await configPath.readFileText();
		const consumer = json.consumeValue({
			path: configPath,
			input: configFile,
		});
		console.log(consumer)
		return {consumer, configPath};
	}

	return undefined;
}

export async function loadUserConfig(): Promise<UserConfig> {
	if (loadedUserConfig !== undefined) {
		return loadedUserConfig;
	}

	const res = await getUserConfigFile();
	if (res === undefined) {
		loadedUserConfig = DEFAULT_USER_CONFIG;
		return loadedUserConfig;
	} else {
		loadedUserConfig = await normalizeUserConfig(res.consumer, res.configPath);
		return loadedUserConfig;
	}
}
