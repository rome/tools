import {INTERNAL, ROOT, modifyGeneratedFile} from "../_utils";
import {lstat, readDirectory, readFileText} from "@internal/fs";

const virtualModules = ROOT.append("internal", "virtual-packages");
const virtualIndex = INTERNAL.append("core", "common", "virtual-modules.ts");

export async function main() {
	await modifyGeneratedFile(
		{
			path: virtualIndex,
			scriptName: "generated-files/virtual-modules",
		},
		async () => {
			let lines = [];
			let hash = "";

			for (const packagePath of await readDirectory(virtualModules)) {
				const packageName = packagePath.getBasename();

				const files = [];
				for (const path of await readDirectory(packagePath)) {
					const content = await readFileText(path);
					hash += content;
					files.push([
						path.getBasename(),
						{
							mtime: (await lstat(path)).mtimeMs,
							content,
						},
					]);
				}

				lines.push(
					`modules.set("${packageName}", new Map(${JSON.stringify(
						files,
						null,
						"\t",
					)}));`,
				);
			}

			return {lines, hash};
		},
	);
}
