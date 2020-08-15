import {createServerCommand} from "@internal/core/server/commands";
import {commandCategories} from "@internal/core/common/commands";
import {StaticMarkup, markup} from "@internal/markup";
import {ServerRequest, VERSION} from "@internal/core";
import {ExtensionHandler} from "@internal/core/common/file-handlers/types";
import {dedent} from "@internal/string-utils";
import {
	createDirectory,
	exists,
	readFileTextMeta,
	writeFile,
} from "@internal/fs";
import {
	JSONObject,
	RJSONCommentMap,
	consumeJSON,
	stringifyRJSON,
} from "@internal/codec-json";
import {getFileHandlerFromPath} from "@internal/core/common/file-handlers";
import Linter from "../linter/Linter";
import {PROJECT_CONFIG_DIRECTORY} from "@internal/project";
import {AbsoluteFilePathMap} from "@internal/path";
import {getVCSClient} from "@internal/vcs";
import {
	Diagnostic,
	createSingleDiagnosticError,
	descriptions,
} from "@internal/diagnostics";
import {
	Manifest,
	convertManifestToJSON,
	normalizeManifest,
} from "@internal/codec-js-manifest";
import {parseSemverRange} from "@internal/codec-semver";
import {spawn} from "@internal/child-process";

type Flags = {
	apply: boolean;
	allowDirty: boolean;
};

