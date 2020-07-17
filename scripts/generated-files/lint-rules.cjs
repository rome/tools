/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

require("../_setup.cjs");

const {readGeneratedFile, write, readFile} = require("../_utils.cjs");
const {lintRulesFolder, categoriesFile, lintRulesDocFolder} = require(
	"../_constants.cjs",
);
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
			const ruleName = `${category}/${basename}`;
			const ruleDocFile = path.join(lintRulesDocFolder, `${ruleName}.md`);

			defs.push({
				docs: ruleDocFile,
				hasRJSON: fs.existsSync(path.join(loc, `${basename}.test.rjson`)),
				basename,
				category,
				ruleName,
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

// Generate tests
const testsLoc = path.join(lintRulesFolder, "tests.ts");
let tests = readGeneratedFile(testsLoc);
for (const {basename, ruleName, hasRJSON} of defs) {
	if (hasRJSON) {
		tests += "// @ts-ignore\n";
		tests += `import ${basename} from "./${ruleName}.test.rjson";\n`;
	}
}
tests += "\n";
tests += "export const tests: Tests = {\n";
for (const {basename, ruleName, hasRJSON} of defs) {
	if (hasRJSON) {
		tests += `	"${ruleName}": ${basename},\n`;
	}
}
tests += "};\n";
write(testsLoc, tests);

// Generate markdown file on website folder
function getDocRuleDescription(content) {
	const description = content.match(/description:(.*)/);
	if (description) {
		return description[1];
	}

	return null;
}

const lintRulesDocFile = path.join(lintRulesDocFolder, "index.md");

let docTemplate = readGeneratedFile(lintRulesDocFile, false);

// Used for doc headings
const categoryAlias = {
	js: "JavaScript",
	"jsx-a11y": "JSX Accessibility",
	react: "React",
	ts: "TypeScript",
};

let docTemplateTable = "";

for (const {basename, ruleName, category, docs} of defs) {
	if (categoryAlias[category]) {
		if (docTemplateTable) {
			docTemplate += docTemplateTable;
			docTemplate += "\n";
		}

		docTemplate += `## ${categoryAlias[category]}\n\n`;
	}

	// Remove this, so each doc headings will be added once
	categoryAlias[category] = null;

	const content = readFile(docs).toString();
	const description = getDocRuleDescription(content);
	if (!description) {
		console.log(`/docs/lint/rules/${ruleName}.md is missing a description\n`);
	}

	docTemplateTable += `- [${basename}](/docs/lint/rules/${ruleName}):  ${description ||
	""}\n`;
}

docTemplate += docTemplateTable;
docTemplate += "\n";

write(lintRulesDocFile, docTemplate);
