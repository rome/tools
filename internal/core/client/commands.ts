/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import start from "./commands/start";
import develop from "./commands/develop";
import stop from "./commands/stop";
import run from "./commands/run";
import restart from "./commands/restart";
import status from "./commands/status";
import lsp from "./commands/lsp";

//
import {UnknownObject} from "@internal/typescript-helpers";
import ClientRequest from "./ClientRequest";
import {SharedCommand} from "../common/commands";
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

// rome-ignore lint/ts/noExplicitAny
export const localCommands: Map<string, LocalCommand<any>> = new Map();
localCommands.set("start", start);
localCommands.set("develop", develop);
localCommands.set("stop", stop);
localCommands.set("run", run);
localCommands.set("restart", restart);
localCommands.set("status", status);
localCommands.set("lsp", lsp);
