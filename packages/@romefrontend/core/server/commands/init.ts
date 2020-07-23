import {createServerCommand} from "@romefrontend/core/server/commands";
import {commandCategories} from "@romefrontend/core/common/commands";
import {markup} from "@romefrontend/cli-layout";
import {ServerRequest} from "@romefrontend/core";
import {getFileHandlerFromExtension} from "@romefrontend/core/common/file-handlers";
import {ExtensionHandler} from "@romefrontend/core/common/file-handlers/types";
import {dedent} from "@romefrontend/string-utils";
import {exists, writeFile} from "@romefrontend/fs";

export default createServerCommand({
	category: commandCategories.PROCESS_MANAGEMENT,
	description: markup`initialise the project`,
	usage: "",
	examples: [],
	defineFlags() {
		return {};
	},
	async callback({server, client, reporter}: ServerRequest) {
		const projectPath = client.flags.cwd;
		const editorConfigPath = projectPath.append(".editorconfig");

		if (await exists(editorConfigPath)) {
			reporter.info(markup`.editorconfig file already exists`);
		} else {
			await server.projectManager.assertProject(projectPath);

			const paths = server.memoryFs.glob(projectPath);

			// tracking unique extensions
			const uniqueExtensions = new Set(
				Array.from(paths, (path) => path.getExtensions()),
			);
			// track ext => handler
			const uniqueHandlers = new Map<string, ExtensionHandler>();
			for (const ext of uniqueExtensions) {
				// there might be multiple projects, but we want to keep everything unique
				for (const [, project] of server.projectManager.projects) {
					const handler = getFileHandlerFromExtension(ext, project.config);
					if (handler && !uniqueHandlers.has(ext)) {
						uniqueHandlers.set(ext, handler);
					}
				}
			}

			let editorConfigTabExtensions: Array<string> = [];
			let editorConfigSpaceExtensions: Array<string> = [];
			for (const [ext, handler] of uniqueHandlers) {
				if (handler.hasTabs) {
					editorConfigTabExtensions.push(`*${ext}`);
				} else {
					editorConfigSpaceExtensions.push(`*${ext}`);
				}
			}

			let editorConfigTemplate = dedent`
				[*]
				end_of_line = lf
				trim_trailing_whitespace = true
				insert_final_newline = true
				charset = utf-8
				indent_style = space
				indent_size = 2
        `;

			if (editorConfigTabExtensions.length > 0) {
				editorConfigTemplate += "\n\n";

				editorConfigTemplate += dedent`
                [{${editorConfigTabExtensions.join(", ")}}]
				end_of_line = lf
				trim_trailing_whitespace = true
				insert_final_newline = true
				charset = utf-8
				indent_style = tab
				indent_size = 2
            `;
			}

			if (editorConfigSpaceExtensions.length > 0) {
				editorConfigTemplate += "\n\n";

				editorConfigTemplate += dedent`
                [{${editorConfigSpaceExtensions.join(", ")}}]
				end_of_line = lf
				trim_trailing_whitespace = true
				insert_final_newline = true
				charset = utf-8
				indent_style = space
				indent_size = 2
            `;
			}

			await writeFile(editorConfigPath, editorConfigTemplate);

			reporter.success(markup`.editorconfig successfully created`);
		}

		return true;
	},
});
