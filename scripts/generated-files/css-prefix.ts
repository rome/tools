import {INTERNAL, modifyGeneratedFile} from "../_utils";

const cssPrefixFolder = INTERNAL.append("compiler", "transforms", "compile", "css-handler", "prefix");

type VisitorDefinition = {
	basename: string;
};

export async function getPrefixVisitorDefs(): Promise<VisitorDefinition[]> {
	let defs: VisitorDefinition[] = [];

	for (const path of await cssPrefixFolder.append("prefixes").readDirectory()) {
		if (
			path.getBasename()[0] !== "." &&
			path.hasEndExtension("ts")
		) {
			defs.push({
				basename: path.getExtensionlessBasename(),
			});
		}
	}

	defs = defs.sort((a, b) => {
		return a.basename.localeCompare(b.basename);
	});

	return defs;
}
export async function main() {
	const defs = await getPrefixVisitorDefs();

	// Generate visitor list
	await modifyGeneratedFile(
		{
			path: cssPrefixFolder.append("index.ts"),
			scriptName: "generated-files/lint-rules",
		},
		async () => {
			let lines = [];
			for (const {basename} of defs) {
				lines.push(`import ${basename} from "./prefixes/${basename}";`);
			}
			lines.push("");
			lines.push("const prefixVisitors: PrefixVisitor<UnknownObject>[] = [",);
			for (const {basename} of defs) {
				lines.push(`\t...${basename},`);
			}
			lines.push("];");

			return {lines};
		},
	);
}
