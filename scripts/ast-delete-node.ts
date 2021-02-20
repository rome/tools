import {INTERNAL, reporter} from "./_utils";
import {main as generateAST} from "./generated-files/ast";
import {createAnyPath} from "@internal/path";
import {markup} from "@internal/markup";

export async function main([filename]: string[]) {
	if (filename === undefined) {
		reporter.error(
			markup`./rome run ast-delete-node [language]/[category]/[nodeType]`,
		);
		return 1;
	}

	const segments = createAnyPath(filename).getSegments();
	if (segments.length !== 3) {
		reporter.error(markup`Expected three segments in filename argument`);
		return 1;
	}

	const [, category, nodeName] = segments;

	// Remove files
	await INTERNAL.append("formatter", "builders", `${filename}.ts`).removeFile();
	await	INTERNAL.append("js-analysis", "evaluators", `${category}/${nodeName}.ts`).removeFile();
	await INTERNAL.append("ast", `${filename}.ts`).removeFile();

	// Regenerate indexes
	await generateAST();
	return 0;
}
