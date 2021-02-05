import {createServerCommand} from "@internal/core/server/commands";
import {markup} from "@internal/markup";
import {commandCategories} from "@internal/core/common/commands";
import {ServerRequest} from "@internal/core";
import Linter from "@internal/core/server/linter/Linter";
import {getVCSClient} from "@internal/vcs";
import {
	Diagnostics,
	DiagnosticsError,
	createSingleDiagnosticError,
	descriptions,
} from "@internal/diagnostics";
import {UnknownObject} from "@internal/typescript-helpers";
import {RSERValue} from "@internal/codec-binary-serial";

interface Flags extends UnknownObject {
	checkVSC: boolean;
}

export type AutoConfig = RSERValue & {
	lint?: {
		diagnostics: Diagnostics;
		savedCount: number;
	};
	licenses?: Diagnostics;
};

export default createServerCommand<Flags>({
	category: commandCategories.PROCESS_MANAGEMENT,
	description: markup`Configure the project and fixes possible issues tha might occur while using Rome commands.`,
	usage: "",
	examples: [],
	defineFlags(c) {
		return {
			checkVSC: c.get(
				"checkVSC",
				{
					description: markup`Check the existence of uncommitted files inside the repository.`,
				},
			).asBoolean(false),
		};
	},
	async callback(
		req: ServerRequest,
		flags: Flags,
	): Promise<DiagnosticsError | AutoConfig | undefined> {
		const {server, client, reporter} = req;

		// const {args} = req.query;
		const {cwd} = client.flags;
		const {checkVSC} = flags;
		const currentProject = await server.projectManager.assertProject(cwd);
		if (currentProject === undefined) {
			reporter.error(
				markup`No Rome project found at <emphasis>${cwd}</emphasis>`,
			);
			reporter.info(markup`Run <cmd>rome init</cmd> to boostrap your project.`);
			return;
		}

		if (!currentProject.initialized) {
			reporter.error(markup`Project not initialised.`);
			return;
		}

		const result: AutoConfig = {};

		// Check for no or dirty repo
		if (checkVSC) {
			const vcsClient = await getVCSClient(cwd);
			if (vcsClient === undefined) {
				throw createSingleDiagnosticError({
					location: req.getDiagnosticLocationForClientCwd(),
					description: descriptions.INIT_COMMAND.EXPECTED_REPO,
				});
			} else {
				const uncommittedFiles = await vcsClient.getUncommittedFiles();
				if (uncommittedFiles.length > 0) {
					throw createSingleDiagnosticError({
						location: req.getDiagnosticLocationForClientCwd(),
						description: descriptions.INIT_COMMAND.UNCOMMITTED_CHANGES,
					});
				}
			}
		} else {
			// Generate files
			await reporter.steps([
				{
					message: markup`Generating lint config and apply formatting`,
					async callback() {
						const linter = new Linter(
							req,
							{
								apply: true,
							},
						);
						const {printer, savedCount} = await linter.runSingle();
						result.lint = {
							diagnostics: printer.processor.getDiagnostics(),
							savedCount,
						};
					},
				},
				{
					message: markup`Scanning dependencies their licenses.`,
					async callback() {
						for (const def of currentProject.manifests.values()) {
							const diagnostics = def.manifest.diagnostics.license;

							if (diagnostics && diagnostics.length > 0) {
								if (!result.licenses) {
									result.licenses = [];
								}
								result.licenses.push(...diagnostics);
							}
						}
					},
				},
			]);
		}

		return result;
	},
});
