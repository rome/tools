/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Server} from "@romefrontend/core";
import {modules} from "./runtime-modules";
import {AbsoluteFilePath} from "@romefrontend/path";
import {createDirectory, lstat, updateTimes, writeFile} from "@romefrontend/fs";
import {
	ProjectConfig,
	createDefaultProjectConfig,
	createDefaultProjectConfigMeta,
} from "@romefrontend/project";

export default class VirtualModules {
	constructor(server: Server) {
		this.server = server;
		this.runtimeModulesPath = server.userConfig.runtimeModulesPath;
	}

	runtimeModulesPath: AbsoluteFilePath;
	server: Server;

	async init() {
		const {runtimeModulesPath} = this;

		// Materalize virtual files to disk
		// We could technically keep these in memory and never materialize them but
		// this way we can have something to point at on disk for errors etc
		await createDirectory(runtimeModulesPath);
		await Promise.all(
			Array.from(
				modules,
				async ([name, files]) => {
					const modulePath = runtimeModulesPath.append(name);
					await createDirectory(modulePath);

					await Promise.all(
						Array.from(
							files,
							async ([basename, {content, mtime}]) => {
								const path = modulePath.append(basename);
								try {
									const stats = await lstat(path);

									// Check if it's the same file
									if (Math.floor(stats.mtimeMs / 1_000) === mtime) {
										return;
									}
								} catch (err) {
									// Swallow file doesn't exist errors
									if (err.code !== "ENOENT") {
										throw err;
									}
								}

								await writeFile(path, content);
								await updateTimes(path, mtime, mtime);
							},
						),
					);
				},
			),
		);

		// Initialize as project
		const projectConfig: ProjectConfig = {
			...createDefaultProjectConfig(),
			name: "rome-runtime",
		};
		await this.server.projectManager.declareProject({
			projectFolder: runtimeModulesPath,
			meta: createDefaultProjectConfigMeta(),
			config: projectConfig,
		});
		await this.server.memoryFs.watch(runtimeModulesPath);
	}

	resolve(name: string): undefined | AbsoluteFilePath {
		if (modules.has(name)) {
			return this.runtimeModulesPath.append(name);
		} else {
			return undefined;
		}
	}
}
