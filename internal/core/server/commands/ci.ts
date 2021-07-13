/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ServerRequest} from "@internal/core";
import {commandCategories} from "../../common/commands";
import {chainCommands, createServerCommand} from "../commands";
import check from "./check";
import test from "./test";
import {Consumer} from "@internal/consume";
import {markup} from "@internal/markup";

type Flags = {
	fix: boolean;
};

export default createServerCommand({
	category: commandCategories.CODE_QUALITY,
	description: markup`run lint and tests`,
	usage: "",
	hidden: true,
	examples: [],
	defineFlags(consumer: Consumer): Flags {
		return {
			fix: consumer.get(
				"fix",
				{
					description: markup`enables --update-snapshots for test, and --apply for the lint command`,
				},
			).required(false).asBoolean(),
		};
	},
	async callback(req: ServerRequest, flags: Flags): Promise<void> {
		req.updateRequestFlags({
			truncateDiagnostics: false,
			maxDiagnostics: Infinity,
		});

		await chainCommands(
			req,
			[
				{
					title: markup`<code>rome check</code>`,
					callback: async () => {
						await check.callback(
							req,
							{
								formatOnly: false,
								decisions: [],
								apply: flags.fix,
								changed: undefined,
								suppressionExplanation: undefined,
							},
						);
					},
				},
				{
					title: markup`<code>rome test</code>`,
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
								runInSync: false,
								sourceMaps: true,
								suppressLogs: true,
							},
						);
					},
				},
			],
		);
	},
});
