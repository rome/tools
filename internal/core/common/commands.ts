/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Consumer} from "@internal/consume";
import {UnknownObject} from "@internal/typescript-helpers";
import {StaticMarkup} from "@internal/markup";
import {Examples} from "@internal/cli-flags";

// List of valid command names, includes both server and client commands
export type CommandName =
	| "_evict"
	| "_moduleSignature"
	| "_projectDump"
	| "auto-config"
	| "analyzeDependencies"
	| "bundle"
	| "cache dir"
	| "cache clear"
	| "check"
	| "ci"
	| "compile"
	| "config location"
	| "config disable"
	| "config enable"
	| "config push"
	| "config set"
	| "config set-directory"
	| "develop"
	| "format"
	| "init"
	| "lsp"
	| "json"
	| "noop"
	| "parse"
	| "publish"
	| "resolve"
	| "restart"
	| "start"
	| "run"
	| "stop"
	| "status"
	| "test"
	| "recover apply"
	| "recover clear"
	| "recover diff"
	| "recover dir"
	| "recover list"
	| "recover pop";

export interface SharedCommand<Req, Flags extends UnknownObject, Ret> {
	category: string;
	description: StaticMarkup;
	defineFlags: (c: Consumer) => Flags;
	usage: StaticMarkup;
	examples: Examples;
	hidden?: boolean;
	ignoreFlags?: string[];
	allowRequestFlags?: Array<"review" | "watch">;
	callback: (req: Req, flags: Flags) => Ret;
}

export const commandCategories = {
	PROCESS_MANAGEMENT: "Process Management",
	CODE_QUALITY: "Code Quality",
	SOURCE_CODE: "Source Code",
	PROJECT_MANAGEMENT: "Project Management",
	SOURCE_CONTROL: "Source Control",
	INTERNAL: "Internal",
};
