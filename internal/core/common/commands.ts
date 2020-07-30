/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Consumer} from "@romefrontend/consume";
import {Dict} from "@romefrontend/typescript-helpers";
import {Markup} from "@romefrontend/markup";
import {Examples} from "@romefrontend/cli-flags/Parser";

export type SharedCommand<Flags extends Dict<unknown>> = {
	category: string;
	description: Markup;
	defineFlags: (c: Consumer) => Flags;
	usage: string;
	examples: Examples;
	hidden?: boolean;
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
