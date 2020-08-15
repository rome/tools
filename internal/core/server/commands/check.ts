/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ServerRequest} from "@internal/core";
import Linter, {
	LinterCompilerOptionsPerFile,
	LinterOptions,
} from "../linter/Linter";
import {markup} from "@internal/markup";
import {createServerCommand} from "../commands";
import {
	LintCompilerOptionsDecisions,
	parseDecisionStrings,
} from "@internal/compiler";
import {Consumer} from "@internal/consume";
import {commandCategories} from "@internal/core/common/commands";
import {createUnknownPath} from "@internal/path";
import {LINTABLE_EXTENSIONS} from "@internal/core/common/file-handlers";

type Flags = {
	decisions: Array<string>;
	apply: boolean;
	changed: undefined | string;
	formatOnly: boolean;
};

export default createServerCommand<Flags>({
	category: commandCategories.CODE_QUALITY,
	description: markup`run lint against a set of files`,
	allowRequestFlags: ["watch", "review"],
	usage: "",
	examples: [],
	defineFlags(consumer: Consumer): Flags {
		return {
			decisions: consumer.get("decisions").asImplicitMappedArray((item) =>
				item.asString()
			),
			apply: consumer.get(
				"apply",
				{
					description: markup`apply safe fixes and formatting`,
				},
			).asBoolean(false),
			formatOnly: consumer.get(
				"formatOnly",
				{
					description: markup`only formatting is applied`,
				},
			).asBoolean(false),
			changed: consumer.get(
				"changed",
				{
					description: markup`only include files that have changed from the specified branch/commit (defaults to main)`,
				},
			).asStringOrVoid(),
		};
	},
	async callback(req: ServerRequest, flags: Flags): Promise<void> {
		const {reporter} = req;

		let lintCompilerOptionsPerFile: LinterCompilerOptionsPerFile = {};
		let globalDecisions: LintCompilerOptionsDecisions = [];
		const {decisions} = flags;
		if (decisions !== undefined) {
			({lintCompilerOptionsPerFile, globalDecisions} = parseDecisionStrings(
				decisions,
				req.client.flags.cwd,
				(description) => {
					throw req.throwDiagnosticFlagError({
						description,
						target: {type: "flag", key: "decisions"},
					});
				},
			));
		}

		// Look up arguments manually in vsc if we were passed a changes branch
		let args;
		if (flags.changed !== undefined) {
			// No arguments expected when using this flag
			req.expectArgumentLength(0);

			const client = await req.getVCSClient();
			const target =
				flags.changed === "" ? await client.getDefaultBranch() : flags.changed;
			args = await client.getModifiedFiles(target);

			// Only include lintable files
			args = args.filter((arg) => {
				const path = createUnknownPath(arg);

				for (const ext of LINTABLE_EXTENSIONS) {
					if (path.hasEndExtension(ext)) {
						return true;
					}
				}

				return false;
			});

			if (args.length === 0) {
				reporter.warn(
					markup`No files changed from <emphasis>${target}</emphasis>`,
				);
			} else {
				reporter.info(markup`Files changed from <emphasis>${target}</emphasis>`);
				reporter.list(args.map((arg) => markup`<filelink target="${arg}" />`));
				reporter.hr();
			}
		}

		const opts: LinterOptions = {
			hasDecisions: flags.decisions.length > 0,
			lintCompilerOptionsPerFile,
			globalDecisions,
			apply: flags.apply,
			formatOnly: flags.formatOnly,
			args,
		};

		const linter = new Linter(req, opts);
		if (req.query.requestFlags.watch) {
			await linter.runWatch();
		} else {
			await linter.throwSingle();
		}
	},
});
