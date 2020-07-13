/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ServerRequest} from "@romefrontend/core";
import {commandCategories} from "../../common/commands";
import {createServerCommand} from "../commands";
import Bundler from "../bundler/Bundler";
import {createDirectory, writeFile} from "@romefrontend/fs";
import {Consumer} from "@romefrontend/consume";
import {markup} from "@romefrontend/string-markup";

type Flags = {
	quiet: boolean;
};

export default createServerCommand<Flags>({
	category: commandCategories.SOURCE_CODE,
	description: "build a standalone js bundle for a package",
	usage: "",
	examples: [],
	hidden: true,
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

		const [entryFilename, outputFolder] = args;
		const bundler = Bundler.createFromServerRequest(req);

		const resolution = await bundler.getResolvedEntry(entryFilename);
		const {files: outFiles} = await bundler.bundleManifest(resolution);

		const savedList = [];
		const dir = flags.cwd.resolve(outputFolder);
		for (const [filename, {kind, content}] of outFiles) {
			const buff = content();
			const file = dir.append(filename);
			const loc = file.join();
			savedList.push(
				markup`<filelink target="${loc}">${filename}</filelink> <filesize dim>${Buffer.byteLength(
					buff,
				)}</filesize> <inverse> ${kind} </inverse>`,
			);
			await createDirectory(file.getParent());
			await writeFile(file, buff);
		}

		if (commandFlags.quiet) {
			reporter.success(`Saved to <emphasis>${dir.toMarkup()}</emphasis>`);
		} else {
			reporter.success(
				`Saved the following files to <emphasis>${dir.toMarkup()}</emphasis>`,
			);
			reporter.list(savedList);
		}
	},
});
