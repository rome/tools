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
	category: commandCategories.PROCESS_MANAGEMENT,
	description: markup`stop daemon`,
	usage: "",
	examples: [],
	defineFlags() {
		return {};
	},
	async callback({server}: ServerRequest) {
		await server.end();
	},
});
