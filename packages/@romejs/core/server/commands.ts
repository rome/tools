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
import lint from "./commands/lint";
import ci from "./commands/ci";
import test from "./commands/test";
import noop from "./commands/noop";

// Hidden commands, useful for internal debugging but not much else
import _evict from "./commands/_evict";
import _moduleSignature from "./commands/_moduleSignature";
import _projectDump from "./commands/_projectDump";

//
import {Dict} from "@romejs/typescript-helpers";
import ServerRequest from "./ServerRequest";
import {ClientFlags, ClientRequestFlags} from "../common/types/client";
import {JSONPropertyValue} from "@romejs/codec-json";
import {SharedCommand} from "../common/commands";

export type ServerCommand<Flags extends Dict<unknown>> = SharedCommand<Flags> & {
	overrideClientFlags?: Partial<ClientFlags>;
	overrideRequestFlags?: Partial<ClientRequestFlags>;
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

// rome-ignore lint/js/noExplicitAny
export const serverCommands: Map<string, ServerCommand<any>> = new Map();
serverCommands.set("test", test);
serverCommands.set("lint", lint);
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
