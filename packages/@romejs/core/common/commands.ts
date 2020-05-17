/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Consumer} from "@romejs/consume";
import {Dict} from "@romejs/typescript-helpers";

export type SharedCommand<Flags extends Dict<unknown>> = {
	category: string;
	description: string;
	defineFlags: (c: Consumer) => Flags;
	usage: string;
	examples: Array<{
		description: string;
		command: string;
	}>;
	ignoreFlags?: Array<string>;
	allowRequestFlags?: Array<"review" | "watch">;
};

export const commandCategories = {
	PROCESS_MANAGEMENT: "Process Management",
	CODE_QUALITY: "Code Quality",
	SOURCE_CODE: "Source Code",
	PROJECT_MANAGEMENT: "Project Management",
	SOURCE_CONTROL: "Source Control",
	INTERNAL: "Internal",
};
