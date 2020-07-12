/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ServerRequest} from "@romefrontend/core";
import {commandCategories} from "../../common/commands";
import {createServerCommand} from "../commands";

export default createServerCommand({
	category: commandCategories.PROCESS_MANAGEMENT,
	description: "stop daemon",
	usage: "",
	examples: [],
	hidden: true,
	defineFlags() {
		return {};
	},
	async callback({server}: ServerRequest) {
		server.end();
	},
});
