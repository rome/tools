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

		const paths =
			args.length === 0 ? server.fileAllocator.getAllOwnedFilenames() : args;

		for (const path of paths) {
			const resolved = client.flags.cwd.resolve(path);
			await server.fileAllocator.evict(resolved, markup`client request`);
			reporter.success(markup`Evicted ${resolved}`);
		}

		reporter.info(markup`Evicted ${String(paths.length)} files`);
	},
});
