import {createServerCommand} from "@internal/core/server/commands";
import {markup} from "@internal/markup";
import {commandCategories} from "@internal/core/common/commands";
import {ServerRequest} from "@internal/core";
import {getVCSClient} from "@internal/vcs";
import {
	Diagnostic,
	createSingleDiagnosticsError,
	descriptions,
} from "@internal/diagnostics";
import {UnknownObject} from "@internal/typescript-helpers";
import Checker from "../checker/Checker";

interface Flags extends UnknownObject {
	checkVSC: boolean;
}

export type AutoConfig = {
	lint?: {
		diagnostics: Diagnostic[];
		savedCount: number;
	};
	licenses?: Diagnostic[];
};

export default createServerCommand<Flags>({
	category: commandCategories.PROCESS_MANAGEMENT,
	description: markup`Configure the project and fixes possible issues tha might occur while using Rome commands.`,
	usage: markup``,
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
	): Promise<AutoConfig | undefined> {
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

		const result: AutoConfig = {};

		// Check for no or dirty repo
		if (checkVSC) {
			const vcsClient = await getVCSClient(
				cwd,
			);
			if (vcsClient === undefined) {
				throw createSingleDiagnosticsError({
					location: req.getDiagnosticLocationForClientCwd(),
					description: descriptions.INIT_COMMAND.EXPECTED_REPO,
				});
			} else {
				const uncommittedFiles = await vcsClient.getUncommittedFiles();
				if (uncommittedFiles.length > 0) {
					throw createSingleDiagnosticsError({
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
						const checker = new Checker(
							req,
							{
								apply: true,
							},
						);
						const {printer, savedCount} = await checker.runSingle();
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
