/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Master} from "@romejs/core";
import {modules} from "./runtime-modules";
import {AbsoluteFilePath} from "@romejs/path";
import {createDirectory, writeFile} from "@romejs/fs";
import {
	DEFAULT_PROJECT_CONFIG,
	DEFAULT_PROJECT_CONFIG_META,
	ProjectConfig,
} from "@romejs/project";

export default class VirtualModules {
	constructor(master: Master) {
		this.master = master;
		this.runtimeModulesPath = master.userConfig.runtimeModulesPath;
	}

	runtimeModulesPath: AbsoluteFilePath;
	master: Master;

	async init() {
		const {runtimeModulesPath} = this;

		// Materalize virtual files to disk
		// We could technically keep these in memory and never materialize them but
		// this way we can have something to point at on disk for errors etc
		await createDirectory(runtimeModulesPath);
		for (const [name, files] of modules) {
			const modulePath = runtimeModulesPath.append(name);
			await createDirectory(modulePath);
			for (const [basename, content] of files) {
				await writeFile(modulePath.append(basename), content);
			}
		}

		// Initialize as project
		const projectConfig: ProjectConfig = {
			...DEFAULT_PROJECT_CONFIG,
			name: "rome-runtime",
		};
		await this.master.projectManager.addProjectWithConfig({
			projectFolder: runtimeModulesPath,
			meta: DEFAULT_PROJECT_CONFIG_META,
			config: projectConfig,
		});
		await this.master.memoryFs.watch(runtimeModulesPath, projectConfig);
	}

	resolve(name: string): undefined | AbsoluteFilePath {
		if (modules.has(name)) {
			return this.runtimeModulesPath.append(name);
		} else {
			return undefined;
		}
	}
}
