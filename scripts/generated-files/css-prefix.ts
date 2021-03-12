import {INTERNAL, modifyGeneratedFile} from "../_utils";

const cssPrefixFolder = INTERNAL.append("compiler", "transforms", "compile", "css-handler", "prefix");

type VisitorDefinition = {
	basename: string;
	array: boolean;
};

export async function getPrefixVisitorDefs(): Promise<VisitorDefinition[]> {
	let defs: VisitorDefinition[] = [];

	for (const path of await cssPrefixFolder.append("prefixes").readDirectory()) {
		if (
			path.getBasename()[0] !== "." &&
			path.hasEndExtension("ts")
		) {
			const basename = path.getExtensionlessBasename();

			defs.push({
				basename,
				array: false,
			});
		}
	}

	defs = defs.sort((a, b) => {
		return a.basename.localeCompare(b.basename);
	});

	return defs;
}
// TODO test this
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
			for (const {basename, array} of defs) {
				lines.push(array ? `\t...${basename},` : `\t${basename},`);
			}
			lines.push("];");

			return {lines};
		},
	);
}
