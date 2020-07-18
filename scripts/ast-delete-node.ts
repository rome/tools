import {PACKAGES, reporter} from "./_utils";
import {main as generateAST} from "./generated-files/ast";
import {removeFile} from "@romefrontend/fs";
import {createUnknownFilePath} from "@romefrontend/path";

export async function main([filename]: Array<string>) {
	if (filename === undefined) {
		reporter.error(
			"./rome run ast-delete-node [language]/[category]/[nodeType]",
		);
		return 1;
	}

	const segments = createUnknownFilePath(filename).getSegments();
	if (segments.length !== 3) {
		reporter.error("Expected three segments in filename argument");
		return 1;
	}

	const [, category, nodeName] = segments;

	// Remove files
	await removeFile(
		PACKAGES.appendList("formatter", "builders", `${filename}.ts`),
	);
	await removeFile(
		PACKAGES.appendList(
			"js-analysis",
			"evaluators",
			`${category}/${nodeName}.ts`,
		),
	);
	await removeFile(PACKAGES.appendList("ast", `${filename}.ts`));

	// Regenerate indexes
	await generateAST();
	return 0;
}
