/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

require("../_setup.cjs");

const {readGeneratedFile, write} = require("../_utils.cjs");
const {lintRulesFolder, categoriesFile} = require("../_constants.cjs");
const path = require("path");
const fs = require("fs");

let defs = [];

for (const category of fs.readdirSync(lintRulesFolder)) {
	const loc = path.join(lintRulesFolder, category);

	if (fs.statSync(loc).isFile()) {
		continue;
	}

	for (const filename of fs.readdirSync(loc)) {
		if (filename.endsWith(".ts") && !filename.endsWith(".test.ts")) {
			const basename = path.basename(filename, path.extname(filename));
			defs.push({
				basename,
				category,
				ruleName: `${category}/${basename}`,
			});
		}
	}
}

defs = defs.sort((a, b) => {
	return a.ruleName.localeCompare(b.ruleName);
});

// Generate rules index
const indexLoc = path.join(lintRulesFolder, "index.ts");
let index = readGeneratedFile(indexLoc);

for (const {basename, ruleName} of defs) {
	index += `import ${basename} from "./${ruleName}";\n`;
}

index += "\n";
index += "export const lintTransforms = [\n";
for (const {basename} of defs) {
	index += `	${basename},\n`;
}
index += "];\n";

write(indexLoc, index);

// Generate categories
let categories = readGeneratedFile(categoriesFile);
categories += "type LintDiagnosticCategory =";
for (const {ruleName} of defs) {
	categories += `\n	| "lint/${ruleName}"`;
}
categories += ";\n";
write(categoriesFile, categories);
