/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

require("../_setup.cjs");

const {toCamelCase, write, readFile, camelCaseToKebabCase} = require(
	"../_utils.cjs",
);
const {lintRulesFolder, descriptionsFolder, lintRulesDocFolder} = require(
	"../_constants.cjs",
);
const path = require("path");

const ruleName = process.argv[2];
const category = process.argv[3];
if (ruleName === undefined || category === undefined) {
	console.error("node scripts/lint/add.cjs [ruleName] [category]");
	process.exit(1);
}

const camelCasedName = toCamelCase(ruleName);
const groupCamelCasedName = toCamelCase(`${category}-${ruleName}`);
const spacedName = camelCasedName.replace(/([A-Z+])/g, " $1").trim().toLowerCase();
const descriptionKey = `${category}_${spacedName}`.toUpperCase().replace(
	/[\s\-]/g,
	"_",
);
const categoryName = `lint/${category}/${camelCasedName}`;

const ruleLoc = path.join(lintRulesFolder, category, `${camelCasedName}.ts`);
const testLoc = path.join(
	lintRulesFolder,
	category,
	`${camelCasedName}.test.ts`,
);
const docLoc = path.join(
	lintRulesDocFolder,
	category,
	`${camelCaseToKebabCase(camelCasedName)}.md`,
);

write(
	ruleLoc,
	`import {Path, TransformExitResult} from "@romejs/compiler";
import {descriptions} from "@romejs/diagnostics";

export default {
 name: "${groupCamelCasedName}",
 enter(path: Path): TransformExitResult {
	 const {node} = path;

	 if (false) {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.${descriptionKey},
			);
	 }

	 return node;
 },
};
`,
);

write(
	testLoc,
	`import {test} from "rome";
import {testLint} from "../../utils/testing";

test(
	"${category} ${spacedName}",
	async (t) => {
		await testLint(
			t,
			{
				invalid: [""],
				valid: [""],
			},
			{category: "${categoryName}"},
		);
	},
);
`,
);

write(
	docLoc,
	`---
title: Rome
layout: layouts/base.njk
showHero: false
description: DESCRIPTION HERE
---

# ${camelCasedName}

not documented yet
`,
);

// Add description
const descriptionsFile = path.join(descriptionsFolder, "lint.ts");
let descriptions = readFile(descriptionsFile, "utf8");
descriptions = descriptions.replace(
	"createDiagnosticsCategory({",
	`createDiagnosticsCategory({\n	${descriptionKey}: {
		category: "${categoryName}",
		message: "INSERT MESSAGE HERE",
	},`,
);
write(descriptionsFile, descriptions);

require("./update.cjs");
