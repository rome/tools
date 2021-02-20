/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ServerRequest} from "@internal/core";
import {createServerCommand} from "../commands";
import {commandCategories} from "../../common/commands";
import {json} from "@internal/codec-config";
import {markup} from "@internal/markup";

export default createServerCommand({
	category: commandCategories.INTERNAL,
	description: markup`dump an RJSON file to regular JSON`,
	usage: "",
	examples: [],
	defineFlags() {
		return {};
	},
	async callback(req: ServerRequest): Promise<undefined | string> {
		const {reporter} = req;
		const path = await req.resolveEntryAssertPathArg(0);

		const file = await path.readFileText();
		const value = json.parse({
			path,
			input: file,
		});

		const serial = json.stringify(value);
		reporter.write(serial);
		return serial;
	},
});
