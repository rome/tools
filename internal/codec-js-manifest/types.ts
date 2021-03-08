/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ManifestDependenciesField} from "./normalize/dependencies";
import {SPDXExpressionNode} from "@internal/codec-spdx-license";
import {SemverVersionNode} from "@internal/codec-semver";
import {Consumer} from "@internal/consume";
import {
	AbsoluteFilePath,
	RelativePath,
	RelativePathMap,
	URLPath,
} from "@internal/path";
import {JSONObject, JSONPropertyValue} from "@internal/codec-config";
import {Dict} from "@internal/typescript-helpers";
import {PathPattern} from "@internal/path-match";
import {Diagnostic} from "@internal/diagnostics";

export type StringObject = Dict<string>;

export type MString = undefined | string;

export type MStringArray = undefined | (string[]);

export type MStringObject = undefined | StringObject;

export type ManifestStringMap = Map<string, string>;

export type ManifestPersonField = {
	name: MString;
	email: MString;
	twitter: MString;
	github: MString;
	url: MString;
};

export type ManifestExportsField = RelativePathMap<ManifestExportConditions>;

export type ManifestExportRelativeCondition = {
	type: "relative";
	consumer: Consumer;
	relative: RelativePath;
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

export type ManifestDependencies = {
	dependencies: ManifestDependenciesField;
	devDependencies: ManifestDependenciesField;
	optionalDependencies: ManifestDependenciesField;
	peerDependencies: ManifestDependenciesField;
	bundledDependencies: string[];
};

export type ManifestPeople = {
	author: undefined | ManifestPersonField;
	contributors: undefined | (ManifestPersonField[]);
	maintainers: undefined | (ManifestPersonField[]);
};

export type ManifestURLs = {
	homepage: undefined | URLPath;
	repository:
		| undefined
		| {
				type: string;
				url: string;
				directory: MString;
			};
	bugs:
		| undefined
		| {
				url: MString;
				email: MString;
			};
};

export type ManifestEnvironment = {
	cpu: string[];
	os: string[];
	engines: ManifestStringMap;
};

export type ManifestMetadata = {
	name: ManifestName;
	description: MString;
	version: undefined | SemverVersionNode;
	license: undefined | SPDXExpressionNode;
	private: boolean;
	keywords: string[];
};

export type ManifestFiles = {
	type: undefined | "module" | "commonjs";
	bin: ManifestStringMap;
	main: undefined | RelativePath;
	exports: boolean | ManifestExportsField;
	files: PathPattern[];
	scripts: ManifestStringMap;
};

export type Manifest = ManifestMetadata &
	ManifestDependencies &
	ManifestFiles &
	ManifestPeople &
	ManifestURLs &
	ManifestEnvironment & {
		raw: JSONObject;
		diagnostics: {
			license: Diagnostic[] | undefined;
		};
	};

// Serialized version of a Manifest
export type JSONManifest = {
	name: MString;
	description: Manifest["description"];
	version: MString;
	license: MString;
	private: undefined | Manifest["private"];
	type: Manifest["type"];
	homepage: undefined | string;
	repository: Manifest["repository"];
	bugs: Manifest["bugs"];
	main: undefined | string;
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
