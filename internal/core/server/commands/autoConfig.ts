import {createServerCommand} from "@internal/core/server/commands";
import {markup} from "@internal/markup";
import {commandCategories} from "@internal/core/common/commands";
import {ServerRequest} from "@internal/core";
import {Diagnostic, descriptions} from "@internal/diagnostics";
import {UnknownObject} from "@internal/typescript-helpers";
import Checker from "../checker/Checker";
import {checkVSCWorkingDirectory} from "@internal/core/server/utils/checkVCSWorkingDirectory";

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
			).required(false).asBoolean(),
		};
	},
	async callback(
		req: ServerRequest,
		flags: Flags,
	): Promise<AutoConfig | undefined> {
		const {server, client, reporter} = req;

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
			await checkVSCWorkingDirectory(
				req,
				[
					descriptions.INIT_COMMAND.EXPECTED_REPO.advice,
					descriptions.INIT_COMMAND.UNCOMMITTED_CHANGES.advice,
				],
			);
		}
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

		return result;
	},
});
