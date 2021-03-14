import {createServerCommand} from "@internal/core/server/commands";
import {commandCategories} from "@internal/core/common/commands";
import {StaticMarkup, markup} from "@internal/markup";
import {ServerRequest, VERSION} from "@internal/core";
import {ExtensionHandler} from "@internal/core/common/file-handlers/types";
import {dedent} from "@internal/string-utils";
import {ConfigCommentMap, JSONObject, rjson} from "@internal/codec-config";
import {getFileHandlerFromPath} from "@internal/core/common/file-handlers";
import {IndentStyle, PROJECT_CONFIG_DIRECTORY} from "@internal/project";
import {AbsoluteFilePathMap} from "@internal/path";
import {
	Diagnostic,
	catchDiagnosticsSync,
	createSingleDiagnosticsError,
	descriptions,
} from "@internal/diagnostics";
import {convertManifestToJSON} from "@internal/codec-js-manifest";
import {parseSemverRange} from "@internal/codec-semver";
import {spawn} from "@internal/child-process";
import {ConfigHandler} from "@internal/codec-config/types";
import updateConfig from "@internal/core/server/utils/updateConfig";
import retrieveConfigHandler from "@internal/codec-config/retrieveConfigHandler";

type ConfigType = "rjson" | "toml" | "json";

type Flags =
	| {
			configType: ConfigType;
			indentStyle: IndentStyle;
			indentSize: number;
			checkProject?: boolean;
		}
	| {
			configType?: ConfigType;
			indentStyle?: IndentStyle;
			indentSize?: number;
			checkProject: boolean;
		};

