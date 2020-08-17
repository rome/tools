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
	changed?: string;
};

export default createServerCommand({
	category: commandCategories.CODE_QUALITY,
	description: markup`run lint and tests`,
	usage: "",
	hidden: true,
	examples: [],
	defineFlags(consumer: Consumer): Flags {
		return {
			changed: consumer.get(
				"changed",
				{
					description: markup`only check files that have changed from the specified branch/commit (defaults to main)`,
				},
			).asStringOrVoid(),
			fix: consumer.get(
				"fix",
				{
					description: markup`enables --update-snapshots for test, and --apply for the lint command`,
				},
			).asBoolean(false),
		};
	},
	async callback(req: ServerRequest, {fix, changed}: Flags): Promise<void> {
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
								apply: fix,
								changed,
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
								freezeSnapshots: !fix,
								updateSnapshots: fix,
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
