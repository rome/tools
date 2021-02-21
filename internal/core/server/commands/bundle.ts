/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ServerRequest} from "@internal/core";
import {commandCategories} from "../../common/commands";
import {createServerCommand} from "../commands";
import Bundler from "../bundler/Bundler";
import {Consumer} from "@internal/consume";
import {markup} from "@internal/markup";

type Flags = {
	quiet: boolean;
};

export default createServerCommand<Flags>({
	category: commandCategories.SOURCE_CODE,
	description: markup`build a standalone js bundle for a package`,
	usage: "",
	examples: [],
	hidden: true,
	allowRequestFlags: ["watch"],
	defineFlags(consumer: Consumer): Flags {
		return {
			quiet: consumer.get("quiet").asBoolean(false),
		};
	},
	async callback(req: ServerRequest, commandFlags: Flags): Promise<void> {
		const {flags} = req.client;
		const {args} = req.query;
		const {reporter} = req;
		req.expectArgumentLength(2);

		const [entryFilename, outputDirectory] = args;
		const bundler = Bundler.createFromServerRequest(req);

		const resolution = await bundler.getResolvedEntry(entryFilename);

		if (req.query.requestFlags.watch) {
			const {diagnosticsEvent, filesEvent, changeEvent} = bundler.bundleManifestWatch(resolution);

			diagnosticsEvent.subscribe(async (diagnostics) => {
				reporter.clearScreen();
				const printer = req.createDiagnosticsPrinter();
				printer.processor.addDiagnostics(diagnostics);
				await printer.print();
			});

			filesEvent.subscribe(([name]) => {
				console.log(name);
			});

			changeEvent.subscribe(paths => {
				if (paths.length === 1) {
					reporter.info(markup`File change ${paths[0]}`);
				} else {
					reporter.info(markup`Multiple file changes`);
					reporter.list(paths);
				}
			});
			
			await req.endEvent.wait();
		} else {
			const {files: outFiles} = await bundler.bundleManifest(resolution);

			const savedList = [];
			const dir = flags.cwd.resolve(outputDirectory);
			for (const [filename, {kind, content}] of outFiles) {
				const buff = content();
				const file = dir.append(filename);
				const loc = file.join();
				savedList.push(
					markup`<filelink target="${loc}">${filename}</filelink> <filesize dim>${String(
						Buffer.byteLength(buff),
					)}</filesize> <inverse> ${kind} </inverse>`,
				);
				await file.getParent().createDirectory();
				await file.writeFile(buff);
			}

			await req.flushFiles();

			if (commandFlags.quiet) {
				reporter.success(markup`Saved to <emphasis>${dir}</emphasis>`);
			} else {
				reporter.success(
					markup`Saved the following files to <emphasis>${dir}</emphasis>`,
				);
				reporter.list(savedList);
			}
		}
	},
});
