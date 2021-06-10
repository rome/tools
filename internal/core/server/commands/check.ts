/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ServerRequest} from "@internal/core";
import Checker, {
	CheckerOptions,
	LinterCompilerOptionsPerFile,
} from "../checker/Checker";
import {markup} from "@internal/markup";
import {createServerCommand} from "../commands";
import {
	LintCompilerOptionsDecisions,
	parseDecisionStrings,
} from "@internal/compiler";
import {Consumer} from "@internal/consume";
import {commandCategories} from "@internal/core/common/commands";
import {createFilePath, createUIDPath} from "@internal/path";
import {LINTABLE_EXTENSIONS} from "@internal/core/common/file-handlers";
import {ServerRequestGlobArgs} from "../ServerRequest";

type Flags = {
	decisions: string[];
	apply: boolean;
	changed: undefined | string;
	formatOnly: boolean;
	suppressionExplanation: undefined | string;
};

let cachedSuppressionExplanation: string;

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
			suppressionExplanation: consumer.get(
				"suppressionExplanation",
				{
					description: markup`insert a predefined explanation for suppressions`,
				},
			).asStringOrVoid(),
		};
	},
	async callback(req: ServerRequest, flags: Flags): Promise<void> {
		if (flags.suppressionExplanation) {
			cachedSuppressionExplanation = flags.suppressionExplanation;
		}

		const {reporter} = req;

		let lintCompilerOptionsPerFile: LinterCompilerOptionsPerFile = {};
		let globalDecisions: LintCompilerOptionsDecisions = [];
		const {decisions} = flags;
		if (decisions !== undefined) {
			({lintCompilerOptionsPerFile, globalDecisions} = parseDecisionStrings({
				path: createUIDPath("argv"),
				decisions: decisions.map((value, i) => {
					return {
						value,
						start: req.getDiagnosticLocationFromFlags({
							type: "flag",
							key: "decisions",
							index: i,
						}).start,
					};
				}),
				cwd: req.client.flags.cwd,
				unexpected: (description) => {
					throw req.throwDiagnosticFlagError({
						description,
						target: {type: "flag", key: "decisions"},
					});
				},
			}));
		}

		// Look up arguments manually in vsc if we were passed a changes branch
		let args: undefined | ServerRequestGlobArgs;
		if (flags.changed !== undefined) {
			// No arguments expected when using this flag
			req.expectArgumentLength(0);

			const client = await req.getVCSClient();
			const target =
				flags.changed === "" ? client.getDefaultBranch() : flags.changed;
			const modifiedFiles = await client.getModifiedFiles(target);
			const flagLoc = req.getDiagnosticLocationFromFlags({
				type: "flag",
				key: "changed",
			});

			// Only include lintable files
			args = [];
			for (const arg of modifiedFiles) {
				const path = createFilePath(arg);
				for (const ext of LINTABLE_EXTENSIONS) {
					if (path.hasEndExtension(ext)) {
						args.push([path, flagLoc]);
						break;
					}
				}
			}

			if (args.length === 0) {
				reporter.warn(
					markup`No files changed from <emphasis>${target}</emphasis>`,
				);
			} else {
				reporter.info(markup`Files changed from <emphasis>${target}</emphasis>`);
				reporter.list(args.map(([path]) => path));
				reporter.hr();
			}
		}

		const opts: CheckerOptions = {
			hasDecisions: flags.decisions.length > 0,
			lintCompilerOptionsPerFile,
			globalDecisions,
			apply: flags.apply,
			formatOnly: flags.formatOnly,
			suppressionExplanation: cachedSuppressionExplanation,
			args,
		};
		const linter = new Checker(req, opts);
		if (req.query.requestFlags.watch) {
			await linter.runWatch();
		} else {
			await linter.throwSingle();
		}
	},
});
