/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ServerRequest} from "@internal/core";
import {commandCategories} from "../../common/commands";
import {createServerCommand} from "../commands";
import {ProjectDefinition} from "@internal/project";
import {markup} from "@internal/markup";

export default createServerCommand({
	category: commandCategories.INTERNAL,
	description: markup`TODO`,
	usage: "",
	examples: [],
	hidden: true,
	defineFlags(c) {
		return {
			complete: c.get("complete").asBoolean(false),
		};
	},
	async callback(req: ServerRequest, flags): Promise<void> {
		const path = await req.resolveEntryAssertPathArg(0);
		let project: undefined | ProjectDefinition = req.server.projectManager.assertProjectExisting(
			path,
		);

		while (project !== undefined) {
			req.reporter.log(markup`${project.directory}`);
			if (flags.complete) {
				req.reporter.inspect(project.config);
			} else {
				const {consumer} = project.meta;
				if (consumer !== undefined) {
					req.reporter.inspect(consumer.asUnknown());
				}
			}
			project = project.parent;
		}
	},
});