export default createServerCommand<Flags>({
	category: commandCategories.PROCESS_MANAGEMENT,
	description: markup`initialise the project`,
	usage: "",
	examples: [],
	defineFlags(c) {
		return {
			apply: c.get(
				"apply",
				{
					description: markup``,
				},
			).asBoolean(false),
			allowDirty: c.get(
				"allowDirty",
				{
					description: markup`Allow running command with a dirty checkout`,
				},
			).asBoolean(false),
		};
	},
	async callback(req: ServerRequest, flags: Flags) {
		const {server, client, reporter} = req;

		const {args} = req.query;
		let {cwd} = client.flags;

		// Warn if provided with arguments
		if (args.length > 0) {
			req.expectArgumentLength(
				0,
				0,
				[
					{
						type: "log",
						category: "info",
						text: markup`If you meant to specify a specific folder to initialize other than the one you're in, use the <code>--cwd</code> flag:`,
					},
					{
						type: "command",
						command: `rome init --cwd ${args[0]}`,
					},
				],
			);
		}

		// Check for sensitive directory
		if (server.projectManager.isBannedProjectPath(cwd)) {
			const diagnostic: Diagnostic = {
				description: descriptions.PROJECT_MANAGER.INITING_SENSITIVE(cwd),
				location: req.getDiagnosticLocationFromFlags("cwd"),
			};
			throw createSingleDiagnosticError(diagnostic);
		}

		// Don't allow if we're already in a project
		const existingProject = await server.projectManager.findProject(cwd);
		if (existingProject !== undefined) {
			reporter.error(
				markup`Project already exists. Defined at <emphasis>${existingProject.meta.configPath}</emphasis>`,
			);
			reporter.info(
				markup`Use <code>rome config</code> to update an existing config`,
			);
			return;
		}

		// Check for no or dirty repo
		if (flags.apply && !flags.allowDirty) {
			const vcsClient = await getVCSClient(cwd);

			if (vcsClient === undefined) {
				throw createSingleDiagnosticError({
					location: req.getDiagnosticLocationFromFlags("cwd"),
					description: descriptions.INIT_COMMAND.EXPECTED_REPO,
				});
			} else {
				const uncommittedFiles = await vcsClient.getUncommittedFiles();
				if (uncommittedFiles.length > 0) {
					throw createSingleDiagnosticError({
						location: req.getDiagnosticLocationFromFlags("cwd"),
						description: descriptions.INIT_COMMAND.UNCOMMITTED_CHANGES,
					});
				}
			}
		}

		reporter.heading(markup`Welcome to Rome! Let's get you started...`);

		const configPath = cwd.append(PROJECT_CONFIG_DIRECTORY, "rome.rjson");

		// Track some information about our project generation
		let savedCheckFiles: number | undefined = undefined;
		let remainingCheckErrors: number | undefined = undefined;
		const files: AbsoluteFilePathMap<StaticMarkup> = new AbsoluteFilePathMap();
		files.set(
			configPath,
			markup`Your project configuration. Documentation: <hyperlink target="https://romefrontend.dev/#project-configuration" />`,
		);

		// We are only using JSONObject here because we don't have an accurate type definition for what
		// the config actually looks like on disk
		let config: JSONObject = {
			root: true,
			name: cwd.getBasename(),
		};

		// Comments to include in the created project config
		const comments: RJSONCommentMap = new Map();
		comments.set(
			"",
			{
				inner: [],
				outer: [
					{
						type: "LineComment",
						value: " For configuration documentation see https://romefrontend.dev/#project-configuration",
					},
				],
			},
		);

		async function updateConfig(partial: JSONObject = {}) {
			// Update it on disk
			config = {
				...config,
				...partial,
			};
			await writeFile(configPath, stringifyRJSON(config, comments) + "\n");
		}

		// Create initial project config
		await createDirectory(configPath.getParent());
		await updateConfig();

		//
		const manifestPath = cwd.append("package.json");
		let manifest: undefined | Manifest;
		if (await exists(manifestPath)) {
			manifest = await normalizeManifest(
				consumeJSON(await readFileTextMeta(manifestPath)),
			);
		}

		// Generate files
		await reporter.steps([
			{
				message: markup`Installing Rome as a dependency`,
				test() {
					return (
						manifest !== undefined &&
						!manifest.dependencies.has("rome") &&
						!manifest.devDependencies.has("rome")
					);
				},
				async callback() {
					if (manifest === undefined) {
						// Should not be because of test()
						return;
					}

					// Modify package.json
					manifest.devDependencies.set(
						"rome",
						{
							type: "semver",
							range: parseSemverRange({input: `^${VERSION}`}),
						},
					);
					await writeFile(
						manifestPath,
						JSON.stringify(convertManifestToJSON(manifest), null, "  "),
					);

					// Run package manager
					let installCommand;
					if (await exists(cwd.append("yarn.lock"))) {
						installCommand = "yarn";
					} else if (await exists(cwd.append("package-lock.json"))) {
						installCommand = "npm";
					}
					if (installCommand !== undefined) {
						const proc = spawn(
							installCommand,
							["install"],
							{
								cwd,
							},
						);
						await proc.waitSuccess();
					}
				},
			},
			{
				message: markup`Generating lint config and apply formatting`,
				test() {
					return flags.apply;
				},
				async callback() {
					const linter = new Linter(
						req,
						{
							apply: true,
						},
					);
					const {printer, savedCount} = await linter.runSingle();
					savedCheckFiles = savedCount;

					const globals: Array<string> = [];
					remainingCheckErrors = 0;
					for (const diag of printer.processor.getDiagnostics()) {
						if (diag.description.category === "lint/js/noUndeclaredVariables") {
							if (diag.meta && diag.meta.identifierName) {
								globals.push(diag.meta.identifierName);
							}
						} else {
							remainingCheckErrors++;
						}
					}
					if (globals.length > 0) {
						await updateConfig({
							lint: {
								globals,
							},
						});
					}
				},
			},
			{
				message: markup`Generating .editorconfig`,
				async callback() {
					const editorConfigPath = cwd.append(".editorconfig");
					if (await exists(editorConfigPath)) {
						reporter.warn(
							markup`<emphasis>${editorConfigPath}</emphasis> already exists`,
						);
						return;
					}

					await server.projectManager.assertProject(cwd);

					// Get unique handlers
					const uniqueHandlers: Map<string, ExtensionHandler> = new Map();
					for (const path of server.memoryFs.glob(cwd)) {
						const {handler} = getFileHandlerFromPath(path, undefined);
						if (handler !== undefined) {
							uniqueHandlers.set(handler.ext, handler);
						}
					}

					let editorConfigTabExtensions: Array<string> = [];
					for (const [ext, handler] of uniqueHandlers) {
						if (handler.hasTabs) {
							editorConfigTabExtensions.push(`*.${ext}`);
						}
					}

					let editorConfigTemplate = "";

					if (editorConfigTabExtensions.length > 0) {
						editorConfigTemplate = dedent`
							[{${editorConfigTabExtensions.sort().join(", ")}}]
							end_of_line = lf
							trim_trailing_whitespace = true
							insert_final_newline = true
							charset = utf-8
							indent_style = tab
							indent_size = 2
						`;
					}

					files.set(
						editorConfigPath,
						markup`Sets editor formatting and indentation options. Documentation: <hyperlink target="https://editorconfig.org/" />`,
					);
					await writeFile(editorConfigPath, editorConfigTemplate.trim() + "\n");
				},
			},
		]);

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

		await reporter.section(
			markup`Files created`,
			async () => {
				reporter.list(
					Array.from(
						files,
						([path, purpose]) =>
							markup`<emphasis>${path}</emphasis>: ${purpose}`
						,
					),
				);
			},
		);

		await reporter.section(
			markup`What next?`,
			() => {
				reporter.list(
					[
						markup`<emphasis>Setup an editor extension</emphasis>\nGet live errors as you type and format when you save. Learn more: <hyperlink target="https://romefrontend.dev/#editor-integration" />`,
						markup`<emphasis>Try a command</emphasis>\n<code>rome check</code> is used to validate your code, verify formatting, and check for lint errors. Run <code>rome --help</code> for a full list of commands and flags.`,
						markup`<emphasis>Read documentation</emphasis>\nOur website serves as a comprehensive source of guides and documentation <hyperlink target="https://romefrontend.dev/" />`,
						markup`<emphasis>Get involved in the community</emphasis>\nAsk questions, get support, or contribute by participating on GitHub (<hyperlink target="https://github.com/romefrontend/rome"/>) or our community Discord (<hyperlink target="https://discord.gg/rome" />)`,
					],
					{
						ordered: true,
						pad: true,
					},
				);
				reporter.br();
			},
		);

		return true;
	},
});
