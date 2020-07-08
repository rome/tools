/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ManifestDefinition} from "@romefrontend/codec-js-manifest";
import {PathPatterns} from "@romefrontend/path-match";
import {BundlerMode} from "@romefrontend/core";
import {
	AbsoluteFilePath,
	AbsoluteFilePathSet,
	TEMP_PATH,
	createAbsoluteFilePath,
} from "@romefrontend/path";
import {Consumer} from "@romefrontend/consume";
import {Dict, RequiredProps} from "@romefrontend/typescript-helpers";
import {SemverRangeNode} from "@romefrontend/codec-semver";

// Project wrapper that contains some other metadata
export type ProjectDefinition = {
	id: number;
	folder: AbsoluteFilePath;
	meta: ProjectConfigMeta;
	config: ProjectConfig;
	packages: Map<string, ManifestDefinition>;
	manifests: Map<number, ManifestDefinition>;
	children: Set<ProjectDefinition>;
	parent: undefined | ProjectDefinition;
};

// Project config objects to categorize settings
export type ProjectConfigObjects = {
	cache: {};
	resolver: {};
	compiler: {};
	bundler: {
		externals: Array<string>;
		mode: BundlerMode;
	};
	lint: {
		globals: Array<string>;
		ignore: PathPatterns;
	};
	typeCheck: {
		enabled: boolean;
		libs: AbsoluteFilePathSet;
	};
	tests: {
		ignore: PathPatterns;
	};
	develop: {
		serveStatic: boolean;
	};
	vcs: {
		root: AbsoluteFilePath;
	};
	files: {
		assetExtensions: Array<string>;
		maxSize: number;
		vendorPath: AbsoluteFilePath;
	};
	dependencies: {
		enabled: boolean;
	};
	targets: Map<string, ProjectConfigTarget>;
};

export type ProjectConfigCategoriesWithIgnore = "tests" | "lint";

export type ProjectConfigTarget = {
	constraints: Array<string>;
};

// This is a project config that contains only things that can be JSON serializable
// This is used to transport and reserialize projects in workers
export type ProjectConfigJSON = ProjectConfigJSONObjectReducer<ProjectConfigBase> & {
	[ObjectKey in keyof ProjectConfigObjects]: ProjectConfigJSONPropertyReducer<
		ProjectConfigObjects[ObjectKey]
	>
};

// Weird way to get the value type from a map
// rome-ignore lint/js/noExplicitAny
type MapValue<T extends Map<string, any>> = NonNullable<ReturnType<T["get"]>>;

// Turn any file paths into strings
// Turn maps into objects
// TODO maybe add path patterns
// rome-ignore lint/js/noExplicitAny
type ProjectConfigJSONPropertyReducer<Type> = Type extends AbsoluteFilePath
	? string
	: Type extends Array<AbsoluteFilePath>
		? Array<string>
		: Type extends AbsoluteFilePathSet
			? Array<string>
			: Type extends Map<string, any>
				? Dict<MapValue<Type>>
				: Type extends Dict<any>
					? ProjectConfigJSONObjectReducer<Type>
					: Type;

type ProjectConfigJSONObjectReducer<Obj> = {
	[PropertyKey in keyof Obj]: ProjectConfigJSONPropertyReducer<Obj[PropertyKey]>
};

// Base of a project config without any objects
type ProjectConfigBase = {
	name: string;
	root: boolean;
	version: undefined | SemverRangeNode;
};

// Data structure we pass around when normalizing and merging project configs
export type PartialProjectConfig = Partial<ProjectConfigBase> & {
	[Key in keyof ProjectConfigObjects]: PartialProjectValue<
		ProjectConfigObjects[Key]
	>
};

// rome-ignore lint/js/noExplicitAny
type PartialProjectValue<Type> = Type extends Map<string, any>
	? Type
	: Partial<Type>;

export type ProjectConfigMeta = {
	projectFolder: undefined | AbsoluteFilePath;
	configPath: undefined | AbsoluteFilePath;
	configHashes: Array<string>;
	configDependencies: AbsoluteFilePathSet;
	consumer: undefined | Consumer;
	configSourceSubKey: undefined | string;
	consumersChain: Array<Consumer>;
};

export type ProjectConfigMetaHard = RequiredProps<
	ProjectConfigMeta,
	"consumer" | "projectFolder" | "configPath"
>;

// Final project config
export type ProjectConfig = ProjectConfigBase & ProjectConfigObjects;

export function createDefaultProjectConfigMeta(): ProjectConfigMeta {
	return {
		projectFolder: undefined,
		configPath: undefined,
		configHashes: [],
		configDependencies: new AbsoluteFilePathSet(),
		consumer: undefined,
		configSourceSubKey: undefined,
		consumersChain: [],
	};
}

export function createDefaultProjectConfig(): ProjectConfig {
	return {
		name: "unknown",
		root: false,
		version: undefined,
		cache: {},
		develop: {
			serveStatic: true,
		},
		bundler: {
			externals: [],
			mode: "modern",
		},
		compiler: {},
		resolver: {},
		typeCheck: {
			enabled: false,
			// Maybe this needs to be cloned...?
			libs: new AbsoluteFilePathSet(),
		},
		dependencies: {
			enabled: false,
		},
		lint: {
			ignore: [],
			globals: [],
		},
		tests: {
			ignore: [],
		},
		vcs: {
			root: createAbsoluteFilePath("/"),
		},
		files: {
			vendorPath: TEMP_PATH.append(`rome-remote`),
			assetExtensions: [],
			maxSize: 40_000_000, // 40 megabytes
		},
		targets: new Map(),
	};
}
