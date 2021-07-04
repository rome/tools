/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ManifestDefinition} from "@internal/codec-js-manifest";
import {PathPattern} from "@internal/path-match";
import {
	AbsoluteFilePath,
	AbsoluteFilePathSet,
	TEMP_PATH,
	createAbsoluteFilePath,
} from "@internal/path";
import {Consumer, consumeUnknown} from "@internal/consume";
import {DeepPartial, Dict} from "@internal/typescript-helpers";
import {SemverRange} from "@internal/codec-semver";
import {LintRuleName} from "@internal/compiler";
import {DIAGNOSTIC_CATEGORIES} from "@internal/diagnostics";
import {GetBrowserProps} from "@internal/browser-features";
import {PathAliasPattern} from "./aliases";

// Project wrapper that contains some other metadata
export type ProjectDefinition = {
	id: number;
	directory: AbsoluteFilePath;
	meta: ProjectConfigMeta;
	config: ProjectConfig;
	packages: Map<string, ManifestDefinition>;
	manifests: Map<number, ManifestDefinition>;
	children: Set<ProjectDefinition>;
	root: undefined | ProjectDefinition;
	parent: undefined | ProjectDefinition;
	initialized: boolean;
	partial: boolean;
};

export type InvalidLicenses = Map<
	string,
	{
		name: string;
		range: SemverRange;
	}[]
>;
export type DependenciesExceptions = {
	invalidLicenses: InvalidLicenses;
};

export type ProjectConfigPresetNames = "electron" | "cypress" | "jest";

type Enableable = {
	enabled: boolean;
};

export type IndentStyle = "tab" | "space";

// Project config objects to categorize settings
export type ProjectConfigObjects = {
	presets: ProjectConfigPresetNames[];
	cache: {};
	resolver: {};
	compiler: {};
	bundler: {
		externals: string[];
	};
	parser: {
		jsxEverywhere: boolean;
	};
	format: Enableable & {
		indentStyle: IndentStyle;
		indentSize: number;
	};
	check: {
		dependencies: boolean;
	};
	lint: Enableable & {
		globals: string[];
		ignore: PathPattern[];
		requireSuppressionExplanations: boolean;
		disabledRules: LintRuleName[];
	};
	typeCheck: Enableable & {
		libs: AbsoluteFilePathSet;
	};
	tests: {
		ignore: PathPattern[];
	};
	develop: {
		serveStatic: boolean;
	};
	vcs: {
		root: AbsoluteFilePath;
	};
	files: {
		assetExtensions: string[];
		maxSizeIgnore: PathPattern[];
		maxSize: number;
		vendorPath: AbsoluteFilePath;
	};
	dependencies: Enableable & {
		exceptions: DependenciesExceptions;
	};
	targets: Map<string, GetBrowserProps[]>;
	aliases: {
		base: AbsoluteFilePath;
		paths: [PathAliasPattern, PathAliasPattern[]][];
	};
};

export type IntegrationPrettierConfig = {
	printWidth: number;
	tabWidth: number;
	useTabs: boolean;
	semi: boolean;
	singleQuote: boolean;
};

export type IntegrationEslintConfig = {
	fix: boolean;
	extensions: string[];
	rulePaths: string[];
	globInputPaths: boolean;
};

export type ProjectConfigIntegrations = {
	eslint: Enableable & Partial<IntegrationEslintConfig>;
	typescriptChecker: Enableable;
	prettier: Enableable & Partial<IntegrationPrettierConfig>;
};

export type ProjectConfigCategoriesWithIgnore = "tests" | "lint";

// Base of a project config without any objects
type ProjectConfigBase = {
	name: string;
	root: boolean;
	version: undefined | SemverRange;
};

