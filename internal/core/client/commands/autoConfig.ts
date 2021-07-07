import {createLocalCommand} from "@internal/core/client/commands";
import {CommandName, commandCategories} from "@internal/core/common/commands";
import {markup} from "@internal/markup";
import ClientRequest from "@internal/core/client/ClientRequest";
import {consumeUnknown} from "@internal/consume";
import {DIAGNOSTIC_CATEGORIES} from "@internal/diagnostics";
import {escapePath} from "@internal/core/server/utils/escapeObjectPaths";

interface Flags {
	allowDirty: boolean;
}

export default createLocalCommand({
	category: commandCategories.PROCESS_MANAGEMENT,
	description: markup`It scans the project and provides possible fixes to apply to Rome configuration`,
	usage: markup`<cmd>rome auto-config</cmd>`,
	examples: [],
	defineFlags(c) {
		return {
			allowDirty: c.get(
				"allowDirty",
				{
					description: markup`Allows running auto-config command by skipping the check on uncommitted files.`,
				},
			).required(false).asBoolean(),
		};
	},
	async callback(req: ClientRequest, flags: Flags) {
		const {client} = req;
		const {reporter} = client;
		const {allowDirty} = flags;
		let savedCheckFiles: number | undefined = undefined;
		let remainingCheckErrors: number | undefined = undefined;

		const result = await req.client.query(
			{
				commandName: "auto-config",
				commandFlags: {
					checkVSC: !allowDirty,
				},
			},
			"server",
		);

		if (result.type !== "SUCCESS") {
			return false;
		}

		const data = consumeUnknown(
			result.data,
			DIAGNOSTIC_CATEGORIES.parse,
			"json",
		);

		if (!data.exists()) {
			reporter.log(markup`No problems or updates found in you project.`);
			return true;
		}

		if (data.get("lint").exists()) {
			const lint = data.get("lint").asMap();
			const diagnostics = lint.get("diagnostics");
			const savedCount = lint.get("savedCount");
			if (savedCount) {
				savedCheckFiles = savedCount.asNumber();
			}
			if (diagnostics) {
				const lintDiagnostics = diagnostics.asImplicitArray();
				if (lintDiagnostics.length > 0) {
					remainingCheckErrors = 0;
					for (const diagnostic of lintDiagnostics) {
						const description = diagnostic.get("description").asMap();
						if (description.has("category")) {
							const category = description.get("category");
							const categoryValue = description.get("categoryValue");
							if (
								category &&
								categoryValue &&
								category.exists() &&
								categoryValue.exists()
							) {
								if (category.asString() === "lint/js/noUndeclaredVariables") {
									await req.client.query(
										{
											commandName: "config push",
											args: ["lint.globals", categoryValue.asString()],
										},
										"server",
									);
								} else {
									remainingCheckErrors++;
								}
							}
						}
					}
				}
			}
		}
		if (data.get("licenses").exists()) {
			const licenses = data.get("licenses").asImplicitArray();

			if (licenses.length > 0) {
				for (const license of licenses) {
					const description = license.get("description").asMap();
					if (description.has("advice")) {
						const advice = description.get("advice");
						if (advice?.exists()) {
							const diags = advice.asImplicitArray();
							if (diags.length > 0) {
								const actionDiagnostics = diags.filter((d) =>
									d.get("type").asString() === "action"
								);
								if (actionDiagnostics.length > 0) {
									for (const action of actionDiagnostics) {
										const command = action.get("command").asString();
										const args = action.get("args").asMappedArray((c) =>
											c.asString()
										);

										await req.client.query(
											{
												commandName: command as CommandName,
												args,
											},
											"server",
										);
									}
								}
							}
						}
					}
				}
			}
		}

		if (data.has("aliases")) {
			const aliases = data.get("aliases");

			if (aliases.has("base")) {
				const base = aliases.get("base").asString();
				const changeBase = await reporter.radioConfirm(
					markup`Would you like to import <emphasis>${base}</emphasis> inside your configuration?`,
				);
				if (changeBase) {
					await req.client.query(
						{
							commandName: "config set",
							args: ["aliases.base", base],
						},
						"server",
					);
				}
			}

			if (aliases.has("paths")) {
				const paths = aliases.get("paths").asMappedArray((item) =>
					(item.asPlainArray() as [string, string[]])
				);

				if (paths.length > 0) {
					reporter.info(
						markup`Rome detected path aliases configured in your <emphasis>tsconfig.json</emphasis> and will try to import them.`,
					);

					const pathKeys = paths.map((item) => `<emphasis>${item[0]}</emphasis>`).join(
						", ",
					);
					const addPaths = await reporter.radioConfirm(
						markup`Would you like to import the following aliases ${pathKeys} in your Rome's configuration?`,
					);
					if (addPaths) {
						for (const [alias, targets] of paths) {
							const configPath = `aliases.paths.${escapePath(alias)}`;
							await req.client.query({
								commandName: "config set",
								args: [configPath, "[]"],
							});

							await req.client.query(
								{
									commandName: "config push",
									args: [configPath, ...targets],
								},
								"server",
							);
						}
					}
				}
			}

			reporter.warn(
				markup`Rome won't keep track of imported configuration from <emphasis>tsconfig.json</emphasis>.`,
			);
			reporter.warn(
				markup`You must re-run <emphasis>auto-config</emphasis> if you want to import settings from <emphasis>tsconfig.json</emphasis> again.`,
			);
		}

		if (savedCheckFiles !== undefined && remainingCheckErrors !== undefined) {
			await reporter.section(
				markup`Summary`,
				async () => {
					if (savedCheckFiles !== undefined && savedCheckFiles > 0) {
						reporter.info(
							markup`<emphasis>${savedCheckFiles}</emphasis> <grammarNumber plural="files" singular="file">${String(
								savedCheckFiles,
							)}</grammarNumber> saved`,
						);
					}
					if (remainingCheckErrors === 0) {
						reporter.success(markup`No problems found!`);
					} else {
						reporter.warn(
							markup`<emphasis>${remainingCheckErrors}</emphasis> errors remaining. Run <code>rome check</code> to view.`,
						);
					}
					reporter.br();
				},
			);
		}

		return true;
	},
});
