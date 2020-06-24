/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ServerRequest} from "@romejs/core";
import {commandCategories} from "../../common/commands";
import {createServerCommand} from "../commands";

export default createServerCommand({
	description: "evict a file from the memory cache",
	category: commandCategories.INTERNAL,
	usage: "",
	examples: [],
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
			await server.fileAllocator.evict(client.flags.cwd.resolve(file));
			reporter.success(`Evicted ${file}`);
		}

		reporter.info(`Evicted ${String(files.length)} files`);
	},
});
