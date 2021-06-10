/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AbsoluteFilePath} from "@internal/path";
import {spawn} from "@internal/child-process";

export function extractFileList(out: string): string[] {
	const lines = out.trim().split("\n");

	const files: string[] = [];

	for (const line of lines) {
		const match = line.trim().match(/^(?:[AM]|\?\?)\s+(.*?)$/);
		if (match != null) {
			files.push(match[1]);
		}
	}

	return files;
}

export class VCSClient {
	constructor(root: AbsoluteFilePath, baseBranch: string) {
		this.root = root;
		this.baseBranch = baseBranch;
	}

	public root: AbsoluteFilePath;
	public baseBranch: string;

	public getModifiedFiles(branch: string): Promise<string[]> {
		throw new Error("unimplemented");
	}

	public getUncommittedFiles(): Promise<string[]> {
		throw new Error("unimplemented");
	}

	public getDefaultBranch(): string {
		throw new Error("unimplemented");
	}
}

class GitVCSClient extends VCSClient {
	constructor(root: AbsoluteFilePath, baseBranch: string) {
		super(root, baseBranch);
	}

	public getDefaultBranch(): string {
		return this.baseBranch;
	}

	public async getUncommittedFiles(): Promise<string[]> {
		try {
			const stdout = (await spawn(
				"git",
				["status", "--short"],
				{cwd: this.root},
			).waitSuccess()).getOutput(true, false);
			return extractFileList(stdout);
		} catch (_) {
			throw new Error("Unexpected error when checking for uncommitted files");
		}
	}

	public async getModifiedFiles(): Promise<string[]> {
		try {
			const stdout = (await spawn(
				"git",
				["diff", "--name-status", this.baseBranch],
				{cwd: this.root},
			).waitSuccess()).getOutput(true, false);
			return extractFileList(stdout);
		} catch (_) {
			// TODO: temporary
			throw new Error(
				`Unexpected error when checking for modified files on the "${this.baseBranch}" branch \n\n\ ${_.message}`,
			);
		}
	}
}

export async function getVCSClient(
	root: AbsoluteFilePath,
	baseBranch: string,
): Promise<undefined | VCSClient> {
	if (await root.append(".git").exists()) {
		return new GitVCSClient(root, baseBranch);
	}

	return undefined;
}
