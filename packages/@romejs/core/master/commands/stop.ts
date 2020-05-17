/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {MasterRequest} from "@romejs/core";
import {commandCategories} from "../../common/commands";
import {createMasterCommand} from "../commands";

export default createMasterCommand({
	category: commandCategories.PROCESS_MANAGEMENT,
	description: "stop daemon",
	usage: "",
	examples: [],
	defineFlags() {
		return {};
	},
	async callback({master}: MasterRequest) {
		master.end();
	},
});
