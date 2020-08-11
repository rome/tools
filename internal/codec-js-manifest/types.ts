/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ManifestDependencies} from "./dependencies";
import {SPDXExpressionNode} from "@internal/codec-spdx-license";
import {SemverVersionNode} from "@internal/codec-semver";
import {Consumer} from "@internal/consume";
import {
	AbsoluteFilePath,
	RelativeFilePath,
	RelativeFilePathMap,
} from "@internal/path";
import {JSONObject, JSONPropertyValue} from "@internal/codec-json";
import {Dict} from "@internal/typescript-helpers";
import {PathPatterns} from "@internal/path-match";

export type StringObject = Dict<string>;

export type MString = undefined | string;

export type MStringArray = undefined | Array<string>;

export type MStringObject = undefined | StringObject;

export type MBoolean = undefined | boolean;

export type ManifestMap = Map<string, string>;

export type ManifestPerson = {
	name: MString;
	email: MString;
	twitter: MString;
	github: MString;
	url: MString;
};

export type ManifestRepository = {
	type: string;
	url: string;
	directory: MString;
};

export type ManifestBugs = {
	url: MString;
	email: MString;
};

export type ManifestExports = RelativeFilePathMap<ManifestExportConditions>;

export type ManifestExportRelativeCondition = {
	type: "relative";
	consumer: Consumer;
	relative: RelativeFilePath;
};

export type ManifestExportNestedCondition = {
	type: "nested";
	consumer: Consumer;
	conditions: Map<string, ManifestExportRelativeCondition>;
};

export type ManifestExportCondition =
	| ManifestExportRelativeCondition
	| ManifestExportNestedCondition;

export type ManifestExportConditions = Map<string, ManifestExportCondition>;

export type ManifestName = {
	org?: string;
	packageName?: string;
};

export type Manifest = {
	name: ManifestName;
	description: MString;
	version: undefined | SemverVersionNode;
	license: undefined | SPDXExpressionNode;
	private: boolean;
	type: undefined | "module" | "commonjs";
	homepage: MString;
	repository: undefined | ManifestRepository;
	bugs: undefined | ManifestBugs;
	main: MString;
	exports: boolean | ManifestExports;
	author: undefined | ManifestPerson;
	contributors: undefined | Array<ManifestPerson>;
	maintainers: undefined | Array<ManifestPerson>;
	files: PathPatterns;
	keywords: Array<string>;
	cpu: Array<string>;
	os: Array<string>;
	bin: ManifestMap;
	scripts: ManifestMap;
	engines: ManifestMap;
	dependencies: ManifestDependencies;
	devDependencies: ManifestDependencies;
	optionalDependencies: ManifestDependencies;
	peerDependencies: ManifestDependencies;
	bundledDependencies: Array<string>;
	raw: JSONObject;
};

// Serialized version of a Manifest
export type JSONManifest = {
	name: MString;
	description: Manifest["description"];
	version: MString;
	license: MString;
	private: Manifest["private"];
	type: Manifest["type"];
	homepage: Manifest["homepage"];
	repository: Manifest["repository"];
	bugs: Manifest["bugs"];
	main: Manifest["main"];
	exports: undefined | false | JSONManifestExports;
	author: Manifest["author"];
	contributors: Manifest["contributors"];
	maintainers: Manifest["maintainers"];
	files: MStringArray;
	keywords: MStringArray;
	cpu: MStringArray;
	os: MStringArray;
	bin: MStringObject;
	scripts: MStringObject;
	engines: MStringObject;
	dependencies: MStringObject;
	devDependencies: MStringObject;
	optionalDependencies: MStringObject;
	peerDependencies: MStringObject;
	bundledDependencies: MStringArray;
	[key: string]: JSONPropertyValue;
};

export type JSONManifestExports = Dict<Dict<Dict<string> | string> | string>;

export type ManifestDefinition = {
	path: AbsoluteFilePath;
	directory: AbsoluteFilePath;
	id: number;
	consumer: Consumer;
	manifest: Manifest;
	hash: string;
};
