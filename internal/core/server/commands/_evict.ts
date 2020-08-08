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

export default createServerCommand({
	description: markup`evict a file from the memory cache`,
	category: commandCategories.INTERNAL,
	usage: "",
	examples: [],
	hidden: true,
	defineFlags() {
		return {};
	},
	async callback(req: ServerRequest): Promise<void> {
		const {
			server,
			reporter,
			client,
			query: {args},
		} = req;

		const files =
			args.length === 0 ? server.fileAllocator.getAllOwnedFilenames() : args;

		for (const file of files) {
			const path = client.flags.cwd.resolve(file);
			await server.fileAllocator.evict(path, markup`client request`);
			reporter.success(markup`Evicted ${path}`);
		}

		reporter.info(markup`Evicted ${String(files.length)} files`);
	},
});
