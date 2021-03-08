/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Diagnostic} from "@internal/diagnostics";
import {SourceMapGenerator} from "@internal/codec-source-map";
import {
	AbsoluteFilePath,
	AbsoluteFilePathSet,
	Path,
	RelativePath,
	RelativePathMap,
} from "@internal/path";
import {ResolverOptions} from "../../server/fs/Resolver";
import BundleRequest from "@internal/core/server/bundler/BundleRequest";
import Bundler from "@internal/core/server/bundler/Bundler";
import {FSReadStream} from "@internal/fs";
import {ProjectDefinition} from "@internal/project";
import {ManifestDefinition} from "@internal/codec-js-manifest";
import {WorkerCompileResult} from "@internal/core/worker/types";
import {Event} from "@internal/events";
import {Resource} from "@internal/resources";

export type BundlerConfig = {
	basePath: Path;
	inlineSourceMap: boolean;
	cwd: AbsoluteFilePath;
	resolver: ResolverOptions;
};

export type AssembledBundle = Array<[0, string] | [1, AbsoluteFilePath]>;

export type BundleAssets = RelativePathMap<BundleAsset>;

export type BundleAsset = {
	etag: string;
	content: () => Promise<string | ArrayBufferView | FSReadStream>;
};

export type BundleRequestResult = {
	request: BundleRequest;
	cached: boolean;
	diagnostics: Diagnostic[];
	assembled: AssembledBundle;
	sourceMap: SourceMapGenerator;
	assets: BundleAssets;
	etag: string;
};

export type BundleBuddyStats = BundleBuddyGraphNode[];

export type BundleBuddyGraphNode = {
	source: string;
	target: string | undefined;
};
export type BundlerEntryResolution = {
	setVersion: undefined | string;
	project: ProjectDefinition;
	manifestDef: undefined | ManifestDefinition;
	resolvedEntry: AbsoluteFilePath;
};

export type BundleCompileResult = WorkerCompileResult & {
	asset?: {
		path: RelativePath;
		value: BundleAsset;
	};
};

export type BundleWatcher = {
	subscription: Resource;
	diagnosticsEvent: Event<Diagnostic[], void>;
	changeEvent: Event<AbsoluteFilePathSet, void>;
	filesEvent: Event<BundleWatcherFiles, void>;
};

export type BundleWatcherFiles = RelativePathMap<undefined | BundlerFile>;

export type BundlerFile = BundleAsset & {
	kind: "asset" | "entry" | "sourcemap" | "stats" | "manifest" | "file";
};

export type BundlerFiles = RelativePathMap<BundlerFile>;

export type BundleResultBundle = {
	etag: string;
	sourceMap: {
		path: RelativePath;
		map: SourceMapGenerator;
	};
	js: {
		assembled: AssembledBundle;
		path: RelativePath;
		content: () => Promise<string>;
	};
};

export type BundleResult = {
	bundler: Bundler;
	files: BundlerFiles;
	bundles: BundleResultBundle[];
	entry: BundleResultBundle;
};
