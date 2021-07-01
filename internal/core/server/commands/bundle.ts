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
import {Markup, markup} from "@internal/markup";
import {getByteLength} from "@internal/binary";
import {promiseAllFrom} from "@internal/async";

type Flags = {
	quiet: boolean;
	setVersion?: string;
	target: string;
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
			quiet: consumer.get("quiet").required(false).asBoolean(),
			setVersion: consumer.get("setVersion").asStringOrVoid(),
			target: consumer.get("target").required("default").asString(),
		};
	},
	async callback(req: ServerRequest, commandFlags: Flags): Promise<void> {
		const {flags} = req.client;
		const {args} = req.query;
		const {reporter} = req;
		req.expectArgumentLength(2);

		const [entryFilename, outputDirectory] = args;
		const bundler = Bundler.createFromServerRequest(
			req,
			{
				target: commandFlags.target,
			},
		);

		const resolution = await bundler.getResolvedEntry(
			entryFilename,
			{
				setVersion: commandFlags.setVersion,
			},
		);

		if (req.query.requestFlags.watch) {
			const {diagnosticsEvent, filesEvent, changeEvent} = bundler.bundleManifestWatch(
				resolution,
			);

			diagnosticsEvent.subscribe(async (diagnostics) => {
				reporter.clearScreen();
				const printer = req.createDiagnosticsPrinter();
				printer.processor.addDiagnostics(diagnostics);
				await printer.print();
			});

			filesEvent.subscribe(([name]) => {
				// TODO write
				name;
			});

			changeEvent.subscribe((paths) => {
				if (paths.size === 1) {
					reporter.info(markup`File change ${Array.from(paths)[0]}`);
				} else {
					reporter.info(markup`Multiple file changes`);
					reporter.list(Array.from(paths));
				}
			});

			await req.endEvent.wait();
		} else {
			const {files: outFiles} = await bundler.bundleManifest(resolution);

			const savedList: Markup[] = [];
			const dir = flags.cwd.resolve(outputDirectory);

			await promiseAllFrom(
				outFiles,
				async ([filename, {kind, content}]) => {
					const buff = await content();
					const path = dir.append(filename);
					await path.getParent().createDirectory();
					await path.writeFile(buff);
					const size = getByteLength(buff);

					savedList.push(
						markup`<filelink target="${path.join()}">${filename}</filelink> <filesize dim>${String(
							size,
						)}</filesize> <inverse> ${kind} </inverse>`,
					);
				},
			);

			await req.flushFiles();

			if (!commandFlags.quiet) {
				reporter.success(
					markup`Saved the following files to <emphasis>${dir}</emphasis>`,
				);
				// TODO sort this list so it's consistent
				reporter.list(savedList);
			}
		}
	},
});
