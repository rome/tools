/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ServerRequest} from "@internal/core";
import {commandCategories} from "../../common/commands";
import {createServerCommand} from "../commands";
import {markup} from "@internal/markup";
import {Diagnostics, descriptions} from "@internal/diagnostics";
import {exists, readFileText} from "@internal/fs";
import {RecoveryDiskStore} from "../fs/RecoveryStore";
import {Consumer} from "@internal/consume";

async function applyStore(req: ServerRequest, storeId: string) {
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

export const list = createServerCommand({
	category: commandCategories.SOURCE_CODE,
	description: markup`show all patches in the recovery store`,
	usage: "",
	examples: [],
	defineFlags() {
		return {};
	},
	async callback(req: ServerRequest) {
		const {reporter} = req;
		req.expectArgumentLength(0);

		const stores = await getAllStores(req);

		if (stores.length === 0) {
			reporter.info(markup`Recovery store is empty`);
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
		}
	},
});

export const diff = createServerCommand({
	category: commandCategories.SOURCE_CODE,
	description: markup`show the differences that would be applied for a given patch`,
	usage: "<id>",
	examples: [],
	defineFlags() {
		return {};
	},
	async callback(req: ServerRequest) {
		const {server} = req;
		req.expectArgumentLength(1);
		const storeId = req.query.args[0];
		const diagnostics: Diagnostics = [];

		const store = await server.recoveryStore.getStore(
			storeId,
			req.getDiagnosticLocationFromFlags({
				type: "arg",
				key: 0,
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
	},
});

type ApplyFlags = {
	select: boolean;
};

export const apply = createServerCommand<ApplyFlags>({
	category: commandCategories.SOURCE_CODE,
	description: markup`apply a specific patch from the recovery store`,
	usage: "<id>",
	examples: [],
	defineFlags(c: Consumer): ApplyFlags {
		return {
			select: c.get(
				"select",
				{
					description: markup`When applying a patch show an interactive prompt to select specific files`,
				},
			).asBoolean(false),
		};
	},
	async callback(req: ServerRequest, flags: ApplyFlags) {
		flags.select;
		req.expectArgumentLength(1);
		await applyStore(req, req.query.args[0]);
	},
});

export const pop = createServerCommand({
	category: commandCategories.SOURCE_CODE,
	description: markup`apply the most recent patch in the recovery story`,
	usage: "",
	examples: [],
	defineFlags() {
		return {};
	},
	async callback(req: ServerRequest) {
		req.expectArgumentLength(0);

		const stores = await getAllStores(req);

		if (stores.length === 0) {
			req.reporter.error(markup`No recovery stores`);
		} else {
			const latestStoreId = stores[stores.length - 1].storeId;
			await applyStore(req, latestStoreId);
		}
	},
});

export const clear = createServerCommand({
	category: commandCategories.SOURCE_CODE,
	description: markup`clear the contents of the recovery store`,
	usage: "",
	examples: [],
	defineFlags() {
		return {};
	},
	async callback(req: ServerRequest) {
		req.expectArgumentLength(0);
		await req.server.recoveryStore.clear();
		req.reporter.success(
			markup`Cleared recovery store at <emphasis>${req.server.recoveryStore.getDirectory()}</emphasis>`,
		);
	},
});

export const dir = createServerCommand({
	category: commandCategories.SOURCE_CODE,
	description: markup`print the location of the recovery store`,
	usage: "",
	examples: [],
	defineFlags() {
		return {};
	},
	async callback(req: ServerRequest) {
		req.expectArgumentLength(0);
		req.reporter.log(markup`${req.server.recoveryStore.getDirectory()}`);
	},
});
