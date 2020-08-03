import {INTERNAL, modifyGeneratedFile} from "../_utils";
import {lstat, readDirectory} from "@internal/fs";
import {AbsoluteFilePath} from "@internal/path";

const astFolder = INTERNAL.append("ast");

type ASTDefinition = {
	category: string;
	language: string;
	nodeType: string;
};

export async function main() {
	let defs: Array<ASTDefinition> = [];

	for (const languageFolder of await readDirectory(astFolder)) {
		const language = languageFolder.getBasename();
		if ((await lstat(languageFolder)).isFile()) {
			continue;
		}

		for (const categoryFolder of await readDirectory(languageFolder)) {
			const category = categoryFolder.getBasename();
			if ((await lstat(categoryFolder)).isFile()) {
				continue;
			}

			for (const path of await readDirectory(categoryFolder)) {
				defs.push({
					category,
					language,
					nodeType: path.getExtensionlessBasename(),
				});
			}
		}
	}

	// Sort nodes by type
	defs = defs.sort((a, b) => {
		return a.nodeType.localeCompare(b.nodeType);
	});

	async function readIndexFile(
		path: AbsoluteFilePath,
		handlers: Array<{
			iterator: (def: ASTDefinition) => string;
			wrapCallback?: (buff: string) => string;
		}>,
	) {
		await modifyGeneratedFile(
			{
				path,
				scriptName: "generated-files/ast",
			},
			async () => {
				const lines = [];

				for (const {iterator, wrapCallback} of handlers) {
					let buff = "";

					for (const def of defs) {
						const defBuff = iterator(def);
						if (defBuff) {
							buff += defBuff;
						}
					}

					if (wrapCallback) {
						buff = wrapCallback(buff);
					}

					if (buff !== "") {
						lines.push(buff);
						lines.push("");
						lines.push("");
					}
				}

				return {lines};
			},
		);
	}

	// Add to ast index
	await readIndexFile(
		astFolder.append("index.ts"),
		[
			{
				iterator({language, category, nodeType}) {
					return `export * from "./${language}/${category}/${nodeType}";\n`;
				},
			},
			{
				iterator(def) {
					return `\n	| n.${def.nodeType}`;
				},
				wrapCallback(buff) {
					return `export type AnyNode = ${buff};`;
				},
			},
		],
	);

	// Add to builders
	await readIndexFile(
		INTERNAL.append("formatter", "builders", "index.ts"),
		[
			{
				iterator({language, category, nodeType}) {
					return `import ${nodeType} from "./${language}/${category}/${nodeType}";\nbuilders.set("${nodeType}", ${nodeType});\n`;
				},
			},
		],
	);

	// Add to analysis
	await readIndexFile(
		INTERNAL.append("js-analysis", "evaluators", "index.ts"),
		[
			{
				iterator({language, category, nodeType}) {
					if (language === "js") {
						return `import ${nodeType} from "./${category}/${nodeType}";\nevaluators.set("${nodeType}", ${nodeType});\n`;
					} else {
						return "";
					}
				},
			},
		],
	);
}
