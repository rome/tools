/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

require("../_setup.cjs");

const {readGeneratedFile, write, camelCaseToKebabCase} = require(
	"../_utils.cjs",
);
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

// Generate markdown file on website folder
function getDocRuleDescription(content) {
	const description = content.match(/description:(.*)/);
	if (description) {
		return description[1];
	}

	return null;
}

const lintRulesDocFile = path.join(lintRulesDocFolder, "index.md");

let docTemplate = `---
title: Rome - A JavaScript toolchain
layout: layouts/base.njk
showHero: false
---

# Rules\n
`;

//Used for doc headings
const categoryAlias = {
	js: "Javascript",
	"jsx-a11y": "JSX a11y",
	react: "React",
	ts: "TypeScript",
};

let docTemplateTable = null;
let countExistingDocFiles = 0;
let countMissingDocFiles = 0;

for (const {basename, category} of defs) {
	if (categoryAlias[category]) {
		if (docTemplateTable) {
			docTemplate += docTemplateTable;
			docTemplate += `\n`;
		}

		docTemplate += `## ${categoryAlias[category]}\n\n`;

		docTemplateTable = `| Rule | Description |\n| ------------- | ------------- |\n`;
	}
	//Remove this, so each doc headings will be added once
	categoryAlias[category] = null;

	const ruleNameKebabCase = camelCaseToKebabCase(basename);
	const ruleDocFile = path.join(lintRulesDocFolder, `${ruleNameKebabCase}.md`);

	if (fs.existsSync(ruleDocFile)) {
		const content = fs.readFileSync(ruleDocFile).toString();
		const description = getDocRuleDescription(content);
		if (!description) {
			console.log(
				`/lint/rules/${ruleNameKebabCase}.md is missing a description\n`,
			);
		}

		countExistingDocFiles += 1;
		docTemplateTable += `| [${ruleNameKebabCase}](/lint/rules/${ruleNameKebabCase}) | ${description ||
		""} |\n`;

		continue;
	}

	docTemplateTable += `| ${ruleNameKebabCase} |  |\n`;
	countMissingDocFiles += 1;
}

docTemplate += docTemplateTable;
docTemplate += `\n`;

fs.writeFile(
	lintRulesDocFile,
	docTemplate,
	function(err) {
		if (err) {
			return console.log(err);
		}

		console.log(`Wrote: ${lintRulesDocFile}`);
		console.log(
			`\nRules doc progress: ${countExistingDocFiles}/${countExistingDocFiles +
			countMissingDocFiles}`,
		);
	},
);
