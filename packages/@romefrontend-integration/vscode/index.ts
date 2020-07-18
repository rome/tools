/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import languageClient = require("vscode-languageclient");
import path = require("path");
import vscode = require("vscode");
import fs = require("fs");
import os = require("os");

let client: languageClient.LanguageClient;

function crawl(root: string): Iterable<string> {
	return {
		[Symbol.iterator]() {
			return {
				next() {
					const value = root;
					root = path.dirname(value);

					return {
						value,
						done: root === "." || root === "/",
					};
				},
			};
		},
	};
}

async function tryChain(
	root: string,
	suffix: string,
): Promise<undefined | string> {
	for (const dir of crawl(root)) {
		const possible = path.join(dir, suffix);

		try {
			await fs.promises.access(possible);
			return possible;
		} catch (err) {
		}
	}
	return undefined;
}

async function tryManifest(root: string): Promise<undefined | string> {
	for (const dir of crawl(root)) {
		const manifestLoc = path.join(dir, "package.json");

		try {
			const content = await fs.promises.readFile(manifestLoc, "utf8");
			const json = JSON.parse(content);
			if (json.romeLSPBin) {
				return String(path.resolve(dir, json.romeLSPBin));
			}
		} catch (err) {
			if (err instanceof SyntaxError || err.code === "ENOENT") {
				continue;
			} else {
				throw err;
			}
		}
	}
	return undefined;
}

async function getRomeLocation(): Promise<
	| undefined
	| {
			path: string;
			env: {
				[key: string]: string;
			};
		}
> {
	const {workspaceFolders} = vscode.workspace;
	if (workspaceFolders === undefined) {
		return undefined;
	}

	// Find relative to workspace directories
	for (const {uri} of workspaceFolders) {
		if (uri.scheme === "file") {
			const manifest = await tryManifest(uri.path);
			if (manifest !== undefined) {
				return {path: manifest, env: {}};
			}

			const nodeModules = await tryChain(
				uri.path,
				"node_modules/rome/bin/rome/index.js",
			);
			if (nodeModules !== undefined) {
				return {path: nodeModules, env: {}};
			}
		}
	}

	// Find development build
	try {
		const possible = path.join(os.tmpdir(), "rome-dev", "index.js");
		await fs.promises.access(possible);
		return {path: possible, env: {ROME_DEV: "1"}};
	} catch (err) {
	}

	return undefined;
}

export async function activate() {
	const outputChannel = vscode.window.createOutputChannel("Rome");

	function log(message: string) {
		outputChannel.appendLine(message);
	}

	let res = await getRomeLocation();

	// If no romePath was found then watch workspace folders until we find one
	if (res === undefined) {
		log(
			"No Rome path found. Waiting for workspace folder changes before trying again",
		);

		await new Promise((resolve) => {
			const event = vscode.workspace.onDidChangeWorkspaceFolders(() => {
				getRomeLocation().then((_res) => {
					if (_res !== undefined) {
						res = _res;
						event.dispose();
						resolve();
					}
				});
			});
		});
	}

	if (res === undefined) {
		throw new Error("Should have been defined");
	}

	const {path: romePath, env} = res;

	log(`Discovered Rome path ${romePath}`);

	let serverOptions: languageClient.ServerOptions = {
		module: romePath,
		args: ["lsp"],
		transport: languageClient.TransportKind.stdio,
		options: {
			env,
		},
	};

	let clientOptions: languageClient.LanguageClientOptions = {
		outputChannel,
		documentSelector: [
			{scheme: "file", language: "javascript"},
			{scheme: "file", language: "javascriptreact"},
			{scheme: "file", language: "typescript"},
			{scheme: "file", language: "typescriptreact"},
			{scheme: "file", language: "json"},
		],
	};

	client = new languageClient.LanguageClient(
		"rome",
		"Rome",
		serverOptions,
		clientOptions,
	);

	client.start();
}

export function deactivate(): Thenable<void> | undefined {
	if (!client) {
		return undefined;
	}
	return client.stop();
}
