import {PACKAGES, ROOT, modifyGeneratedFile} from "../_utils";
import {lstat, readDirectory, readFileText} from "@romefrontend/fs";

const runtimeModules = ROOT.append("packages", "@romefrontend-runtime");
const runtimeIndex = PACKAGES.append("core", "common", "virtual-modules.ts");

export async function main() {
	await modifyGeneratedFile(
		runtimeIndex,
		async () => {
			let lines = [];
			let hash = "";

			for (const packagePath of await readDirectory(runtimeModules)) {
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
