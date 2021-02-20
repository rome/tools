/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import start from "./commands/start";
import stop from "./commands/stop";
import run from "./commands/run";
import restart from "./commands/restart";
import status from "./commands/status";
import lsp from "./commands/lsp";
import init from "./commands/init";
import autoConfig from "./commands/autoConfig";

//
import {UnknownObject} from "@internal/typescript-helpers";
import ClientRequest from "./ClientRequest";
import {CommandName, SharedCommand} from "../common/commands";
import {ServerQueryResponse} from "@internal/core";

export type LocalCommand<Flags extends UnknownObject> = SharedCommand<
	ClientRequest,
	Flags,
	Promise<boolean | ServerQueryResponse>
>;

export function createLocalCommand<Flags extends UnknownObject>(
	cmd: LocalCommand<Flags>,
): LocalCommand<Flags> {
	return cmd;
}

// rome-ignore lint/ts/noExplicitAny: future cleanup
export const localCommands: Map<CommandName, LocalCommand<any>> = new Map();
localCommands.set("start", start);
localCommands.set("stop", stop);
localCommands.set("run", run);
localCommands.set("restart", restart);
localCommands.set("status", status);
localCommands.set("lsp", lsp);
localCommands.set("init", init);
localCommands.set("auto-config", autoConfig);
