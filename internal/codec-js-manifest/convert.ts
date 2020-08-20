/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {stringifySPDXLicense} from "@internal/codec-spdx-license";
import {ManifestDependencies, stringifyDependencyPattern} from "./dependencies";
import {
	JSONManifest,
	JSONManifestExports,
	Manifest,
	ManifestExportCondition,
	ManifestExports,
} from "./types";
import {stringifySemver} from "@internal/codec-semver";
import {Dict} from "@internal/typescript-helpers";
import {stringifyPathPattern} from "@internal/path-match";
import {manifestNameToString} from "./name";

export function convertManifestToJSON(manifest: Manifest): JSONManifest {
	return {
		// Include unknown properties from the initial package.json
		...manifest.raw,
		name: manifestNameToString(manifest.name),
		description: manifest.description,
		private: manifest.private,
		type: manifest.type,
		homepage: manifest.homepage,
		repository: manifest.repository,
		bugs: manifest.bugs,
		main: manifest.main,
		// TODO we now support fallbacks which means manifest.exports is lossy
		exports: exportsToObject(manifest.exports),
		author: manifest.author,
		contributors: manifest.contributors,
		maintainers: manifest.maintainers,
		version: manifest.version === undefined
			? undefined
			: stringifySemver(manifest.version),
		license: manifest.license === undefined
			? undefined
			: stringifySPDXLicense(manifest.license),
		files: maybeArray(
			manifest.files.map((pattern) => stringifyPathPattern(pattern)),
		),
		keywords: maybeArray(manifest.keywords),
		cpu: maybeArray(manifest.cpu),
		os: maybeArray(manifest.os),
		bin: mapToObject(manifest.bin),
		scripts: mapToObject(manifest.scripts),
		engines: mapToObject(manifest.engines),
		dependencies: dependencyMapToObject(manifest.dependencies),
		devDependencies: dependencyMapToObject(manifest.devDependencies),
		optionalDependencies: dependencyMapToObject(manifest.optionalDependencies),
		peerDependencies: dependencyMapToObject(manifest.peerDependencies),
		// Common misspelling. If this existed then it was turned into bundledDependencies
		bundleDependencies: undefined,
		bundledDependencies: maybeArray(manifest.bundledDependencies),
	};
}

function exportsToObject(
	exports: boolean | ManifestExports,
): undefined | false | JSONManifestExports {
	if (exports === false) {
		return false;
	}

	if (exports === true) {
		return;
	}

	if (exports.size === 0) {
		return {};
	}

	const obj: JSONManifestExports = {};

	for (const [key, entries] of exports) {
		const entriesObj: JSONManifestExports[keyof JSONManifestExports] = {};
		for (const [type, condition] of entries) {
			entriesObj[type] = exportConditionToObject(condition);
		}
		obj[key.join()] = entriesObj;
	}

	return obj;
}

function exportConditionToObject(
	condition: ManifestExportCondition,
): string | Dict<string> {
	if (condition.type === "relative") {
		return condition.relative.join();
	}

	const obj: Dict<string> = {};
	for (const [key, value] of condition.conditions) {
		obj[key] = value.relative.join();
	}
	return obj;
}

function maybeArray<T>(items: Array<T>): undefined | Array<T> {
	if (items.length === 0) {
		return undefined;
	} else {
		return items;
	}
}

function mapToObject<T>(map: Map<string, T>): undefined | Dict<T> {
	if (map.size === 0) {
		return;
	}

	const obj: Dict<T> = {};
	for (const [key, value] of map) {
		obj[key] = value;
	}
	return obj;
}

function dependencyMapToObject(
	map: ManifestDependencies,
): undefined | Dict<string> {
	if (map.size === 0) {
		return;
	}

	const obj: Dict<string> = {};
	for (const [name, pattern] of map) {
		obj[name] = stringifyDependencyPattern(pattern);
	}
	return obj;
}
