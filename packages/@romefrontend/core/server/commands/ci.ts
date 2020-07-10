/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ServerRequest} from "@romefrontend/core";
import {commandCategories} from "../../common/commands";
import {chainCommands, createServerCommand} from "../commands";
import check from "./check";
import test from "./test";
import {Consumer} from "@romefrontend/consume";

type Flags = {
	fix: boolean;
};

export default createServerCommand({
	category: commandCategories.CODE_QUALITY,
	description: "run lint and tests",
	usage: "",
	examples: [],
	defineFlags(consumer: Consumer): Flags {
		return {
			fix: consumer.get("fix").asBoolean(false),
		};
	},
	async callback(req: ServerRequest, flags: Flags): Promise<void> {
		req.updateRequestFlags({
			verboseDiagnostics: "NO_TRUNCATE",
		});

		await chainCommands(
			req,
			[
				{
					title: "Running lint",
					callback: async () => {
						await check.callback(
							req,
							{
								formatOnly: false,
								decisions: [],
								apply: flags.fix,
								changed: undefined,
							},
						);
					},
				},
				{
					title: "Running tests",
					callback: async () => {
						await test.callback(
							req,
							{
								filter: undefined,
								focusAllowed: false,
								coverage: false,
								freezeSnapshots: !flags.fix,
								updateSnapshots: flags.fix,
								showAllCoverage: false,
								syncTests: false,
							},
						);
					},
				},
			],
		);
	},
});
