/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ProjectConfig, ProjectConfigJSON} from "./types";
import {AbsoluteFilePathSet, createAbsoluteFilePath} from "@romefrontend/path";

export function serializeJSONProjectConfig(
	config: ProjectConfig,
): ProjectConfigJSON {
	const targets: ProjectConfigJSON["targets"] = {};
	for (const [name, target] of config.targets) {
		targets[name] = target;
	}

	return {
		...config,
		vcs: {
			...config.vcs,
			root: config.vcs.root.join(),
		},
		typeCheck: {
			...config.typeCheck,
			libs: Array.from(config.typeCheck.libs, (path) => path.join()),
		},
		files: {
			...config.files,
			vendorPath: config.files.vendorPath.join(),
		},
		targets,
	};
}

export function hydrateJSONProjectConfig(
	config: ProjectConfigJSON,
): ProjectConfig {
	return {
		...config,
		files: {
			...config.files,
			vendorPath: createAbsoluteFilePath(config.files.vendorPath),
		},
		vcs: {
			...config.vcs,
			root: createAbsoluteFilePath(config.vcs.root),
		},
		typeCheck: {
			...config.typeCheck,
			libs: new AbsoluteFilePathSet(
				config.typeCheck.libs.map((str) => createAbsoluteFilePath(str)),
			),
		},
		targets: new Map(Object.entries(config.targets)),
	};
}
