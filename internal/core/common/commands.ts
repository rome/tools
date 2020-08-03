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

export interface SharedCommand<Req, Flags extends UnknownObject, Ret> {
	category: string;
	description: StaticMarkup;
	defineFlags: (c: Consumer) => Flags;
	usage: string;
	examples: Examples;
	hidden?: boolean;
	ignoreFlags?: Array<string>;
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
