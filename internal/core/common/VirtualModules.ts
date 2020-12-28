/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {modules} from "./virtual-modules";
import {
	AbsoluteFilePath,
	AbsoluteFilePathMap,
	createAbsoluteFilePath,
} from "@internal/path";
import {FSStats, createFakeStats} from "@internal/fs";

export type VirtualModulesMap = Map<
	string,
	Map<
		string,
		{
			mtime: number;
			content: string;
		}
	>
>;

export type VirtualModuleStatMap = AbsoluteFilePathMap<{
	stats: FSStats;
	content: undefined | string;
}>;

export default class VirtualModules {
	constructor() {
		// A NULL character isn't allowed in Windows or Unix file paths
		// We abuse that to distinguish and represent virtual paths
		this.nullAbsolute = createAbsoluteFilePath("/\0");

		this.statMap = new AbsoluteFilePathMap();
	}

	private nullAbsolute: AbsoluteFilePath;
	private statMap: VirtualModuleStatMap;

	public getMockDirectory(): AbsoluteFilePath {
		return this.nullAbsolute;
	}

	public getFakeStats(path: AbsoluteFilePath): FSStats {
		return this.statMap.assert(path).stats;
	}

	public init() {
		const {statMap} = this;

		for (const [moduleName, files] of modules) {
			for (const [subpath, {content, mtime}] of files) {
				statMap.set(
					this.nullAbsolute.append(moduleName).append(subpath),
					{
						content,
						stats: createFakeStats({
							type: "file",
							date: new Date(Math.round(mtime)),
							size: BigInt(content.length),
						}),
					},
				);
			}
		}

		// Add directories
		for (const [path, {stats: fileStats}] of statMap) {
			if (!fileStats.isFile()) {
				continue;
			}

			for (const directory of path.getParent().getChain()) {
				if (directory.getBasename() === "\0") {
					// Reached the "root"
					break;
				}

				const directoryEntry = statMap.get(directory);
				if (
					directoryEntry !== undefined &&
					fileStats.mtime < directoryEntry.stats.mtime
				) {
					continue;
				}

				statMap.set(
					directory,
					{
						content: undefined,
						stats: createFakeStats({
							type: "directory",
							date: fileStats.mtime,
							size: 0n,
						}),
					},
				);
			}
		}
	}

	public getStatMap(): VirtualModuleStatMap {
		return this.statMap;
	}

	public isVirtualPath(path: AbsoluteFilePath): boolean {
		const segments = path.getSegments();
		return segments[0] === "" && segments[1] === "\0";
	}

	public getPossibleVirtualFileContents(
		path: AbsoluteFilePath,
	): undefined | string {
		if (this.isVirtualPath(path)) {
			const entry = this.statMap.assert(path);
			return entry.content;
		}

		return undefined;
	}

	public resolvePossibleVirtualModuleName(
		name: string,
	): undefined | AbsoluteFilePath {
		if (modules.has(name)) {
			return this.nullAbsolute.append(name);
		} else {
			return undefined;
		}
	}
}
