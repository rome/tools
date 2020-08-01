/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import run from "./commands/run";
import publish from "./commands/publish";
import status from "./commands/status";
import stop from "./commands/stop";
import develop from "./commands/develop";
import config from "./commands/config";
import compile from "./commands/compile";
import resolve from "./commands/resolve";
import analyzeDependencies from "./commands/analyzeDependencies";
import parse from "./commands/parse";
import bundle from "./commands/bundle";
import format from "./commands/format";
import lsp from "./commands/lsp";
import check from "./commands/check";
import ci from "./commands/ci";
import test from "./commands/test";
import noop from "./commands/noop";
import json from "./commands/json";
import recover from "./commands/recover";

// Hidden commands, useful for internal debugging but not much else
import _evict from "./commands/_evict";
import _moduleSignature from "./commands/_moduleSignature";
import _projectDump from "./commands/_projectDump";

//
import {Dict} from "@internal/typescript-helpers";
import ServerRequest from "./ServerRequest";
import {JSONPropertyValue} from "@internal/codec-json";
import {SharedCommand} from "../common/commands";
import {DiagnosticsPrinter} from "@internal/cli-diagnostics";
import {Markup} from "@internal/markup";
import init from "@internal/core/server/commands/init";

export type ServerCommand<Flags extends Dict<unknown>> = SharedCommand<Flags> & {
	callback: (
		req: ServerRequest,
		commandFlags: Flags,
	) => undefined | Promise<JSONPropertyValue>;
};

export function createServerCommand<Flags extends Dict<unknown>>(
	cmd: ServerCommand<Flags>,
): ServerCommand<Flags> {
	return cmd;
}

export async function chainCommands(
	req: ServerRequest,
	fns: Array<{
		title: Markup;
		progress: Markup;
		callback: () => Promise<void>;
	}>,
): Promise<void> {
	let printer: undefined | DiagnosticsPrinter;

	await req.reporter.steps(
		fns.map(({callback, progress, title}) => {
			return {
				clear: true,
				message: progress,
				async callback() {
					try {
						await callback();
					} catch (err) {
						if (err instanceof DiagnosticsPrinter) {
							if (printer === undefined) {
								printer = req.createDiagnosticsPrinter();
							}
							printer.inject(title, err);
						} else {
							throw err;
						}
					}
				},
			};
		}),
	);

	if (printer !== undefined) {
		throw printer;
	}
}

// rome-ignore lint/ts/noExplicitAny
export const serverCommands: Map<string, ServerCommand<any>> = new Map();
serverCommands.set("test", test);
serverCommands.set("check", check);
serverCommands.set("config", config);
serverCommands.set("bundle", bundle);
serverCommands.set("parse", parse);
serverCommands.set("analyzeDependencies", analyzeDependencies);
serverCommands.set("resolve", resolve);
serverCommands.set("compile", compile);
serverCommands.set("stop", stop);
serverCommands.set("status", status);
serverCommands.set("run", run);
serverCommands.set("publish", publish);
serverCommands.set("ci", ci);
serverCommands.set("develop", develop);
serverCommands.set("format", format);
serverCommands.set("lsp", lsp);
serverCommands.set("_evict", _evict);
serverCommands.set("_moduleSignature", _moduleSignature);
serverCommands.set("noop", noop);
serverCommands.set("_projectDump", _projectDump);
serverCommands.set("json", json);
serverCommands.set("init", init);
serverCommands.set("recover", recover);
