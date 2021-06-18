/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Consumer} from "@internal/consume";
import {PathPattern, parsePathPattern} from "@internal/path-match";
import {AbsoluteFilePath, AbsoluteFilePathSet} from "@internal/path";
import {PartialProjectConfig} from "./types";
import {
	ESLINT_CONFIG_FILENAMES,
	PROJECT_CONFIG_FILENAMES,
	VCS_IGNORE_FILENAMES,
	PRETTIER_CONFIG_FILESNAMES
} from "./constants";

export function arrayOfStrings(consumer: Consumer): string[] {
	if (consumer.exists()) {
		return consumer.asMappedArray((item) => item.asString());
	} else {
		return [];
	}
}

export function arrayOfPatterns(consumer: Consumer): PathPattern[] {
	// TODO consumer.handleThrownDiagnostics
	return consumer.asMappedArray((item) => {
		return parsePathPattern({
			path: consumer.path,
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

// Get an array of possible files in parent directories that will cause a project cache invalidation
export function getParentConfigDependencies(
	{projectDirectory, rootProjectDirectory, partialConfig}: {
		projectDirectory: AbsoluteFilePath;
		rootProjectDirectory: AbsoluteFilePath;
		partialConfig: PartialProjectConfig;
	},
): AbsoluteFilePathSet {
	const deps: AbsoluteFilePathSet = new AbsoluteFilePathSet();

	for (const directory of projectDirectory.getChain()) {
		const atRoot = directory.equal(rootProjectDirectory);

		// If we are at the root directory then stop
		if (atRoot && !projectDirectory.equal(rootProjectDirectory)) {
			break;
		}

		let basenames: string[] = ["package.json"];

		// If eslint integration is enabled then eslint config changes should cause invalid cache files
		if (partialConfig.integrations.eslint.enabled) {
			basenames = basenames.concat(ESLINT_CONFIG_FILENAMES);
		}

		if (partialConfig.integrations.prettier.enabled) {
			basenames = basenames.concat(PRETTIER_CONFIG_FILESNAMES);
		}

		if (partialConfig.vcs.root?.equal(directory)) {
			basenames = basenames.concat(VCS_IGNORE_FILENAMES);
		}

		for (const configFilename of PROJECT_CONFIG_FILENAMES) {
			basenames.push(configFilename);
			deps.add(directory.append(".config", configFilename));
		}

		for (const basename of basenames) {
			deps.add(directory.append(basename));
		}

		// If we are the root project then don't go higher
		if (atRoot) {
			break;
		}
	}

	return deps;
}
