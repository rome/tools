import {INTERNAL, reporter} from "./_utils";
import {main as generateAST} from "./generated-files/ast";
import {removeFile} from "@internal/fs";
import {createUnknownPath} from "@internal/path";
import {markup} from "@internal/markup";

export async function main([filename]: Array<string>) {
	if (filename === undefined) {
		reporter.error(
			markup`./rome run ast-delete-node [language]/[category]/[nodeType]`,
		);
		return 1;
	}

	const segments = createUnknownPath(filename).getSegments();
	if (segments.length !== 3) {
		reporter.error(markup`Expected three segments in filename argument`);
		return 1;
	}

	const [, category, nodeName] = segments;

	// Remove files
	await removeFile(INTERNAL.append("formatter", "builders", `${filename}.ts`));
	await removeFile(
		INTERNAL.append("js-analysis", "evaluators", `${category}/${nodeName}.ts`),
	);
	await removeFile(INTERNAL.append("ast", `${filename}.ts`));

	// Regenerate indexes
	await generateAST();
	return 0;
}
