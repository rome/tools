/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Diagnostics} from "@romejs/diagnostics";
import {SourceMapGenerator} from "@romejs/codec-source-map";
import {AbsoluteFilePath} from "@romejs/path";
import {ResolverOptions} from "../../master/fs/Resolver";

export type BundlerConfig = {
	inlineSourceMap: boolean;
	cwd: AbsoluteFilePath;
	resolver: ResolverOptions;
};

export type BundlerMode = "modern" | "legacy";

export const BUNDLER_MODES: Array<BundlerMode> = ["modern", "legacy"];

export type BundleRequestResult = {
	cached: boolean;
	diagnostics: Diagnostics;
	content: string;
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
		path: string;
		content: string;
	};
};

export type BundleResult = {
	files: BundlerFiles;
	bundles: Array<BundleResultBundle>;
	entry: BundleResultBundle;
};
