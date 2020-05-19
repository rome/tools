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
	category: commandCategories.INTERNAL,
	description: "TODO",
	usage: "",
	examples: [],
	defineFlags() {
		return {};
	},
	async callback(req: MasterRequest): Promise<void> {
		req;
	},
});
