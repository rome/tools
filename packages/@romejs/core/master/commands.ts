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

// Hidden commands, useful for internal debugging but not much else
import _evict from "./commands/_evict";
import _moduleSignature from "./commands/_moduleSignature";
import _noop from "./commands/_noop";

//
import {Dict} from "@romejs/typescript-helpers";
import MasterRequest from "./MasterRequest";
import {ClientFlags, ClientRequestFlags} from "../common/types/client";
import {JSONPropertyValue} from "@romejs/codec-json";
import {SharedCommand} from "../common/commands";

export type MasterCommand<Flags extends Dict<unknown>> = SharedCommand<Flags> & {
	overrideClientFlags?: Partial<ClientFlags>;
	overrideRequestFlags?: Partial<ClientRequestFlags>;
	callback: (
		req: MasterRequest,
		commandFlags: Flags,
	) => undefined | Promise<JSONPropertyValue>;
};

export function createMasterCommand<Flags extends Dict<unknown>>(
	cmd: MasterCommand<Flags>,
): MasterCommand<Flags> {
	return cmd;
}

// rome-ignore lint/javascript/noExplicitAny
export const masterCommands: Map<string, MasterCommand<any>> = new Map();
masterCommands.set("test", test);
masterCommands.set("lint", lint);
masterCommands.set("config", config);
masterCommands.set("bundle", bundle);
masterCommands.set("parse", parse);
masterCommands.set("analyzeDependencies", analyzeDependencies);
masterCommands.set("resolve", resolve);
masterCommands.set("compile", compile);
masterCommands.set("stop", stop);
masterCommands.set("status", status);
masterCommands.set("run", run);
masterCommands.set("publish", publish);
masterCommands.set("ci", ci);
masterCommands.set("develop", develop);
masterCommands.set("format", format);
masterCommands.set("lsp", lsp);
masterCommands.set("_evict", _evict);
masterCommands.set("_moduleSignature", _moduleSignature);
masterCommands.set("_noop", _noop);
