/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ServerRequest} from "@internal/core";
import {commandCategories} from "../../common/commands";
import {createServerCommand} from "../commands";
import {markup} from "@internal/markup";

export const dir = createServerCommand({
	category: commandCategories.SOURCE_CODE,
	description: markup`show the location of the cache directory`,
	usage: "",
	examples: [],
	defineFlags() {
		return {};
	},
	async callback(req: ServerRequest): Promise<void> {
		req.reporter.log(markup`${req.server.cache.getDirectory()}`);
	},
});

export const clear = createServerCommand({
	category: commandCategories.SOURCE_CODE,
	description: markup`clear the cache directory`,
	usage: "",
	examples: [],
	defineFlags() {
		return {};
	},
	async callback(req: ServerRequest): Promise<void> {
		await req.server.cache.clear();
		req.reporter.success(
			markup`Cleared cache at <emphasis>${req.server.cache.getDirectory()}</emphasis>`,
		);
	},
});
