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
import * as config from "./commands/config";
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
import * as recover from "./commands/recover";
import * as cache from "./commands/cache";

// Hidden commands, useful for internal debugging but not much else
import _evict from "./commands/_evict";
import _moduleSignature from "./commands/_moduleSignature";
import _projectDump from "./commands/_projectDump";

//
import {UnknownObject} from "@internal/typescript-helpers";
import ServerRequest from "./ServerRequest";
import {JSONPropertyValue} from "@internal/codec-json";
import {SharedCommand} from "../common/commands";
import {DiagnosticsPrinter} from "@internal/cli-diagnostics";
import {StaticMarkup} from "@internal/markup";
import init from "@internal/core/server/commands/init";

export type ServerCommandReturn = undefined | Promise<JSONPropertyValue>;

export type ServerCommand<Flags extends UnknownObject> = SharedCommand<
	ServerRequest,
	Flags,
	ServerCommandReturn
>;

export function createServerCommand<Flags extends UnknownObject>(
	cmd: ServerCommand<Flags>,
): ServerCommand<Flags> {
	return cmd;
}

export async function chainCommands(
	req: ServerRequest,
	fns: Array<{
		title: StaticMarkup;
		progress: StaticMarkup;
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
serverCommands.set("_evict", _evict);
serverCommands.set("_moduleSignature", _moduleSignature);
serverCommands.set("_projectDump", _projectDump);
serverCommands.set("analyzeDependencies", analyzeDependencies);
serverCommands.set("bundle", bundle);
serverCommands.set("cache dir", cache.dir);
serverCommands.set("cache clear", cache.clear);
serverCommands.set("check", check);
serverCommands.set("ci", ci);
serverCommands.set("compile", compile);
serverCommands.set("config location", config.location);
serverCommands.set("config disable", config.disable);
serverCommands.set("config enable", config.enable);
serverCommands.set("config push", config.push);
serverCommands.set("config set", config.set);
serverCommands.set("config set-directory", config.setDirectory);
serverCommands.set("develop", develop);
serverCommands.set("format", format);
serverCommands.set("init", init);
serverCommands.set("lsp", lsp);
serverCommands.set("json", json);
serverCommands.set("noop", noop);
serverCommands.set("parse", parse);
serverCommands.set("publish", publish);
serverCommands.set("resolve", resolve);
serverCommands.set("run", run);
serverCommands.set("stop", stop);
serverCommands.set("status", status);
serverCommands.set("test", test);
serverCommands.set("recover apply", recover.apply);
serverCommands.set("recover clear", recover.clear);
serverCommands.set("recover diff", recover.diff);
serverCommands.set("recover dir", recover.dir);
serverCommands.set("recover list", recover.list);
serverCommands.set("recover pop", recover.pop);
