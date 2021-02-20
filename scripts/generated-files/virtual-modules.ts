import {INTERNAL, ROOT, modifyGeneratedFile, valueToCode} from "../_utils";

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

			for (const packagePath of await virtualModules.readDirectory()) {
				const packageName = packagePath.getBasename();

				const files = [];
				for (const path of await packagePath.readDirectory()) {
					const content = await path.readFileText();
					hash += content;
					files.push([
						path.getBasename(),
						{
							mtime: Number((await path.lstat()).mtimeMs),
							content,
						},
					]);
				}

				lines.push(
					`modules.set("${packageName}", new Map(${valueToCode(files)}));`,
				);
			}

			return {lines, hash};
		},
	);
}