// Data structure we pass around when normalizing and merging project configs
export type PartialProjectConfig = Partial<ProjectConfigBase> & {
	[Key in keyof ProjectConfigObjects]: PartialProjectValue<
		ProjectConfigObjects[Key]
	>
} & {
	integrations: {
		[Key in keyof ProjectConfigIntegrations]: PartialProjectValue<
			ProjectConfigIntegrations[Key]
		>
	};
};

// rome-ignore lint/ts/noExplicitAny: future cleanup
type PartialProjectValue<Type> = Type extends Map<string, any>
	? Type
	: Partial<Type>;

export type ProjectConfigMeta = {
	projectDirectory: AbsoluteFilePath;
	configPath: AbsoluteFilePath;
	configCacheKeys: Dict<string>;
	configDependencies: AbsoluteFilePathSet;
	consumer: Consumer;
	configSourceSubKey: undefined | string;
	consumersChain: Consumer[];
};

// Final project config
export type ProjectConfig = ProjectConfigBase &
	ProjectConfigObjects & {
		integrations: ProjectConfigIntegrations;
	};

// The actual type that we allow users to specify their configuration
// Types are deliberately wider than they need to be to more accurately represent how they will be provided. ie. `string` rather than string literals
export type RawUserProjectConfig = DeepPartial<{
	name: string;
	version: string;
	root: boolean;
	extends: boolean;
	cache: {};
	resolver: {};
	compiler: {};
	bundler: {
		externals: string[];
	};
	typeChecking: Enableable & {
		libs: string[];
	};
	dependencies: Enableable & {
		exceptions: {
			invalidLicenses: {
				[key: string]: string[];
			};
		};
	};
	check: {
		dependencies: boolean;
	};
	lint: Enableable & {
		ignore: string[];
		globals: string[];
		disabledRules: string[];
		requireSuppressionExplanations: boolean;
	};
	format: {
		enabled: boolean;
		indentStyle: string;
		indentSize: number;
	};
	tests: {
		ignore: string[];
	};
	parser: {
		jsxEverywhere: boolean;
	};
	develop: {
		serveStatic: boolean;
	};
	files: {
		vendorPath: string;
		maxSize: number;
		maxSizeIgnore: string[];
		assetExtensions: string[];
	};
	vcs: {
		root: string;
	};
	targets: {
		[key: string]: {
			constraints: string[];
		};
	};
	integrations: {
		eslint: Enableable;
		typescriptChecker: Enableable;
		prettier: Enableable & Partial<IntegrationPrettierConfig>;
	};
	aliases: {
		base: string;
		paths: {
			[key: string]: string;
		};
	};
}>;

export function createMockProjectConfigMeta(
	projectDirectory: AbsoluteFilePath,
): ProjectConfigMeta {
	return {
		projectDirectory,
		configPath: projectDirectory.append("package.json"),
		configCacheKeys: {},
		configDependencies: new AbsoluteFilePathSet(),
		consumer: consumeUnknown({}, DIAGNOSTIC_CATEGORIES.parse, "json"),
		configSourceSubKey: undefined,
		consumersChain: [],
	};
}

export function createDefaultProjectConfig(): ProjectConfig {
	return {
		name: "unknown",
		root: false,
		version: undefined,
		presets: [],
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
		check: {
			dependencies: true,
		},
		parser: {
			jsxEverywhere: false,
		},
		dependencies: {
			enabled: false,
			exceptions: {
				invalidLicenses: new Map(),
			},
		},
		format: {
			enabled: true,
			indentStyle: "tab",
			indentSize: 1,
		},
		lint: {
			enabled: true,
			ignore: [],
			globals: [],
			requireSuppressionExplanations: true,
			disabledRules: [],
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
			maxSizeIgnore: [],
			maxSize: 40_000_000, // 40 megabytes
		},
		targets: new Map(),
		integrations: {
			eslint: {
				enabled: false,
			},
			typescriptChecker: {
				enabled: false,
			},
			prettier: {
				enabled: false,
			},
		},
		aliases: {
			base: createAbsoluteFilePath("/"),
			paths: [],
		},
	};
}
