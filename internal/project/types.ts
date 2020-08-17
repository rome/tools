/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ManifestDefinition} from "@internal/codec-js-manifest";
import {PathPatterns} from "@internal/path-match";
import {
	AbsoluteFilePath,
	AbsoluteFilePathSet,
	TEMP_PATH,
	createAbsoluteFilePath,
} from "@internal/path";
import {Consumer} from "@internal/consume";
import {RequiredProps} from "@internal/typescript-helpers";
import {SemverRangeNode} from "@internal/codec-semver";

// Project wrapper that contains some other metadata
export type ProjectDefinition = {
	id: number;
	directory: AbsoluteFilePath;
	meta: ProjectConfigMeta;
	config: ProjectConfig;
	packages: Map<string, ManifestDefinition>;
	manifests: Map<number, ManifestDefinition>;
	children: Set<ProjectDefinition>;
	parent: undefined | ProjectDefinition;
	initialized: boolean;
};

// Project config objects to categorize settings
export type ProjectConfigObjects = {
	cache: {};
	resolver: {};
	compiler: {};
	bundler: {
		externals: Array<string>;
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

// rome-ignore lint/ts/noExplicitAny
type PartialProjectValue<Type> = Type extends Map<string, any>
	? Type
	: Partial<Type>;

export type ProjectConfigMeta = {
	projectDirectory: undefined | AbsoluteFilePath;
	configPath: undefined | AbsoluteFilePath;
	configHashes: Array<string>;
	configDependencies: AbsoluteFilePathSet;
	consumer: undefined | Consumer;
	configSourceSubKey: undefined | string;
	consumersChain: Array<Consumer>;
};

export type ProjectConfigMetaHard = RequiredProps<
	ProjectConfigMeta,
	"consumer" | "projectDirectory" | "configPath"
>;

// Final project config
export type ProjectConfig = ProjectConfigBase & ProjectConfigObjects;

export function createDefaultProjectConfigMeta(): ProjectConfigMeta {
	return {
		projectDirectory: undefined,
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
			vendorPath: TEMP_PATH.append("rome-remote"),
			assetExtensions: [],
			maxSize: 40_000_000, // 40 megabytes
		},
		targets: new Map(),
	};
}
