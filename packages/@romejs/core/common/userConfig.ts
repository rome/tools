/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {consumeJSON} from "@romejs/codec-json";
import {VERSION} from "./constants";
import {ROME_CONFIG_FILENAMES} from "@romejs/project";
import {AbsoluteFilePath, HOME_PATH, TEMP_PATH} from "@romejs/path";
import {existsSync, readFileTextSync} from "@romejs/fs";
import {Consumer} from "@romejs/consume";
import {descriptions} from "@romejs/diagnostics";

export type UserConfig = {
	runtimeModulesPath: AbsoluteFilePath;
	cachePath: AbsoluteFilePath;
	syntaxTheme: undefined | Consumer;
};

const VERSION_PATH = TEMP_PATH.append(`rome-${VERSION}`);

export const DEFAULT_USER_CONFIG: UserConfig = {
	runtimeModulesPath: VERSION_PATH.append("runtime"),
	cachePath: VERSION_PATH.append("cache"),
	syntaxTheme: undefined,
};

export function loadUserConfig(): UserConfig {
	for (const configFilename of ROME_CONFIG_FILENAMES) {
		const configPath = HOME_PATH.append([".config", configFilename]);

		if (!existsSync(configPath)) {
			continue;
		}

		const configFile = readFileTextSync(configPath);
		const consumer = consumeJSON({
			path: configPath,
			input: configFile,
		});

		const userConfig: UserConfig = {
			...DEFAULT_USER_CONFIG,
		};

		if (consumer.has("cachePath")) {
			userConfig.cachePath = consumer.get("cachePath").asAbsoluteFilePath(
				undefined,
				configPath.getParent(),
			);
		}

		if (consumer.has("runtimeModulesPath")) {
			userConfig.runtimeModulesPath = consumer.get("runtimeModulesPath").asAbsoluteFilePath(
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

	return DEFAULT_USER_CONFIG;
}
