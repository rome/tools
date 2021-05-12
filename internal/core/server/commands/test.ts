/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ServerRequest} from "@internal/core";
import {Consumer} from "@internal/consume";
import {commandCategories} from "../../common/commands";
import {createServerCommand} from "../commands";
import TestServer from "../testing/TestServer";
import {JS_EXTENSIONS} from "../../common/file-handlers/javascript";
import {TestServerRunnerOptions} from "../testing/types";
import {markup} from "@internal/markup";

type Flags = Omit<TestServerRunnerOptions, "verboseDiagnostics"> & {
	suppressLogs: boolean;
};

export default createServerCommand({
	category: commandCategories.CODE_QUALITY,
	description: markup`run tests`,
	hidden: true,
	usage: "",
	examples: [],
	defineFlags(c: Consumer): Flags {
		return {
			filter: c.get(
				"filter",
				{description: markup`filter tests using the provided string`},
			).asStringOrVoid(),
			coverage: c.get(
				"coverage",
				{description: markup`collect code coverage and print it`},
			).default(false).asBoolean(),
			showAllCoverage: c.get(
				"showAllCoverage",
				{description: markup`show code coverage for all the tests`},
			).default(false).asBoolean(),
			updateSnapshots: c.get(
				"updateSnapshots",
				{description: markup`update failing snapshots`},
			).default(false).asBoolean(),
			freezeSnapshots: c.get(
				"freezeSnapshots",
				{description: markup`prevents snapshots from updating`},
			).default(false).asBoolean(),
			focusAllowed: c.get(
				"focusAllowed",
				{description: markup`prevent tests from being focused`},
			).default(true).asBoolean(),
			runInSync: c.get("runInSync", {description: markup`run tests in sync`}).default(
				false,
			).asBoolean(),
			sourceMaps: c.get(
				"sourceMaps",
				{description: markup`prevent sourceMaps from being generated`},
			).default(true).asBoolean(),
			suppressLogs: c.get(
				"suppressLogs",
				{description: markup`print all logs to the console`},
			).default(true).asBoolean(),
		};
	},
	async callback(req: ServerRequest, commandFlags: Flags): Promise<void> {
		const globber = await req.glob({
			tryAlternateArg: (path) => {
				if (path.hasExtension("test")) {
					return undefined;
				} else {
					return path.getParent().append(
						`${path.getExtensionlessBasename()}.test${path.getExtensions()}`,
					);
				}
			},
			test: (path) => path.hasExtension("test"),
			noun: "test",
			verb: "testing",
			configCategory: "tests",
			advice: [
				{
					type: "log",
					category: "info",
					text: markup`Searched for files with <emphasis>.test.*</emphasis> file extension`,
				},
			],
			extensions: JS_EXTENSIONS,
		});
		const paths = await globber.get();

		const runner = new TestServer({
			options: {
				...commandFlags,
				verboseDiagnostics: req.query.requestFlags.verboseDiagnostics,
			},
			paths,
			request: req,
		});
		await runner.init();
	},
});
