/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Consumer} from "@internal/consume";
import {PathPatterns, parsePathPattern} from "@internal/path-match";
import {AbsoluteFilePath, AbsoluteFilePathSet} from "@internal/path";
import {ProjectConfig, ProjectConfigMeta, ProjectConfigMetaHard} from "./types";
import {PROJECT_CONFIG_FILENAMES} from "./constants";

export function assertHardMeta(meta: ProjectConfigMeta): ProjectConfigMetaHard {
	const {configPath, projectDirectory: directory, consumer} = meta;
	if (
		configPath === undefined ||
		directory === undefined ||
		consumer === undefined
	) {
		throw new Error("This is not a disk project");
	}

	return {
		...meta,
		configPath,
		consumer,
		projectDirectory: directory,
	};
}

export function arrayOfStrings(consumer: Consumer): string[] {
	if (consumer.exists()) {
		return consumer.asMappedArray((item) => item.asString());
	} else {
		return [];
	}
}

export function arrayOfPatterns(consumer: Consumer): PathPatterns {
	// TODO consumer.handleThrownDiagnostics
	return consumer.asMappedArray((item) => {
		return parsePathPattern({
			path: consumer.filename,
			input: item.asString(),
			offsetPosition: item.getLocation("inner-value").start,
		});
	});
}

export function mergeArrays<T>(
	a: undefined | (T[]),
	b: undefined | (T[]),
): undefined | (T[]) {
	if (a === undefined) {
		return a;
	}

	if (b === undefined) {
		return a;
	}

	return [...a, ...b];
}

export function mergeAbsoluteFilePathSets(
	a: undefined | AbsoluteFilePathSet,
	b: undefined | AbsoluteFilePathSet,
): undefined | AbsoluteFilePathSet {
	if (a === undefined) {
		return a;
	}

	if (b === undefined) {
		return a;
	}

	return new AbsoluteFilePathSet([...a, ...b]);
}

const ESLINT_CONFIG_FILENAMES: Array<string> = [
	".eslintrc.js",
	".eslintrc.cjs",
	".eslintrc.yaml",
	".eslintrc.yml",
	".eslintrc.json",
	".eslintignore",
];

// Get an array of possible files in parent directories that will cause a project cache invalidation
export function getParentConfigDependencies(
	path: AbsoluteFilePath,
	config: ProjectConfig,
): AbsoluteFilePathSet {
	const deps: AbsoluteFilePathSet = new AbsoluteFilePathSet();

	for (const directory of path.getChain()) {
		let basenames: Array<string> = [
			"package.json",
		];

		// If eslint integration is enabled then eslint config changes should cause invalid cache files
		if (config.integrations.eslint.enabled) {
			basenames = basenames.concat(ESLINT_CONFIG_FILENAMES);
		}

		for (const configFilename of PROJECT_CONFIG_FILENAMES) {
			basenames.push(configFilename);
			deps.add(directory.append(".config", configFilename));
		}

		for (const basename of basenames) {
			deps.add(directory.append(basename));
		}
	}

	return deps;
}
