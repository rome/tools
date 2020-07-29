/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ServerRequest} from "@romefrontend/core";
import {commandCategories} from "../../common/commands";
import {createServerCommand} from "../commands";
import {markup} from "@romefrontend/markup";
import {Diagnostics, descriptions} from "@romefrontend/diagnostics";
import {exists, readFileText} from "@romefrontend/fs";
import {RecoveryDiskStore} from "../fs/RecoveryStore";

type Flags = {
	select: boolean;
};

async function apply(req: ServerRequest, storeId: string) {
	const {server, reporter} = req;

	const entries = await server.recoveryStore.apply(
		req,
		storeId,
		req.getDiagnosticLocationFromFlags({
			type: "arg",
			key: 1,
		}),
		/*async (store) => {
			if (!flags.select) {
				return undefined;
			}

			const options: SelectOptions = {};
			for (const {fileId, originalPath, artifactPath} of store.entries) {
				options[fileId] = {
					label: markup`${artifactPath} -> ${originalPath}`,
				};
			}

			const selected = await reporter.select(markup`Select files to apply`, {options});
			return Array.from(selected);
		},*/
	);

	reporter.success(
		markup`Successfully applied patch <emphasis>${storeId}</emphasis>`,
	);
	reporter.list(
		Array.from(
			entries,
			({artifactPath, originalPath}) =>
				markup`${artifactPath} -> ${originalPath}`
			,
		),
	);
}

async function getAllStores(
	req: ServerRequest,
): Promise<Array<RecoveryDiskStore>> {
	const {server, reporter} = req;
	const {stores, diagnostics} = await server.recoveryStore.getAllStores();

	// May have encountered some issues loading corrupted stores. Invalid index.json files etc
	if (diagnostics.length > 0) {
		reporter.warn(markup`Encountered errors reading from recovery store`);
		await req.printDiagnostics({diagnostics});
		reporter.hr();
	}

	return stores;
}

export default createServerCommand<Flags>({
	category: commandCategories.SOURCE_CODE,
	description: markup`reverts the last file modifications`,
	usage: "",
	examples: [],
	hidden: false,
	defineFlags(c) {
		return {
			select: c.get(
				"select",
				{
					description: markup`When applying a patch show an interactive prompt to select specific files`,
				},
			).asBoolean(false),
		};
	},
	async callback(req: ServerRequest, flags: Flags): Promise<void> {
		const {server} = req;
		const {reporter} = req;

		req.expectArgumentLength(1, 2);
		const [action] = req.query.args;

		switch (action) {
			case "list": {
				req.expectArgumentLength(1);
				const stores = await getAllStores(req);

				if (stores.length === 0) {
					reporter.info(markup`Recovery store is empty`);
					break;
				} else {
					reporter.heading(markup`Recovery stores`);

					for (const {storeId, timestamp, command, entries} of stores) {
						await reporter.section(
							markup`${storeId}`,
							() => {
								reporter.log(
									markup`<emphasis>Ran <duration>${String(
										Date.now() - new Date(timestamp).valueOf(),
									)}</duration> ago</emphasis> <dim>(${timestamp})</dim>`,
								);
								reporter.command(command);
								reporter.br();
								reporter.list(
									Array.from(
										entries,
										({artifactPath, originalPath}) =>
											markup`${originalPath} -> ${artifactPath}`
										,
									),
								);
								reporter.br();

								reporter.info(markup`To select specific files to patch run:`);
								reporter.command(`rome recover apply ${storeId} --select`);
								reporter.br();

								reporter.info(markup`To see the changes with this patch run:`);
								reporter.command(`rome recover diff ${storeId}`);
								reporter.br();

								reporter.info(
									markup`To apply <emphasis>everything</emphasis> in this patch run:`,
								);
								reporter.command(`rome recover apply ${storeId}`);
								reporter.br();
							},
						);
					}

					break;
				}
			}

			case "diff": {
				req.expectArgumentLength(2);
				const storeId = req.query.args[1];
				const diagnostics: Diagnostics = [];

				const store = await server.recoveryStore.getStore(
					storeId,
					req.getDiagnosticLocationFromFlags({
						type: "arg",
						key: 1,
					}),
				);

				for (const {originalPath, artifactPath} of store.entries) {
					// Original may have been deleted
					let original = "";
					if (await exists(originalPath)) {
						original = await readFileText(originalPath);
					}

					const artifact = await readFileText(artifactPath);

					diagnostics.push({
						location: {
							filename: originalPath.join(),
						},
						description: descriptions.RECOVERY_STORE.DIFF(original, artifact),
					});
				}

				await req.printDiagnostics({diagnostics});
				break;
			}

			case "apply": {
				flags.select;
				req.expectArgumentLength(2);
				await apply(req, req.query.args[1]);
				break;
			}

			case "pop": {
				req.expectArgumentLength(1);

				const stores = await getAllStores(req);

				if (stores.length === 0) {
					reporter.error(markup`No recovery stores`);
					break;
				}

				const latestStoreId = stores[stores.length - 1].storeId;
				await apply(req, latestStoreId);
				break;
			}

			case "clear": {
				await req.server.recoveryStore.clear();
				reporter.success(markup`Cleared recovery store`);
				break;
			}

			default:
				throw req.throwDiagnosticFlagError({
					description: descriptions.FLAGS.UNKNOWN_ACTION(action),
					target: {
						type: "arg",
						key: 0,
					},
				});
		}
	},
});