export default createServerCommand<Flags>({
	category: commandCategories.PROCESS_MANAGEMENT,
	description: markup``,
	usage: "",
	examples: [],
	hidden: true,
	defineFlags(c) {
		return {
			checkProject: c.get("checkProject").asBoolean(),
			configType: c.get(
				"configType",
				{
					description: markup``,
				},
			).asStringSet<ConfigType>(["rjson", "json"], "rjson"),
			indentStyle: c.get(
				"indentStyle",
				{
					description: markup``,
				},
			).asStringSet<IndentStyle>(["tab", "space"], "tab"),
			indentSize: c.get(
				"indentSize",
				{
					description: markup``,
				},
			).asNumber(1),
		};
	},
	async callback(req: ServerRequest, flags: Flags) {
		const {server, client, reporter, query} = req;
		const {args} = query;
		const {cwd} = client.flags;
		const {configType, indentSize, indentStyle, checkProject} = flags;

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

		// This command is split in two:
		// - first part checks if a project exists or not
		// - second part creates a basic configuration file and and the .editorconfig
		// part of the command where we check if a configuration already exists
		if (checkProject) {
			req.expectArgumentLength(1);
			// Check for sensitive directory
			if (server.projectManager.isBannedProjectPath(cwd)) {
				const diagnostic: Diagnostic = {
					description: descriptions.PROJECT_MANAGER.INITING_SENSITIVE(cwd),
					location: req.getDiagnosticLocationFromFlags("cwd"),
				};
				throw createSingleDiagnosticsError(diagnostic);
			}

			// Don't allow if we're already in a project
			const {value} = catchDiagnosticsSync(async () => {
				const existingProject = await server.projectManager.findProject(cwd);
				if (existingProject !== undefined) {
					reporter.error(
						markup`Project already exists. Defined at <emphasis>${existingProject.meta.configPath}</emphasis>`,
					);
					reporter.info(
						markup`Use <code>rome config</code> to update an existing config`,
					);
					return true;
				}
				return false;
			});

			const existingProject = await server.projectManager.findProject(cwd);
			if (existingProject !== undefined) {
				reporter.error(
					markup`Project already exists. Defined at <emphasis>${existingProject.meta.configPath}</emphasis>`,
				);
				reporter.info(
					markup`Use <code>rome config</code> to update an existing config`,
				);
				return true;
			}
			return value;
		} else if (configType && indentSize && indentStyle) {
			req.expectArgumentLength(3);
			let configHandler: ConfigHandler = retrieveConfigHandler(configType);

			if (!configType) {
				client.reporter.warn(
					markup`No extension chosen; Rome will now use RJSON extension for your project configuration as fallback.`,
				);
				configHandler = rjson;
			}

			reporter.heading(markup`Welcome to Rome! Let's get you started...`);

			const configPath = cwd.append(
				PROJECT_CONFIG_DIRECTORY,
				`rome.${configType}`,
			);

			// Track some information about our project generation
			const files: AbsoluteFilePathMap<StaticMarkup> = new AbsoluteFilePathMap();
			files.set(
				configPath,
				markup`Your project configuration. Documentation: <hyperlink target="https://rome.tools/#project-configuration" />`,
			);

			// We are only using JSONObject here because we don't have an accurate type definition for what
			// the config actually looks like on disk
			let config: JSONObject = {
				root: true,
				name: cwd.getBasename(),
				format: {
					enabled: true,
					indentSize,
					indentStyle,
				},
			};

			// Comments to include in the created project config
			const comments: ConfigCommentMap = new Map();
			comments.set(
				"",
				{
					inner: [],
					outer: [
						{
							type: "LineComment",
							value: " For configuration documentation see https://rome.tools/#project-configuration",
						},
					],
				},
			);

			// Create initial project config
			await configPath.getParent().createDirectory();
			await updateConfig({
				comments,
				configHandler,
				configPath,
				config,
			});

			//
			const manifestPath = cwd.append("package.json");
			const manifestDefinition = server.memoryFs.getManifestDefinition(cwd);
			if (!manifestDefinition) {
				reporter.error(markup`Couldn't find any manifest at path ${cwd}`);

				return;
			}

			// Generate files
			await reporter.steps([
				{
					message: markup`Installing Rome as a dependency`,
					test() {
						return (
							manifestDefinition !== undefined &&
							!manifestDefinition.manifest.dependencies.has("rome") &&
							!manifestDefinition.manifest.devDependencies.has("rome")
						);
					},
					async callback() {
						if (manifestDefinition === undefined) {
							// Should not be because of test()
							return;
						}

						// Modify package.json
						manifestDefinition.manifest.devDependencies.set(
							"rome",
							{
								type: "semver",
								range: parseSemverRange({input: `^${VERSION}`}),
							},
						);
						await manifestPath.writeFile(
							JSON.stringify(
								convertManifestToJSON(manifestDefinition.manifest),
								null,
								"  ",
							),
						);

						// Run package manager
						let installCommand;
						if (await cwd.append("yarn.lock").exists()) {
							installCommand = "yarn";
						} else if (await cwd.append("package-lock.json").exists()) {
							installCommand = "npm";
						} else if (await cwd.append("pnpm-lock.yml").exists()) {
							installCommand = "pnpm";
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
					message: markup`Generating .editorconfig`,
					async callback() {
						const editorConfigPath = cwd.append(".editorconfig");
						if (await editorConfigPath.exists()) {
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

						let editorConfigTabExtensions: string[] = [];
						for (const [ext, handler] of uniqueHandlers) {
							if (handler.hasTabs) {
								editorConfigTabExtensions.push(`*.${ext}`);
							}
						}

						let editorConfigTemplate = "";

						if (editorConfigTabExtensions.length > 0) {
							editorConfigTemplate = dedent`
								[{${editorConfigTabExtensions.sort().join(",")}}]
								end_of_line = lf
								trim_trailing_whitespace = true
								insert_final_newline = true
								charset = utf-8
								indent_style = ${indentStyle}
								indent_size = ${String(indentSize)}
							`;
						}

						files.set(
							editorConfigPath,
							markup`Sets editor formatting and indentation options. Documentation: <hyperlink target="https://editorconfig.org/" />`,
						);
						await editorConfigPath.writeFile(editorConfigTemplate.trim() + "\n");
					},
				},
			]);

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
							markup`<emphasis>Setup an editor extension</emphasis>\nGet live errors as you type and format when you save. Learn more: <hyperlink target="https://rome.tools/#editor-integration" />`,
							markup`<emphasis>Try a command</emphasis>\n<code>rome check</code> is used to validate your code, verify formatting, and check for lint errors. Run <code>rome --help</code> for a full list of commands and flags.`,
							markup`<emphasis>Read documentation</emphasis>\nOur website serves as a comprehensive source of guides and documentation <hyperlink target="https://rome.tools/" />`,
							markup`<emphasis>Get involved in the community</emphasis>\nAsk questions, get support, or contribute by participating on GitHub (<hyperlink target="https://github.com/rome/tools"/>) or our community Discord (<hyperlink target="https://discord.gg/rome" />)`,
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
		}
		return true;
	},
});
