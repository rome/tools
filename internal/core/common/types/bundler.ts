/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Diagnostics} from "@internal/diagnostics";
import {SourceMapGenerator} from "@internal/codec-source-map";
import {AbsoluteFilePath} from "@internal/path";
import {ResolverOptions} from "../../server/fs/Resolver";
import BundleRequest from "@internal/core/server/bundler/BundleRequest";
import Bundler from "@internal/core/server/bundler/Bundler";

export type BundlerConfig = {
	inlineSourceMap: boolean;
	cwd: AbsoluteFilePath;
	resolver: ResolverOptions;
};

export type AssembledBundle = Array<[0, string] | [1, AbsoluteFilePath]>;

export type BundleRequestResult = {
	request: BundleRequest;
	cached: boolean;
	diagnostics: Diagnostics;
	assembled: AssembledBundle;
	sourceMap: SourceMapGenerator;
	assets: Map<string, Buffer>;
};

export type BundleBuddyStats = Array<BundleBuddyGraphNode>;

export type BundleBuddyGraphNode = {
	source: string;
	target: string | undefined;
};

export type BundlerFiles = Map<
	string,
	{
		kind: "asset" | "entry" | "sourcemap" | "stats" | "manifest" | "file";
		content: () => string | Buffer;
	}
>;

export type BundleResultBundle = {
	sourceMap: {
		path: string;
		map: SourceMapGenerator;
	};
	js: {
		assembled: AssembledBundle;
		path: string;
		content: () => string;
	};
};

export type BundleResult = {
	bundler: Bundler;
	files: BundlerFiles;
	bundles: Array<BundleResultBundle>;
	entry: BundleResultBundle;
};
