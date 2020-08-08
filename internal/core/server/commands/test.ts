/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ServerRequest} from "@internal/core";
import {Diagnostics} from "@internal/diagnostics";
import {Consumer} from "@internal/consume";
import {commandCategories} from "../../common/commands";
import {createServerCommand} from "../commands";
import TestServerRunner from "../testing/TestServerRunner";
import Bundler from "../bundler/Bundler";
import {JS_EXTENSIONS} from "../../common/file-handlers/javascript";
import {TestServerRunnerOptions, TestSources} from "../testing/types";
import {markup} from "@internal/markup";

type Flags = Omit<TestServerRunnerOptions, "verboseDiagnostics">;

export default createServerCommand({
	category: commandCategories.CODE_QUALITY,
	description: markup`run tests`,
	hidden: true,
	usage: "",
	examples: [],
	defineFlags(c: Consumer): Flags {
		return {
			filter: c.get("filter").asStringOrVoid(),
			coverage: c.get("coverage").asBoolean(false),
			showAllCoverage: c.get("showAllCoverage").asBoolean(false),
			updateSnapshots: c.get("updateSnapshots").asBoolean(false),
			freezeSnapshots: c.get("freezeSnapshots").asBoolean(false),
			focusAllowed: c.get("focusAllowed").asBoolean(true),
			syncTests: c.get("syncTests").asBoolean(false),
			sourceMaps: c.get("sourceMaps").asBoolean(true),
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
			disabledDiagnosticCategory: "tests/disabled",
		});
		const paths = await globber.get();

		let addDiagnostics: Diagnostics = [];

		const tests: TestSources = new Map();

		const bundler = new Bundler(
			req,
			req.getBundlerConfigFromFlags({
				mocks: true,
			}),
		);

		for (const [path, res] of await bundler.bundleMultiple(
			Array.from(paths),
			{
				deferredSourceMaps: true,
			},
		)) {
			tests.set(
				path.join(),
				{
					code: res.entry.js.content,
					sourceMap: res.entry.sourceMap.map,
					ref: req.server.projectManager.getFileReference(path),
				},
			);
		}

		const runner = new TestServerRunner({
			addDiagnostics,
			options: {
				...commandFlags,
				verboseDiagnostics: req.query.requestFlags.verboseDiagnostics,
			},
			sources: tests,
			request: req,
		});
		await runner.init();
	},
});
